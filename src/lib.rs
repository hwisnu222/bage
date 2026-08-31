use std::{fs::File, io::{self, BufReader, BufWriter, ErrorKind}, path::Path};

use age::{Decryptor, Encryptor, secrecy::SecretString};
use tar::{Archive, Builder};
use walkdir::WalkDir;

pub fn encrypt(path: String) -> io::Result<()>{
    println!("encrypt path: {}", path);

    let target = Path::new(&path);

    if !target.exists(){
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "Path doesn't exists"
        ));
    };

    let passhrase = rpassword::prompt_password("Passphrase: ").unwrap();
    let confirm_passphrase = rpassword::prompt_password("Confirm passphrase: ").unwrap();

    let is_same = passhrase == confirm_passphrase;

    if !is_same{
        return Err(io::Error::new(ErrorKind::InvalidInput, "Passpharse doesn't match"));
    }


    for entry in WalkDir::new(target)
    .min_depth(1)
    .max_depth(1){
        let entry_dir = entry?;
        let entry_path = entry_dir.path();
        let passphrase_c = passhrase.clone();

        if entry_path.is_dir(){
            let output_path = format!("{}.tar.age", entry_path.display().to_string());
            let output_file = File::create(output_path)?;
            
            let buffered_writer = BufWriter::new(output_file);

            let encryptor = Encryptor::with_user_passphrase(age::secrecy::SecretString::new(passphrase_c.into()));
            let encrypt_stream = encryptor.wrap_output(buffered_writer)?;

            let Some(file_stem) = entry_path.file_stem().and_then(|x| x.to_str()) else {
                return Err(io::Error::new(
                    ErrorKind::InvalidInput, 
                    "filename or path is invalid"
                ));
            };

            let mut archive = Builder::new(encrypt_stream);
            archive.append_dir_all(file_stem, entry_path)?;


            let encrypted_stream = archive.into_inner()?;
            encrypted_stream.finish()?;

            println!("{} is encrypted", entry_path.display().to_string());

        }
    }
    
    Ok(())
}

pub fn decrypt(path: String) -> io::Result<()>{
    let passhrase = rpassword::prompt_password("Passphrase: ").unwrap();
    let confirm_passphrase = rpassword::prompt_password("Confirm passphrase: ").unwrap();

    let is_same = passhrase == confirm_passphrase;

    if !is_same{
        return Err(io::Error::new(ErrorKind::InvalidInput, "Passpharse doesn't match"));
    }

    for entry in WalkDir::new(&path)
    .min_depth(1)
    .max_depth(1){
        let entry_dir = entry?;
        let entry_path = entry_dir.path();
        let passphrase_c = passhrase.clone();

        if entry_path.extension().is_some_and(|e| e == "age"){
            let input_file = File::open(entry_path)?;
            let buffered_reader = BufReader::new(input_file);
            
            let decryptor = Decryptor::new(buffered_reader)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            let identity = age::scrypt::Identity::new(SecretString::from(passphrase_c));
            let decrypted_stream = decryptor.decrypt(std::iter::once(&identity as &dyn age::Identity))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            let mut archive = Archive::new(decrypted_stream);

            archive.unpack(&path)?;

            println!("all file is decrypted");
        }
    }

    Ok(())
}
