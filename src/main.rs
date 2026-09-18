use std::{io::{self}};
use bage::{decrypt, encrypt, ui::{Cli, CommandEnum}};
use clap::{Parser};

fn main() -> io::Result<()>{
    let args = Cli::parse();

    match args.command{
        CommandEnum::Encrypt (filter_args)=>{
            match encrypt( filter_args){
                Ok(_)=> println!("Success encrypt files"),
                Err(error) =>{
                    println!("Error: {}", error.to_string());
                } 
            };
        }
        CommandEnum::Decrypt (filter_args)=>{
            match decrypt( filter_args){
                Ok(_) => println!("Success decrypt files"),
                Err(error) => {
                    println!("Error: {}", error.to_string())
                }
            };
        }
    }

    Ok(())
}
