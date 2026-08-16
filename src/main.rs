use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Clone)]
struct SearchResult {
    engine: &'static str,
    version: String,
    recommended: bool,
    engine_id: u8,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "install" => {
            if args.len() < 3 {
                println!("Error: Specify a package to install.");
                return;
            }
            handle_install(&args[2]);
        }
        "remove" => {
            if args.len() < 3 {
                println!("Error: Specify a package to remove.");
                return;
            }
            handle_passthrough("apt", &["remove", &args[2..].join(" ")]);
        }
        "clean" => handle_clean(),
        "cache" => handle_cache(),
        "update" => handle_update(),
        "search" => {
            if args.len() >= 3 {
                let _ = search_all(&args[2]);
            }
        }
        _ => print_help(),
    }
}

fn handle_install(pkg: &str) {
    println!("searching the packages...");
    let results = search_all(pkg);

    if results.is_empty() {
        println!("No packages found across engines for: {}", pkg);
        return;
    }

    println!("\nthe package/s \"{}\" found on {}/3 Engines", pkg, results.len());
    for (idx, res) in results.iter().enumerate() {
        let tag = if res.recommended { "(recommended)" } else { "(Not recommended)" };
        print!("[{}] {} {} v{}  |  ", idx + 1, res.engine, tag, res.version);
    }

    print!("\nChoose engine [1-{}]: ", results.len());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let parsed: Result<usize, _> = input.trim().parse();
    let choice = match parsed {
        Ok(num) if num > 0 && num <= results.len() => num - 1,
        _ => return,
    };

    let selected = &results[choice];
    print!("choosing {}... confirm? [(y)es (n)o (b)ack]: ", selected.engine);
    io::stdout().flush().unwrap();

    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).unwrap();
    if confirm.trim().eq_ignore_ascii_case("y") {
        match selected.engine_id {
            1 => run_cmd("sudo", &["apt", "install", "-y", pkg]),
            2 => run_cmd("flatpak", &["install", "-y", pkg]),
            3 => run_cmd("sudo", &["snap", "install", pkg]),
            _ => (),
        }
    }
}

fn search_all(pkg: &str) -> Vec<SearchResult> {
    let (tx, rx) = mpsc::channel();

    // 1. Check APT (Thread 1)
    let tx1 = tx.clone();
    let pkg_apt = pkg.to_string();
    thread::spawn(move || {
        let output = Command::new("apt-cache")
            .args(["policy", &pkg_apt])
            .output();
        if let Ok(out) = output {
            let txt = String::from_utf8_lossy(&out.stdout);
            if let Some(ver) = extract_apt_version(&txt) {
                let _ = tx1.send(SearchResult {
                    engine: "apt",
                    version: ver,
                    recommended: true,
                    engine_id: 1,
                });
            }
        }
    });

    // 2. Check Flatpak (Thread 2)
    let tx2 = tx.clone();
    let pkg_flat = pkg.to_string();
    thread::spawn(move || {
        let output = Command::new("flatpak")
            .args(["search", &pkg_flat])
            .output();
        if let Ok(out) = output {
            if !out.stdout.is_empty() {
                let _ = tx2.send(SearchResult {
                    engine: "Flatpak",
                    version: "latest".to_string(),
                    recommended: false,
                    engine_id: 2,
                });
            }
        }
    });

    // 3. Check Snap (Thread 3)
    let tx3 = tx.clone();
    let pkg_snap = pkg.to_string();
    thread::spawn(move || {
        let output = Command::new("snap")
            .args(["find", &pkg_snap])
            .output();
        if let Ok(out) = output {
            let txt = String::from_utf8_lossy(&out.stdout);
            if txt.lines().count() > 1 {
                let _ = tx3.send(SearchResult {
                    engine: "Snap",
                    version: "latest".to_string(),
                    recommended: false,
                    engine_id: 3,
                });
            }
        }
    });

    drop(tx);
    rx.into_iter().collect()
}

fn handle_clean() {
    println!("--- Cleaning System Packages ---");
    run_cmd("sudo", &["apt", "autoremove", "-y"]);
    run_cmd("flatpak", &["uninstall", "--unused", "-y"]);
    run_cmd("sudo", &["snap", "set", "system", "refresh.retain=2"]);
}

fn handle_cache() {
    println!("--- Cleaning Caches ---");
    run_cmd("sudo", &["apt", "clean"]);
    run_cmd("flatpak", &["repair"]);
}

fn handle_update() {
    println!("--- Updating APT ---");
    run_cmd("sudo", &["apt", "update"]);
    run_cmd("sudo", &["apt", "upgrade", "-y"]);
    println!("--- Updating Flatpak ---");
    run_cmd("flatpak", &["update", "-y"]);
    println!("--- Updating Snap ---");
    run_cmd("sudo", &["snap", "refresh"]);
}

fn extract_apt_version(output: &str) -> Option<String> {
    for line in output.lines() {
        if line.contains("Candidate:") && !line.contains("(none)") {
            return Some(line.replace("Candidate:", "").trim().to_string());
        }
    }
    None
}

fn run_cmd(cmd: &str, args: &[&str]) {
    Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .ok();
}

fn handle_passthrough(cmd: &str, args: &[&str]) {
    run_cmd("sudo", &[cmd, args.join(" ").as_str()]);
}

fn print_help() {
    println!("cpm - c00l-P4CK4G3-M4N4G3R");
    println!("Usage: cpm <install|remove|clean|cache|update|search> [package]");
}
