use actix_identity::Identity;
use actix_web::{HttpResponse, Responder, Result};

pub async fn delete(identity: Identity) -> Result<impl Responder> {
    if identity.identity().is_none() {
        return Ok(HttpResponse::Ok().finish());
    }

    identity.forget();

    Ok(HttpResponse::Ok().finish())
}
