use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct CmdArgs {
    filename: String,
}
impl CmdArgs {
    fn new (args: Vec<String>) -> CmdArgs{
        let mut cmd = CmdArgs {
            filename: String::new(),
        };
        for (i, item) in args.iter().enumerate() {
            match item.as_str() {
                "--filename" => {
                    if (i + 1) < args.len() {
                        cmd.filename = args[i + 1].clone();
                    }
                },
                _ => (),
            };
            println!("{} {:?}", i, item);
        }
        cmd
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = CmdArgs::new(args);
    let mut file:File = File::open(&cmd.filename).expect("Cargo.toml");
    let mut buf = String::new();
    let result = file.read_to_string(&mut buf);
    println!("{:?}", cmd);
    println!("{}", buf.as_str());
}
