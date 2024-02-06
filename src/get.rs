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
    ultimas_transacoes: Vec<ITransacao>
}

#[get("/clientes/<id>/extrato")]
pub async fn extrato(id: i32) -> Result<status::Custom<Json<Extrato>>, Status> {
    let dbclient = connect().await;
    if !client_exists(&dbclient, id).await {
        return Err(Status::NotFound);
    }
    let transactions = get_transactions(&dbclient, id).await;
    let client = get_client(&dbclient, id).await;
    let now = chrono::Utc::now().to_rfc3339();
    Ok(status::Custom(Status::Ok, Json(
        Extrato {
            saldo: Saldo {
                total: client.saldo,
                limite: client.limite,
                data_extrato: now
            },
            ultimas_transacoes: transactions
        }
    )))
}