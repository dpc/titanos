use std::process::{Command, Output};

pub fn main() {
    match Command::new("git").arg("clone").arg("--depth").arg("1")
        .arg("https://github.com/rust-lang/rust.git").output() {
            Ok(Output { status: exit, stdout: _ , stderr: err }) => {
                if exit.success() {
                } else {
                    println!("Error getting rustc version: {}", String::from_utf8(err).unwrap());
                }
            }
            Err(err) => {
                println!("Error cloning rustc git repo: {}", err);
            }
        }
}
