use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use anyhow::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

pub fn watch_files(paths: &[String], restart_tx: tokio::sync::mpsc::Sender<()>) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    for path in paths {
        watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
    }

    std::thread::spawn(move || {
        let mut last_restart = std::time::Instant::now();
        loop {
            match rx.recv() {
                Ok(_) => {
                    if last_restart.elapsed() > Duration::from_secs(1) {
                        if let Err(e) = restart_tx.blocking_send(()) {
                            eprintln!("Failed to send restart signal: {}", e);
                        }
                        last_restart = std::time::Instant::now();
                    }
                }
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        }
    });

    Ok(())
}
