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
            None => println!("{}: nothing to time", PROGRAM),
            Some(name) => {
                let mut program = Command::new(name);

                program.stdin(Stdio::inherit());
                program.stdout(Stdio::inherit());
                program.stderr(Stdio::inherit());

                for i in 2..env::args().count() {
                    if let Some(arg) = env::args().nth(i) {
                        program.arg(arg);
                    }
                }

                match program.status() {
                    Ok(status) => if let Some(code) = status.code() {
                        exit_code = code;
                    },
                    Err(err) => {
                        println!("{}", err);
                        exit_code = 1;
                    }
                }
            }
        }
    }

    match start.elapsed() {
        Ok(time) => {
            println!("{}: took {:#?}", PROGRAM, time);
        }
        Err(err) => {
            println!("{}", err);
            exit_code = 1;
        }
    }

    std::process::exit(exit_code)
}
