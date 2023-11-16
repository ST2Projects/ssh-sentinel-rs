mod auth;
mod model;

use std::fs::File;
use std::io::Write;
use rocket::{get};
use rocket::serde::json;
use rocket_okapi::{openapi, openapi_get_routes, openapi_get_spec};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use sqlx::{migrate, PgPool};
use crate::model::User;

#[openapi(operation_id = "hello")]
#[get("/hello")]
fn index(user: User) -> Option<String> {
    Some(format!("Hello, {}!", user.user_name?))
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {

    migrate!().run(&pool).await.expect("Failed to run migrations");
    write_open_api_spec();

    let rocket = rocket::build()
        .manage(pool)
        .mount("/", openapi_get_routes![index])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/openapi.json".to_owned(),
                ..Default::default()
            }),
        );

    Ok(rocket.into())
}

fn write_open_api_spec() {
    let spec = openapi_get_spec![index];
    let mut spec_file = File::create("./open-api.json").expect("Failed to create open-api.json");
    spec_file
        .write_all(
            json::to_pretty_string(&spec)
                .expect("Failed to unmarshall openapi")
                .as_bytes(),
        )
        .expect("Failed to write to openAPI");
}