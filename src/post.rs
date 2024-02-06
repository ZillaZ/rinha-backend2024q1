use rocket::http::Status;

use crate::*;

#[post("/clientes/<id>/transacoes", data = "<input>")]
pub async fn transacoes(id: i32, input: Json<Transacao>) -> Result<status::Custom<Json<Answer>>, Status> {
    if input.valor < 0 {
        return Err(Status::NotAcceptable);
    }
    let dbclient= connect().await;
    if !client_exists(&dbclient, id).await {
        return Err(Status::NotFound);
    }
    let mut answer = get_client(&dbclient, id).await;
    let success = update_client(&dbclient, id, &input.0, &mut answer).await;
    if !success {
        return Err(Status::UnprocessableEntity);
    }
    new_transaction(&dbclient, id, input.0).await;
    Ok(status::Custom(Status::Ok, Json(answer)))
}