use std::path::Path;
use rocket::info;
use shuttle_secrets::SecretStore;
use ssh_key::{Algorithm, LineEnding, PrivateKey};
use ssh_key::rand_core::OsRng;

const SSH_SIGNING_PRIV_KEY_NAME: &str = "ssh_signing.key";
const SSH_SIGNING_PUB_KEY_NAME: &str = "ssh_signing.pub";
const SSH_SIGNING_SECRET_NAME: &str = "SSH_KEY_PASSWORD";

pub fn create_signing_key(secret_store: &SecretStore) -> std::io::Result<()> {

    let ssh_signing_priv_key = Path::new(SSH_SIGNING_PRIV_KEY_NAME);
    match ssh_signing_priv_key.exists() {
        true => {
            info!("Signing key already exists");
            Ok(())
        }
        false => {

            let private_key = PrivateKey::random(&mut OsRng, Algorithm::Ed25519).unwrap();
            let enc_private_key = private_key.encrypt(&mut OsRng, secret_store.get(SSH_SIGNING_SECRET_NAME).unwrap()).unwrap();
            enc_private_key.write_openssh_file(ssh_signing_priv_key, LineEnding::LF).unwrap();

            info!("Created new signing key");

            Ok(())
        }
    }
}

pub fn get_signing_key(secret_store: &SecretStore) -> ssh_key::Result<PrivateKey> {
    let ssh_signing_priv_key = Path::new(SSH_SIGNING_PRIV_KEY_NAME);

    let enc_priv_key = PrivateKey::read_openssh_file(ssh_signing_priv_key).unwrap();

    enc_priv_key.decrypt(secret_store.get(SSH_SIGNING_SECRET_NAME).unwrap())
}