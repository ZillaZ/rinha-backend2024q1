use rocket::http::Status;

use crate::*;


#[derive(Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
struct Saldo {
    total: i32,
    data_extrato: String,
    limite: i32
}

#[derive(Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
struct Transacao {
    valor: u64,
    tipo: char,
    descricao: String,
    realizada_em: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct Extrato {
    saldo: Saldo,
    ultimas_transacoes: Option<Vec<ITransacao>>
}

#[get("/clientes/<id>/extrato")]
pub async fn extrato(id: i32, pool: &State<Pool>) -> Result<status::Custom<Json<Extrato>>, Status> {
    if id > 5 || id < 0 {
        return Err(Status::NotFound);
    }
    
    let dbclient = pool.get().await.unwrap();
    let transactions = get_transactions(&dbclient, id).await;
    let now = chrono::Utc::now().to_rfc3339();

    Ok(status::Custom(Status::Ok, Json::from(
        Extrato {
            saldo: Saldo {
                total: transactions.client.saldo,
                limite: transactions.client.limite,
                data_extrato: now
            },
            ultimas_transacoes: transactions.transacoes
        }
    )))
}
