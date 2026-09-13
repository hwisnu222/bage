use std::{io::{self}};
use clap::{Parser, Subcommand, Args};

use bage::{Options, decrypt, encrypt};




#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli{
    #[command(subcommand)] 
    command: CommandEnum,

}

#[derive(Subcommand, Debug)]
enum CommandEnum{
    Encrypt(FilterArgs),
    Decrypt(FilterArgs)
}

#[derive(Args, Debug)]
struct FilterArgs{
   #[arg(short, long, default_value=".")]
    path: String,

    #[arg(short='i', long="include", value_delimiter=',')]
    include: Vec<String>,

    #[arg(short='e', long="exclude", value_delimiter=',')]
    exclude: Vec<String>
}

fn main() -> io::Result<()>{
    let args = Cli::parse();


    match args.command{
        CommandEnum::Encrypt (filter_args)=>{
            let options = Options{
                include: filter_args.include,
                exclude: filter_args.exclude
            };

            match encrypt(filter_args.path, options){
                Ok(_)=> println!("Success encrypt files"),
                Err(error) =>{
                    println!("Error: {}", error.to_string());
                } 
            };
        }
        CommandEnum::Decrypt (filter_args)=>{
            let options = Options{
                include: filter_args.include,
                exclude: filter_args.exclude
            };

            match decrypt(filter_args.path, options){
                Ok(_) => println!("Success decrypt files"),
                Err(error) => {
                    println!("Error: {}", error.to_string())
                }
            };
        }
    }

    Ok(())
}
