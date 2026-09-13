use std::env;
use std::fs::File;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    if let Err(message) = dev() {
        eprintln!("{message}");
        std::process::exit(1);
    }
}

/// Native dev loop: server in the background, client in the foreground.
fn dev() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask always sits one level below the workspace root")
        .to_path_buf();

    if !cargo(&root, &["build", "-p", "server"]) {
        return Err("server build failed".into());
    }

    let log_path = env::temp_dir().join("pejsesten-server.log");
    let log = File::create(&log_path).map_err(|e| format!("{}: {e}", log_path.display()))?;
    let server = Command::new(root.join("target/debug/server"))
        .current_dir(&root)
        .stdout(Stdio::from(log.try_clone().map_err(|e| e.to_string())?))
        .stderr(Stdio::from(log))
        .spawn()
        .map_err(|e| format!("cannot start server: {e}"))?;
    let mut server = Reaper(server);

    println!("server: pid {}, log: {}", server.0.id(), log_path.display());

    // client has no reconnect, so do not start it until the port accepts
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    while TcpStream::connect(format!("127.0.0.1:{port}")).is_err() {
        match server.0.try_wait() {
            Ok(Some(status)) => return Err(format!("server died ({status}), see log")),
            Err(e) => return Err(format!("cannot poll server: {e}")),
            Ok(None) => sleep(Duration::from_millis(200)),
        }
    }

    match cargo(&root, &["run", "-p", "client"]) {
        true => Ok(()),
        false => Err("client exited with an error".into()),
    }
}

fn cargo(root: &Path, args: &[&str]) -> bool {
    Command::new(env!("CARGO"))
        .args(args)
        .current_dir(root)
        .status()
        .is_ok_and(|status| status.success())
}

/// Kills the server on every exit path, including the error ones.
struct Reaper(Child);

impl Drop for Reaper {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
