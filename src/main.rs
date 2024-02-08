mod db;
mod get;
mod post;

use std::{net::{IpAddr, Ipv4Addr}, str::FromStr};

use db::*;
use get::*;
use post::*;

use rocket::{State, serde::{Serialize, Deserialize, json::Json}, response::status};
use deadpool_postgres::Pool;

#[macro_use] extern crate rocket;

#[rocket::launch]
async fn launch() -> _ {
    dotenvy::dotenv().ok();

    let pool = init_pool().await;

    rocket::build()
    .manage(pool)
    .configure(rocket::Config::figment().merge(("port", 8000)))
    .configure(rocket::Config::figment().merge(("workers", 3)))
    .configure(rocket::Config::figment().merge(("address", IpAddr::V4(Ipv4Addr::from_str("0.0.0.0").unwrap()))))
    .mount("/", routes![transacoes, extrato])
}