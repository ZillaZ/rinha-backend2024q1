use chrono::DateTime;
use tokio_postgres::{tls::NoTls, *};
use std::{collections::HashMap, env};
use deadpool_postgres::{Config, Runtime};

use crate::*;

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct Answer {
    pub limite: i32,
    pub saldo: i32
}    

#[derive(Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct Transacao {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String
}    

#[derive(Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct ITransacao {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
    pub realizada_em: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct Wrapper {
    pub client: Answer,
    pub transacoes: Option<Vec<ITransacao>>
}

pub async fn init_pool() -> Pool {
    let vars = env::vars().collect::<HashMap<String, String>>();
    
    let user = vars.get("POSTGRES_USER").unwrap();
    let pass = vars.get("POSTGRES_PASSWORD").unwrap();
    let db = vars.get("POSTGRES_DB").unwrap();
    let host = vars.get("POSTGRES_HOST").unwrap();
    let max = vars.get("POSTGRES_POOLSIZE").unwrap().parse::<usize>().unwrap();

    let mut cfg = Config::new();
    cfg.host = Some(host.clone());
    cfg.dbname = Some(db.clone());
    cfg.user = Some(user.clone());
    cfg.password = Some(pass.clone());
    cfg.pool = Some(deadpool_postgres::PoolConfig::new(max));

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    return pool
}

pub async fn update_client(client: &Client, input: &Json<Transacao>, id: i32) -> Answer {
    let query = client.prepare("select * from update_client($1, $2, $3, $4, $5);").await.unwrap();
    let now = chrono::Utc::now();
    let val : i32 = match input.tipo.as_str() {
        "c" => input.valor * -1,
        _ => input.valor
    };
    let row = client.query_one(&query, &[&id, &val, &input.tipo, &input.descricao, &now]).await.unwrap();

    Answer {
        limite: row.get(0),
        saldo: row.get(1)
    }
}

pub async fn get_transactions(client: &Client, id: i32) -> Wrapper {
    let query  = client.prepare("SELECT * FROM  get_client_and_transactions($1);").await.unwrap();
    let data = client.query(&query, &[&id]).await.unwrap();
    let rtn = Wrapper {
        client: Answer {
            limite: data[0].get::<usize, i32>(0),
            saldo: data[0].get::<usize, i32>(1)
        },
        transacoes: data.iter().map(|x| {
            if data[0].try_get::<usize, i32>(4).is_err() {
                return None
            }
            Some(ITransacao {
                tipo: x.get(2),
                descricao: x.get(3),
                valor: x.get(4),
                realizada_em: x.get::<usize, DateTime<chrono::Utc>>(5).to_rfc3339()
            })
        }).collect()
    };

    rtn
}
