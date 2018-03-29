extern crate minigrep;


use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

// #[derive(Debug)]
// struct CmdArgs {
//     filename: String,
// }
// impl CmdArgs {
//     fn new (args: Vec<String>) -> Result <CmdArgs, &'static str> {
//         if args.len() < 3 {
//             return Err("fucking wrong");
//         }
//         let mut cmd = CmdArgs {
//             filename: String::new(),
//         };
//         for (i, item) in args.iter().enumerate() {
//             match item.as_str() {
//                 "--filename" => {
//                     if (i + 1) < args.len() {
//                         cmd.filename = args[i + 1].clone();
//                     }
//                 },
//                 _ => (),
//             };
//             println!("{} {:?}", i, item);
//         }
//         Ok(cmd)
//     }
// }
#[derive(Debug)]
struct CmdArgs<'a> {
    filename: &'a str,
    match_str: &'a str,
}
impl<'a> CmdArgs<'a> {
    fn new (args:&'a Vec<String>) -> Result <CmdArgs<'a>, &'static str> {
        if args.len() < 3 {
            return Err("fuck wrong nums");
        }
        let mut cmd = CmdArgs {
            filename: "",
            match_str: "",
        };
        for (i, item) in args.iter().enumerate() {
            println!("num: {}", i);
            match item.as_str() {
                "--filename" => {
                    if (i + 1) < args.len() {
                        cmd.filename = &args[i + 1];
                    }
                },
                "--match" => {
                    if (i + 1) < args.len() {
                        cmd.match_str = &args[i + 1];
                    }
                }
                _ => (),
            };
            println!("{} {:?}", i, item);
        }
        Ok(cmd)
    }
}

fn main() {
    minigrep::test();

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let cmd = CmdArgs::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{:?}", cmd);
    
    let mut file:File = File::open(&cmd.filename).expect("Cargo.toml");
    let mut buf = String::new();
    let result = file.read_to_string(&mut buf);



    // println!("{:?}", cmd);

    for line in buf.lines() {
        if line.contains(&cmd.match_str) {
            println!("{}", line);
        }
    }

    // println!("{}", buf.as_str());
}
