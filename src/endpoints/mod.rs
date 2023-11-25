use std::time::{SystemTime, UNIX_EPOCH};

use rocket::{post, State};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use ssh_key::certificate::{Builder, CertType};
use ssh_key::PublicKey;
use ssh_key::rand_core::OsRng;
use crate::crypto;
use crate::model::User;
use crate::model::request::{KeySignRequest, KeySignResponse};

#[openapi(operation_id = "signKey")]
#[post("/signKey", format = "application/json", data = "<sign_request>")]
pub fn sign_key(user: User, _db_pool: &State<PgPool>, secret_store: &State<SecretStore>, sign_request: Json<KeySignRequest>) -> Json<KeySignResponse> {

    let signing_key = crypto::get_signing_key(secret_store).unwrap();
    let cert_req_key = PublicKey::from_openssh(&sign_request.0.pub_key).expect("Failed to parse ssh pub key");

    let valid_after = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let valid_before = valid_after + 500;

    let mut cert_builder = Builder::new_with_random_nonce(
        &mut OsRng,
        cert_req_key,
        valid_after,
        valid_before
    ).unwrap();

    cert_builder.serial(42).unwrap(); // Change!
    cert_builder.key_id("key id").unwrap();
    cert_builder.cert_type(CertType::User).unwrap();
    cert_builder.valid_principal(user.user_name.unwrap()).unwrap();
    cert_builder.comment("hello").unwrap();

    let cert = cert_builder.sign(&signing_key).unwrap();

    Json(KeySignResponse{
        signed_key: cert.to_openssh().unwrap()
    })
}