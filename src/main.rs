use serde::{Deserialize, Serialize};
use std::{io::*, process::Command};
use std::fs;
#[derive(Deserialize, Serialize, Default, Debug, Clone)]
struct CommandHist {
    command: String,
    sub: Vec<CommandHist>
}

fn main(){
    let mut command_hist: Vec<CommandHist> = serde_json::from_str(&fs::read_to_string("data/hist.json").unwrap()).unwrap();
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        // everything after the first whitespace character 
        //     is interpreted as args to the command
        let mut parts = input.trim().split(' ');
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();
        let depth = args.len();
        let mut current_index = command_hist.iter().position(|r| r.command == command);
        let mut indexes: Vec<usize> = vec![current_index.unwrap()];
        let mut endhist = command_hist[current_index.unwrap()].sub.clone();
        for i in 0..depth {
            current_index = endhist.iter().position(|r| r.command == args[i]);
            match current_index {
                Some(x) => {
                    endhist = endhist[x].sub.clone();
                    indexes.push(x)
                }
                None => {
                    Command::new("python3")
                        .arg("./python/edex.py")
                        .arg(&command)
                        .args(&args[0..i+1])
                        .status()
                        .expect("Python script failed");
                    command_hist = serde_json::from_str(&fs::read_to_string("data/hist.json").unwrap()).unwrap();
                }
            }
        }
        match current_index {
            Some(_)  => {
                let mut possible: Vec<String> = Vec::new();
                for i in endhist.iter() {
                    possible.push(i.command.clone());
                }
                println!("{:?}", possible);
            },
            None => ()
        }
    }
}