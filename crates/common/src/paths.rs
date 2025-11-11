use std::{
  env::{self, temp_dir},
  path::PathBuf,
};

use dirs::runtime_dir;

pub fn get_socket_path() -> PathBuf {
  if let Some(p) = env::var_os("FENSTER_SOCKET_PATH") {
    p.into()
  } else {
    let mut socket_path = runtime_dir().unwrap_or_else(temp_dir);
    socket_path.push("fenster");
    socket_path.push("daemon.sock");

    socket_path
  }
}
