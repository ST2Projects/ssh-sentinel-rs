use rocket::http::Status;
use rocket::{post, State};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use sshcerts::{Certificate, CertType, PublicKey};
use crate::crypto;
use crate::model::User;
use crate::model::request::{KeySignRequest, KeySignResponse};

#[openapi(operation_id = "signKey")]
#[post("/signKey", format = "application/json", data = "<sign_request>")]
pub fn sign_key(_user: User, _db_pool: &State<PgPool>, secret_store: &State<SecretStore>, sign_request: Json<KeySignRequest>) -> Json<KeySignResponse> {

    let signing_key = crypto::get_signing_key();
    let cert_req_key = PublicKey::from_string(&*sign_request.0.pub_key).expect("Failed to parse ssh pub key");

    let cert = Certificate::builder(&cert_req_key, CertType::User, &signing_key.pubkey ).expect("Failed to build certificate")
        .serial(0xFF)
        .key_id("key_id")
        .principal("Test")
        .valid_after(0)
        .valid_before(0xFF)
        .set_extensions(Certificate::standard_extensions())
        .sign(&signing_key);

    Json(KeySignResponse{
        signed_key: cert.expect("Failed to sign cert").to_string()
    })
}