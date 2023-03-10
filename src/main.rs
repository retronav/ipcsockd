// SPDX-FileCopyrightText: 2022 Pranav Karawale <https://karawale.in>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod cli;

use clap::Parser;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;
use std::error::Error;
use std::os::unix::prelude::OwnedFd;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::net::UnixListener;
use tokio::task;
use tokio::{fs, process};

static START_MESSAGE: &str = "OK";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::Cli::parse();
    let limit = cli.limit.clone();

    if Path::new(cli.path.as_str()).exists() {
        fs::remove_file(cli.path.clone()).await?;
    }

    let listener = UnixListener::bind(cli.path.clone())?;
    let thread_count = Arc::new(AtomicU32::new(0));

    let mainloop = tokio::spawn(async move {
        eprintln!("{}", START_MESSAGE);
        loop {
            if thread_count.load(Ordering::Relaxed) < limit {
                let (conn, _) = listener.accept().await.unwrap();
                let std_conn = conn.into_std().unwrap();
                let conn_as_fd = OwnedFd::try_from(std_conn).unwrap();

                let thread_count = Arc::clone(&thread_count);
                let cmd = cli.cmd.clone();

                thread_count.fetch_add(1, Ordering::Relaxed);
                task::spawn(async move {
                    handle_conn(cmd, conn_as_fd, thread_count).await;
                })
                .await
                .unwrap();
            }
        }
    });

    let mut signals = Signals::new(TERM_SIGNALS)?;

    for _ in signals.forever() {
        fs::remove_file(cli.path.clone()).await?;
        // mainloop will probably abort on its own when its dropped, but better
        // be safe.
        mainloop.abort();
        break;
    }

    Ok(())
}

async fn handle_conn(cmd: Vec<String>, conn_as_fd: OwnedFd, count: Arc<AtomicU32>) {
    let (_, cmd_args) = cmd.as_slice().split_first().unwrap();

    let stdin = conn_as_fd.try_clone().unwrap();
    let stdout = conn_as_fd.try_clone().unwrap();

    let mut handle = process::Command::new(cmd.first().to_owned().unwrap())
        .args(cmd_args.to_owned())
        .stdin(stdin)
        .stdout(stdout)
        .spawn()
        .unwrap();

    handle.wait().await.unwrap();
    count.fetch_sub(1, Ordering::SeqCst);
}
