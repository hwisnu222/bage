use std::io::{self};
use clap::{Parser, Subcommand};

use bage::{decrypt, encrypt};


#[derive(Subcommand, Debug)]
enum CommandEnum{
    Encrypt {
        #[arg(short, long, default_value=".")]
        path: String
    },
    Decrypt{
        #[arg(short, long, default_value=".")]
        path: String
    }

}

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli{
    #[command(subcommand)] 
    command: CommandEnum
}

fn main() -> io::Result<()>{
    let args = Cli::parse();

    match args.command{
        CommandEnum::Encrypt { path }=>{
            match encrypt(path){
                Ok(_)=> println!("Success encrypt files"),
                Err(error) =>{
                    println!("Error: {}", error.to_string());
                } 
            };
        }
        CommandEnum::Decrypt { path }=>{
            match decrypt(path){
                Ok(_) => println!("Success decrypt files"),
                Err(error) => {
                    println!("Error: {}", error.to_string())
                }
            };
        }
    }

    Ok(())
}
