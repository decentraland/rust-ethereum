use std::sync::OnceLock;

use crate::server::SignServer;

pub mod cabi;
pub mod server;
pub mod sign;
pub mod verify;

static SIGN_SERVER: OnceLock<SignServer> = OnceLock::new();

pub(crate) fn sign_server() -> &'static SignServer {
    SIGN_SERVER.get_or_init(SignServer::default)
}
