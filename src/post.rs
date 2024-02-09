use rocket::http::Status;

use crate::*;

#[post("/clientes/<id>/transacoes", data = "<input>")]
pub async fn transacoes(id: i32, input: Json<Transacao>, pool: &State<Pool>) -> Result<status::Custom<Json<Answer>>, Status> {
    if id > 5 || id < 0 {
        return Err(Status::NotFound);
    }else if input.descricao.len() < 1 || input.valor < 0 || input.tipo != "c" && input.tipo != "d" || input.descricao.len() > 10 {
        return Err(Status::UnprocessableEntity);
    }

    let dbclient= pool.get().await.unwrap();
    let mut answer = get_client(&dbclient, id).await;
    let success = update_client(&dbclient, id, &input.0, &mut answer).await;
    
    if !success {
        return Err(Status::UnprocessableEntity);
    }

    Ok(status::Custom(Status::Ok, Json::from(answer)))
}
