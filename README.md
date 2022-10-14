# ipcsockd

ipcsockd (**i**nter-**p**rocess **c**ommunication **sock**et **d**aemon) is a
super-server daemon for UNIX domain sockets. Think of it as `xinetd` but for
UNIX domain sockets.

## Examples

To run a command `myserver http` everytime a connection to the socket `a.sock`
in the current directory is made:

```
ipcsockd ./a.sock myserver http
```

For each incoming connection, `ipcsockd` will execute the command `myserver
http` in a separate thread, with passing input from the socket to the thread's
stdin, and passing output from the thread's stdout back to the socket.

To run a command with flags (ex. `myserver http --cool-feature -v`), prepend
the command with a `--` to escape those arguments:

```
ipcsockd ./a.sock -- myserver http --cool-feature -v 
```

If `--` is not provided, `ipcsockd` will assume those flags were meant for
itself rather than for the command.

To perform the same given above, but limit concurrent connections to 20:

```
ipcsockd -l 20 ./a.sock -- myserver http --cool-feature -v
```
`ipcsockd` is not limited to just HTTP applications. You can do much more with 
it.

For more technical information, run `ipcsockd --help`.

### Real demo

This demo requires `ncat` (`nc`) to be installed.

Here, we create a socket `/tmp/cat.sock` which echoes what you send to the
socket back. Bind to the socket using `ipcsockd` and instruct it to run
`cat(1)` for each connection.

```
ipcsockd /tmp/cat.sock cat
```

It should print "OK". Now, in another terminal, run

```
echo "hello" | nc -U /tmp/cat.sock
```

It will print "hello" back!

```
$ echo "hello" | nc -U /tmp/cat.sock
hello
```

You can also try this using `curl(1)`:

```
$ curl --unix-socket /tmp/cat.sock http://localhost/hello --http0.9
GET /hello HTTP/1.1
Host: localhost
User-Agent: curl/7.82.0
Accept: */*
```

It printed the request curl sent to the socket.

## Install

ipcsockd requires a Rust toolchain installed. To build a debug build (ideal for
testing/reporting bugs), run

```
cargo build
```

For a release build (no debug information, ideal to distribute or run in
production), run

```
cargo build --release
```

## Relevant/similar work

- [s6-ipcserver](https://skarnet.org/software/s6/s6-ipcserver.html)

## Copyright and license notices

Copyright (C) 2022  Pranav Karawale <https://karawale.in>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

This notice and license notices of dependencies can be found in the `LICENSE`
file.

