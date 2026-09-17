use std::{io::{self}};
use clap::{Parser, Subcommand, Args};

use bage::{DecryptOptions, EncryptOptions, decrypt, encrypt};

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli{
    #[command(subcommand)] 
    command: CommandEnum,

}

#[derive(Subcommand, Debug)]
enum CommandEnum{
    Encrypt(EncryptFilterArgs),
    Decrypt(DecryptFilterArgs)
}

#[derive(Args, Debug)]
struct EncryptFilterArgs{
    /// Encrypt all subfolders within the source directory
   #[arg(short, long, default_value=".")]
    path: String,

    /// Encrypt specific directories
    #[arg(short='i', long="include", value_delimiter=',')]
    include: Vec<String>,

    /// Encrypt all directories except the excluded ones
    #[arg(short='e', long="exclude", value_delimiter=',')]
    exclude: Vec<String>,

    /// Use random hex strings for filenames
    #[arg(short='x', long="hex-filenames")]
    hex: bool
}


#[derive(Args, Debug)]
struct DecryptFilterArgs{
    /// Encrypt all subfolders within the source directory
   #[arg(short, long, default_value=".")]
    path: String,

    /// Encrypt specific directories
    #[arg(short='i', long="include", value_delimiter=',')]
    include: Vec<String>,

    /// Encrypt all directories except the excluded ones
    #[arg(short='e', long="exclude", value_delimiter=',')]
    exclude: Vec<String>
}

fn main() -> io::Result<()>{
    let args = Cli::parse();

    match args.command{
        CommandEnum::Encrypt (filter_args)=>{
            let options = EncryptOptions{
                include: filter_args.include,
                exclude: filter_args.exclude,
                hex: filter_args.hex
            };

            match encrypt(filter_args.path, options){
                Ok(_)=> println!("Success encrypt files"),
                Err(error) =>{
                    println!("Error: {}", error.to_string());
                } 
            };
        }
        CommandEnum::Decrypt (filter_args)=>{
            let options = DecryptOptions{
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
