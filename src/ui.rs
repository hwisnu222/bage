use clap::{Parser, Subcommand, Args};

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
pub struct Cli{
    #[command(subcommand)] 
   pub command: CommandEnum,

}

#[derive(Subcommand, Debug)]
pub enum CommandEnum{
    Encrypt(EncryptFilterArgs),
    Decrypt(DecryptFilterArgs)
}

#[derive(Args, Debug)]
pub struct EncryptFilterArgs{
    /// Encrypt all subfolders within the source directory
   #[arg(short, long, default_value=".")]
    pub path: String,

    /// Encrypt specific directories
    #[arg(short='i', long="include", value_delimiter=',')]
    pub include: Vec<String>,

    /// Encrypt all directories except the excluded ones
    #[arg(short='e', long="exclude", value_delimiter=',')]
    pub exclude: Vec<String>,

    /// Use random hex strings for filenames
    #[arg(short='x', long="hex-filenames")]
    pub hex: bool,

    #[arg(short, long)]
    pub dry_run: bool
}


#[derive(Args, Debug)]
pub struct DecryptFilterArgs{
    /// Encrypt all subfolders within the source directory
   #[arg(short, long, default_value=".")]
    pub path: String,

    /// Encrypt specific directories
    #[arg(short='i', long="include", value_delimiter=',')]
    pub include: Vec<String>,

    /// Encrypt all directories except the excluded ones
    #[arg(short='e', long="exclude", value_delimiter=',')]
    pub exclude: Vec<String>
}

