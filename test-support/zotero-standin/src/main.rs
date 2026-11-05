//! `zotero-standin <fixtures-dir> [port]`: serves recorded Zotero fixtures on 127.0.0.1.
use zotero_standin::StandIn;

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(dir) = args.next() else {
        eprintln!("usage: zotero-standin <fixtures-dir> [port]");
        std::process::exit(2);
    };
    let port = args.next().and_then(|p| p.parse().ok()).unwrap_or(23119);
    match StandIn::start(dir, port) {
        Ok(server) => {
            println!("zotero-standin listening on 127.0.0.1:{}", server.port());
            loop {
                std::thread::sleep(std::time::Duration::from_secs(3600));
            }
        }
        Err(err) => {
            eprintln!("cannot start: {err}");
            std::process::exit(1);
        }
    }
}
