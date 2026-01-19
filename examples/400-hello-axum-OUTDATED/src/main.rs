use axum::{response::{Html, IntoResponse}, routing::{get, post}, Router, Json, extract::{Extension, Path, Query}, http::{StatusCode, self}};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Serialize;
use std::{net::SocketAddr, collections::HashMap, sync::Arc};

mod api;
use crate::api::{HeroesMap, HeroCount, add_hero, get_all, get_hero};

#[tokio::main]
async fn main() {
    let heroes = Arc::new(HeroesMap::new());
    let hero_count = Arc::new(HeroCount::new());

    // Defining routes (see https://docs.rs/axum/latest/axum/routing/struct.Router.html)
    let app = Router::new()
        .route("/", get(html_hello_world))
        .route("/json", get(json_hello_world))
        .route("/query_greeting", get(query_greeting))
        .route("/protected", get(protected))
        .route("/login", get(login))
        .route("/session", get(session))
        .route("/:name", get(greeting))
        .route("/heroes", post(add_hero))
        .route("/heroes", get(get_all))
        .route("/heroes/:id", get(get_hero))
        .layer(Extension(heroes))
        .layer(Extension(hero_count));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn html_hello_world() -> Html<&'static str> {
    // Return HTML response. See https://docs.rs/axum/latest/axum/response/index.html#structs
    // for details.
    Html(r"
        <h1>Hello Axum</h1>

        <ul>
            <li><a href='/'>Home</a></li>
            <li><a href='/json'>JSON response</a></li>
            <li><a href='/Rainer'>Hello Rainer</a></li>
            <li><a href='/query_greeting?name=Rainer'>Hello Rainer (Query)</a></li>
            <li><a href='/login'>Login (Cookie)</a></li>
            <li><a href='/session'>Session (Cookie)</a></li>
        </ul>
    ")
}

#[derive(Serialize)]
struct Greeting {
    greeting: String,
    name: String,
}

async fn json_hello_world() -> Json<Greeting> {
    Json(Greeting { greeting: "Hello ".to_string(), name: "World".to_string() })
}

async fn greeting(Path(name): Path<String>) -> String {
    // Uses the Path extractor to get path parameter
    // See https://docs.rs/axum/latest/axum/index.html#extractors for details
    format!("Hello {}", name)
}

async fn query_greeting(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    // Uses the Query extractor to get parameter
    // See https://docs.rs/axum/latest/axum/index.html#extractors for details
    if let Some(name) = params.get("name") {
        return format!("{} {}",
            params.get("salutation").unwrap_or(&"Hello".to_string()),
            name).into_response();
    }

    // Uses tuple and IntoResponse to return status code with message.
    // See https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html for details
    (StatusCode::BAD_REQUEST, "Query parameter 'name' missing").into_response()
}

mod api_keys;
use api_keys::ApiKey;

async fn protected(key: ApiKey) -> String {
    // Uses the FromRequest extractor trait to require an API key.
    // See https://docs.rs/axum/latest/axum/extract/trait.FromRequest.html for details
    format!("You are allowed to access this API because you presented key '{}'", key.0)

}

async fn login(cookies: CookieJar) -> (StatusCode, CookieJar) {
    // Uses the CookieJar extractor to set a cookie.
    // See https://docs.rs/axum-extra/0.3.7/axum_extra/extract/cookie/index.html for details
    (http::StatusCode::OK, cookies.add(Cookie::new("session", base64::encode("this_is_a_session_key"))))
}

async fn session(cookies: CookieJar) -> &'static str {
    match cookies.get("session") {
        Some(_) => "You got the cookie!",
        None => "Sorry, no cookie!",
    }
}
