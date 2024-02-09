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

pub async fn get_client(client: &Client, id: i32) -> Answer {
    let query = client.prepare("select * from clientes where id = ($1)").await.unwrap();
    let row = client.query_one(&query, &[&id]).await.unwrap();

    Answer {
        limite: row.get(1),
        saldo: row.get(2)
    }
}

pub async fn update_client(dbclient: &Client, id: i32, data: &Transacao, client: &mut Answer) -> bool {
    if data.tipo == "c" {
        client.saldo += data.valor;
    }else {
        client.saldo -= data.valor;
    }

    if client.saldo < client.limite * -1 {
        return false
    }
    let now = chrono::Utc::now();
    let statement = dbclient.prepare("call update_client($1, $2, $3, $4, $5, $6)").await.unwrap();
    dbclient.execute(&statement, &[&id, &client.saldo, &data.tipo, &data.descricao, &now, &data.valor]).await.unwrap();
    true
}

pub async fn get_transactions(client: &Client, id: i32) -> Vec<ITransacao> {
    let query  = client.prepare("SELECT * FROM transacoes WHERE id_cliente = ($1) ORDER BY realizada_em DESC LIMIT 10;").await.unwrap();
    let data = client.query(&query, &[&id]).await.unwrap();
    if data.len() < 1 {
        return Vec::with_capacity(0);
    }
    let rtn : Vec<ITransacao> = data.iter().map(|x| {
        let aux : DateTime<chrono::Utc> = x.get(3);
        ITransacao {
            tipo: x.get(1),
            descricao: x.get(2),
            valor: x.get(4),
            realizada_em: aux.to_rfc3339()
        }
    }).collect();
    rtn
}
