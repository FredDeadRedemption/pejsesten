use std::env;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::net::TcpStream;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask always sits one level below the workspace root")
        .to_path_buf();

    let args: Vec<String> = env::args().skip(1).collect();
    let settings_prod = args.iter().any(|a| a == "--settings-prod");

    let result = match args.iter().find(|a| !a.starts_with("--")).map(String::as_str) {
        None | Some("native") => native(&root, settings_prod),
        Some("web") => web(&root, settings_prod),
        Some(task) => Err(format!("unknown task `{task}`, expected `native` or `web`, either with `--settings-prod`")),
    };

    if let Err(message) = result {
        eprintln!("{message}");
        std::process::exit(1);
    }
}

/// Native dev loop: server in the background, client in the foreground.
fn native(root: &Path, prod: bool) -> Result<(), String> {
    if !run(cargo(root, &server_args("build", prod))) {
        return Err("server build failed".into());
    }

    let log_path = env::temp_dir().join("pejsesten-server.log");
    let log = File::create(&log_path).map_err(|e| format!("{}: {e}", log_path.display()))?;
    let server = Command::new(root.join("target/debug/server"))
        .current_dir(root)
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

    match run(cargo(root, &["run", "-p", "client"])) {
        true => Ok(()),
        false => Err("client exited with an error".into()),
    }
}

/// Web dev loop: wasm bundle plus assets in dist/, served by the server.
fn web(root: &Path, prod: bool) -> Result<(), String> {
    let wasm = &[
        "build",
        "-p",
        "client",
        "--target",
        "wasm32-unknown-unknown",
        "--release",
    ];
    if !run(cargo(root, wasm)) {
        return Err("wasm build failed".into());
    }

    let dist = root.join("dist");
    fs::create_dir_all(&dist).map_err(|e| format!("{}: {e}", dist.display()))?;

    // symlinks, so a wasm rebuild only needs a browser refresh
    let links = [
        (
            "target/wasm32-unknown-unknown/release/client.wasm",
            "client.wasm",
        ),
        ("client/index.html", "index.html"),
        ("client/mq_js_bundle.js", "mq_js_bundle.js"),
        ("client/app_ws.js", "app_ws.js"),
        ("client/app_storage.js", "app_storage.js"),
        ("client/app_location.js", "app_location.js"),
        ("client/static/favicon.png", "favicon.png"),
        ("client/static/media", "media"),
    ];
    for (target, name) in links {
        link(&root.join(target), &dist.join(name))?;
    }

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    println!("\n→ http://localhost:{port}\n");

    let mut server = cargo(root, &server_args("run", prod));
    server.env("STATIC_DIR", "dist");
    match run(server) {
        true => Ok(()),
        false => Err("server exited with an error".into()),
    }
}

fn link(target: &Path, at: &Path) -> Result<(), String> {
    match fs::remove_file(at) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::NotFound => {}
        Err(e) => return Err(format!("{}: {e}", at.display())),
    }
    symlink(target, at).map_err(|e| format!("{}: {e}", at.display()))
}

fn server_args(subcommand: &'static str, prod: bool) -> Vec<&'static str> {
    let mut args = vec![subcommand, "-p", "server"];
    if prod {
        args.extend(["--features", "settings-prod"]);
    }
    args
}

fn cargo(root: &Path, args: &[&str]) -> Command {
    let mut command = Command::new(env!("CARGO"));
    command.args(args).current_dir(root);
    command
}

fn run(mut command: Command) -> bool {
    command.status().is_ok_and(|status| status.success())
}

/// Kills the server on every exit path, including the error ones.
struct Reaper(Child);

impl Drop for Reaper {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
