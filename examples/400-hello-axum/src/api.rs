use std::{sync::{atomic::{AtomicUsize, Ordering}, RwLock, Arc}, collections::HashMap};

use axum::{Json, Extension, http::StatusCode, response::IntoResponse, extract::Path};
use serde::{Serialize, Deserialize};

pub type ID = usize;

// Rocket uses Serde for serializing/deserializing data.
#[derive(Serialize, Debug, Clone)]
pub struct Hero {
    pub id: ID,
    pub name: String,
    #[serde(rename(serialize = "canFly"))]
    pub can_fly: bool,
}

#[derive(Deserialize, Debug)]
pub struct NewHero {
    pub name: String,
    #[serde(rename(deserialize = "canFly"))]
    pub can_fly: bool,
}

pub struct HeroCount(AtomicUsize);

impl HeroCount {
    pub fn new() -> Self {
        Self(AtomicUsize::new(1))
    }
}

pub struct HeroesMap(RwLock<HashMap<ID, Hero>>);

impl HeroesMap {
    pub fn new() -> Self {
        Self { 0: RwLock::new(HashMap::new()) }
    }
}

pub async fn add_hero(Json(hero): Json<NewHero>, Extension(heroes): Extension<Arc<HeroesMap>>, Extension(hero_count): Extension<Arc<HeroCount>>) -> impl IntoResponse {
    let hid = hero_count.0.fetch_add(1, Ordering::Relaxed);

    // Build new hero
    let new_hero = Hero {
        id: hid,
        name: hero.name,
        can_fly: hero.can_fly,
    };

    // Insert new hero in hashmap
    let mut heroes = heroes.0.write().unwrap();
    heroes.insert(hid, new_hero.clone());

    (StatusCode::CREATED, Json(new_hero)).into_response()
}

pub async fn get_all(Extension(heroes): Extension<Arc<HeroesMap>>) -> Json<Vec<Hero>> {
    Json(heroes.0.read().unwrap().values().cloned().collect())
}

pub async fn get_hero(Path(id): Path<usize>, Extension(heroes): Extension<Arc<HeroesMap>>) -> impl IntoResponse {
    let heroes = heroes.0.read().unwrap();
    if let Some(hero) = heroes.get(&id) {
        Json(hero).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
