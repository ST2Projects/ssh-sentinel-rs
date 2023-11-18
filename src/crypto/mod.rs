use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use rocket::info;
use shuttle_runtime::__internals::tracing_subscriber::fmt::MakeWriter;
use sshcerts::PrivateKey;
use sshcerts::ssh::KeyTypeKind;

mod ssh;

pub fn create_signing_key() -> std::io::Result<()> {

    let ssh_signing_priv_key = Path::new("ssh_signing.key");
    let ssh_signing_pub_key = Path::new("ssh_signing.pub");
    return match ssh_signing_priv_key.exists() {
        true => {
            info!("Signing key already exists");
            Ok(())
        }
        false => {
            let new_priv_key = PrivateKey::new(KeyTypeKind::Ed25519, "SSH Signing Private Key").expect("Failed to create new key");

            let priv_key_file = File::create(ssh_signing_priv_key)?;

            new_priv_key.write(&mut priv_key_file.make_writer())?;

            info!("Created new signing key");

            Ok(())
        }
    }
}

pub fn get_signing_key() -> PrivateKey {
    PrivateKey::from_path(Path::new("ssh_signing.key")).expect("Failed to read private key")
}