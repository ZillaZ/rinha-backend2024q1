mod db;
mod get;
mod post;

use db::*;
use get::*;
use post::*;

use rocket::{serde::{Serialize, Deserialize, json::Json}, response::status};

#[macro_use] extern crate rocket;


#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();
    rocket::build()
    .configure(rocket::Config::figment().merge(("port", 9999)))
    .mount("/", routes![transacoes, extrato])
}