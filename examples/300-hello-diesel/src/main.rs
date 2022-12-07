use std::env;

use diesel::prelude::*;
use diesel::{
    connection::SimpleConnection,
    r2d2::{ConnectionManager, Pool},
    result::{DatabaseErrorKind, Error},
    RunQueryDsl, SqliteConnection,
};

use dotenvy::dotenv;

mod schema;
use schema::*;
mod models;
use models::{Hero, HeroType, NewHero, NewHeroType};

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .unwrap();

    let mut conn = pool.get().unwrap();
    conn.batch_execute("PRAGMA foreign_keys = ON;").unwrap();

    let mut dal = HeroesRepository { conn: &mut conn };

    // Demonstrate delete
    dal.cleanup();

    // Demonstrate insert
    dal.insert_hero_types();

    // Demonstrate insert with error handling
    if let Err(Error::DatabaseError(dbe, _)) = dal.insert_heroes() {
        match dbe {
            DatabaseErrorKind::ForeignKeyViolation => println!("Foreign key violation"),
            // Add more cases here
            _ => println!("Other error"),
        }
    }

    // Demonstrate select
    let types = dal.get_hero_types().unwrap();
    println!("We have {} hero types", types.len());

    println!("{}", dal.get_hero_type_name(types.first().unwrap().id).unwrap());

    for h in dal.get_heroes_sorted("a").unwrap() {
        println!("{}", h.name);
    }

    let hero_id = dal.get_hero_id_by_name("Lux").unwrap();
    dal.update_hero_name(hero_id, "Batman").unwrap();
    for h in dal.get_heroes_sorted("").unwrap() {
        println!("{}", h.name);
    }
}

struct HeroesRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> HeroesRepository<'a> {
    fn cleanup(&mut self) {
        // Delete all rows from heroes and hero_types tables
        diesel::delete(heroes::table).execute(self.conn).unwrap();
        diesel::delete(hero_types::table).execute(self.conn).unwrap();
    }

    fn insert_hero_types(&mut self) {
        let hero_types = vec![
            NewHeroType::new("Warrior".to_string()),
            NewHeroType::new("Mage".to_string()),
            NewHeroType::new("Ranger".to_string()),
        ];
        diesel::insert_into(hero_types::table)
            .values(hero_types)
            .execute(self.conn)
            .unwrap();
    }

    fn insert_heroes(&mut self) -> Result<(), Error> {
        // Get all hero types
        let types = hero_types::table.load::<HeroType>(self.conn).unwrap();

        let new_heroes = vec![
            NewHero::new("Garen".to_string(), types.iter().find(|t| t.name == "Warrior").unwrap().id),
            NewHero::new("Lux".to_string(), types.iter().find(|t| t.name == "Mage").unwrap().id),
            NewHero::new("Vayne".to_string(), types.iter().find(|t| t.name == "Ranger").unwrap().id),
            //NewHero::new("ThisWillFail".to_string(), i32::MAX),
        ];
        // Note: Postgres would have a TransationBuilder.
        // For details see https://docs.diesel.rs/diesel/pg/struct.TransactionBuilder.html
        self.conn.batch_execute("BEGIN TRANSACTION;").unwrap();
        for ht in new_heroes {
            if let Err(err) = diesel::insert_into(heroes::table).values(ht).execute(self.conn) {
                self.conn.batch_execute("ROLLBACK;").unwrap();
                return Err(err);
            }
        }

        self.conn.batch_execute("COMMIT;").unwrap();
        Ok(())
    }

    fn get_hero_types(&mut self) -> Result<Vec<HeroType>, Error> {
        use crate::schema::hero_types::dsl::*;

        // Select everything from table
        hero_types.load(self.conn)
    }

    fn get_hero_type_name(&mut self, hero_type_id: i32) -> Result<String, Error> {
        use crate::schema::hero_types::dsl::*;

        // Get a single value from a single row
        hero_types.select(name).find(hero_type_id).first(self.conn)
    }

    fn get_heroes_sorted(&mut self, name_filter: &str) -> Result<Vec<Hero>, Error> {
        use crate::schema::heroes::dsl::*;

        let query = heroes.filter(name.like(format!("%{}%", name_filter))).order(name.desc());

        // Demonstrate how to print query for debugging
        // let sql = diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query).to_string();
        // println!("{sql}");

        query.load(self.conn)
    }

    fn get_hero_id_by_name(&mut self, hero_name: &str) -> Result<i32, Error> {
        use crate::schema::heroes::dsl::*;

        // Get a single value from a single row
        heroes.select(id).filter(name.eq(hero_name)).first(self.conn)
    }

    fn update_hero_name(&mut self, hero_id: i32, new_name: &str) -> Result<(), Error> {
        use crate::schema::heroes::dsl::*;

        diesel::update(heroes.find(hero_id))
            .set(name.eq(new_name))
            .execute(self.conn)?;

        Ok(())
    }
}
