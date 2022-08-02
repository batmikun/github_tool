use std::{env, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("You need to pass the commit message as an argument");
    }

    let commit_message = &args[1];

    println!("Print commit message : {:?}", commit_message);

    let output = Command::new("git").arg("status").output();

    match output {
        Ok(_stdout) => match Command::new("git").arg("add").arg(".").output() {
            Ok(_stdout) => {
                match Command::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg(commit_message)
                    .output()
                {
                    Ok(_stdout) => {
                        println!("The commit was made succesfully")
                    }
                    Err(err) => println!("{:?}", err),
                }
            }
            Err(err) => println!("{:?}", err),
        },
        Err(err) => println!("{:?}", err),
    };
}
