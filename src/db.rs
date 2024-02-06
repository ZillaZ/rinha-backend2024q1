use chrono::DateTime;
use tokio_postgres::{tls::NoTls, *};
use std::{collections::HashMap, env};
use crate::*;

#[derive(Serialize, Deserialize)]
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

pub async fn connect() -> Client {
    let vars = env::vars().collect::<HashMap<String, String>>();
    let connection_string= vars.get("DB_CONN").unwrap();
    let (client, conn) = tokio_postgres::connect(connection_string, NoTls).await.unwrap();
    rocket::tokio::spawn(async move {
        if let Err(e) = conn.await {
            panic!("muitosexo {}", e);
        }
    });
    client
}

pub async fn get_client(client: &Client, id: i32) -> Answer {
    let data = client.query(&format!("select * from clientes where id = {}", id), &[]).await.unwrap();    
    let row = data.get(0).unwrap();
    Answer {
        limite: row.get(1),
        saldo: row.get(2)
    }
}

pub async fn client_exists(client: &Client, id: i32) -> bool {
    let data = client.query(&format!("select * from clientes where id = {}", id), &[]).await.unwrap();    
    data.len() > 0
}

pub async fn new_transaction(client: &Client, id: i32, data: Transacao) {
    let now = chrono::Utc::now();
    let statement = client.prepare("insert into transacoes values ($1, $2, $3, $4, $5);").await.unwrap();
    client.execute(&statement, &[&id, &data.tipo, &data.descricao, &now, &data.valor]).await.unwrap();
}

pub async fn update_client(dbclient: &Client, id: i32, data: &Transacao, client: &mut Answer) -> bool {
    client.saldo -= data.valor;
    if client.saldo < client.limite * -1 {
        return false
    }
    let statement = dbclient.prepare("update clientes set saldo = ($1) where id = ($2)").await.unwrap();
    dbclient.execute(&statement, &[&client.saldo, &id]).await.unwrap();
    true
}

pub async fn get_transactions(client: &Client, id: i32) -> Vec<ITransacao> {
    let data = client.query(&format!("SELECT * FROM transacoes WHERE id_cliente = {} ORDER BY realizada_em DESC LIMIT 10;", id), &[]).await.unwrap();
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