use std::env;
use std::process::Command;
use std::process::Stdio;
use std::time::SystemTime;

const PROGRAM: &str = "jaketime";

fn main() {
    let start = SystemTime::now();

    let mut exit_code: i32 = 0;

    if env::args().count() > 1 {
        match env::args().nth(1) {
            None => {println!("{}: nothing to time", PROGRAM)}
            Some(name) => {
                let mut program = Command::new(name);

                program.stdin(Stdio::inherit());
                program.stdout(Stdio::inherit());
                program.stderr(Stdio::inherit());

                for i in 2..env::args().count() {
                    match env::args().nth(i) {
                        Some(arg) => { program.arg(arg); }
                        None => {}
                    }
                }

                match program.status() {
                    Ok(status) => {
                        match status.code() {
                            Some(code) => { exit_code = code;}
                            None => {}
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        exit_code = 1;
                    }
                }
            }

        }
    }

    match start.elapsed() {
        Ok(time) => { println!("{}: took {:#?}", PROGRAM, time); }
        Err(err) => {
            println!("{}", err);
            exit_code = 1;
        }
    }


    std::process::exit(exit_code)
}
