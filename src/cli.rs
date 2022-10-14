// SPDX-FileCopyrightText: 2022 Pranav Karawale <https://karawale.in>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

#[allow(unused_imports)]
use clap::{CommandFactory, Parser};

/// A super-server daemon for UNIX domain sockets.
///
/// ipcsockd (inter process communication socket daemon) binds to a UNIX domain
/// socket, spawns the given command for each connection in a thread with the
/// standard input and standard output of the thread piped to the socket.
///
/// If the command to be executed contains flags of its own, they might conflict
/// with the flags passed to ipcsockd itself. To work around this issue, pass '--'
/// before the actual command.
///
/// EXAMPLE: ipcsockd ./a.sock -- mycommand --arg1 --arg2 yes
///
/// Here, the arguments --arg1 and --arg2 are passed to mycommand during the time
/// of execution instead of being parsed by ipcsockd.
///
/// WARNING: ipcsockd will remove the socket if it already exists.
///
/// ipcsockd prints a message "OK" to standard error when it is ready to receive
/// requests. This can be utilised to check the status of the server.
///
/// ipcsockd limits the maximum amount of concurrent requests at a particular point
/// in time, which can be configured using flags. Advanced rate-limiting and similar
/// utilities are left to be handled by a reverse proxy.
#[derive(Parser, Debug)]
#[command(author, version, about, verbatim_doc_comment)]
pub struct Cli {
    /// Path of the UNIX socket to listen on.
    pub path: String,

    /// Command with arguments to execute for every incoming connection.
    pub cmd: Vec<String>,

    /// Maximum concurrent connections per second.
    #[arg(short, long, default_value_t = 40u32)]
    pub limit: u32,
}
