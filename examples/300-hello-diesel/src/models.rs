use diesel::prelude::*;
use crate::schema::{hero_types, heroes};

#[derive(Queryable, Identifiable)]
pub struct HeroType {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = hero_types)]
pub struct NewHeroType {
    pub name: String,
}

impl NewHeroType {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(HeroType))]
#[diesel(table_name = heroes)]
pub struct Hero {
    pub id: i32,
    pub name: String,
    pub hero_type_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = heroes)]
pub struct NewHero {
    pub name: String,
    pub hero_type_id: i32,
}

impl NewHero {
    pub fn new(name: String, hero_type_id: i32) -> Self {
        Self { name, hero_type_id }
    }
}