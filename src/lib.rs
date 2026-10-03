use std::{fs::{self, File}, io::{self, BufReader, BufWriter, ErrorKind, Read}, iter, path::{Path, PathBuf}, str::FromStr};

use age::{Decryptor, Encryptor, secrecy::SecretString, x25519};
use dialoguer::{Confirm, Password};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tar::{Archive, Builder};
use walkdir::WalkDir;

use crate::ui::{DecryptFilterArgs, EncryptFilterArgs};

pub mod ui;

fn confirm_process(dirs: &Vec<PathBuf>) -> io::Result<()>{
    println!("Preparing process {} folder", dirs.len());
    println!("Directory: {}, and {} others", dirs[0].display(), dirs.len() -1 );

    let confirm = Confirm::new()
        .with_prompt("Do you want process these files?")
        .default(true)
        .interact()
        .unwrap();

    if !confirm{
        return Err(io::Error::new(
            ErrorKind::InvalidInput, 
            "Process cancelled"
        ));
    }

    Ok(())
}

fn hex_filename() -> String{
    let random = rand::random::<[u8; 16]>();
    return hex::encode(random);
}

pub fn encrypt(options: EncryptFilterArgs) -> io::Result<()>{
    let path = options.path;
    let target = Path::new(&path);

    if !target.exists(){
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "Path doesn't exists"
        ));
    };

    let dirs: Vec<PathBuf> =  WalkDir::new(target)
        .max_depth(1) 
        .min_depth(1)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| f.file_type().is_dir())
        .filter(|f|{
            // filter if there include/exclude path
            if let Some(dir_name) = f.file_name().to_str(){
                if !options.exclude.is_empty() && options.exclude.iter().any(|x| x == dir_name){
                    return false;
                }

                if !options.include.is_empty() && !options.include.iter().any(|x| x == dir_name){
                    return false;
                }
            }
            true
        })
        .map(|f| f.path().to_path_buf())
        .collect();

    confirm_process(&dirs)?;

    let passphrase: Option<String> = if options.recipient.is_empty(){
        let p = Password::new()
            .with_prompt("Passphrase")
            .interact()
            .unwrap();
        let confirm_p = Password::new()
            .with_prompt("Confirm Passphrase")
            .interact()
            .unwrap();

        if p != confirm_p {
            return Err(io::Error::new(
                ErrorKind::InvalidInput, 
                "Passphrase is not match"
            ));
        }

        Some(p)
    }else{
        None
    };

    let mp = MultiProgress::new();
    let main_pb = mp.add(ProgressBar::new(dirs.len() as u64));
    main_pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40}] {pos}/{len} folder ({percent}%) | Processsing: {msg}")
        .unwrap()
        .progress_chars("██░"));

    for entry in main_pb.wrap_iter(dirs.iter()){
        let last_path = entry.file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("Unknown");
        main_pb.set_message(last_path.to_string());

        // check if filenames use hex filename or not
        let filename = options.hex
            .then(||{
                let o_path = Path::new(target);
                o_path.join(hex_filename()).display().to_string()
            })
            .unwrap_or_else(|| entry.display().to_string());

        let output_path = format!("{}.tar.age", filename);

        // dry-run
        if options.dry_run{
            main_pb.println(format!("{} to {}", entry.display(), output_path));
            continue;
        }
        
        let output_file = File::create(output_path)?;
        let buffered_writer = BufWriter::new(output_file);

        // define encryptor
        let encryptor = if options.recipient.is_empty() {
            // passphrase
            if let Some(pass) = passphrase.clone() {
                Encryptor::with_user_passphrase(age::secrecy::SecretString::new(pass.into()))
            }else{
                return Err(io::Error::new(ErrorKind::Other, "Please insert passphrase"));
            }
        }else{
            let mut file = File::open(&options.recipient)?;
            let mut key_content = String::new();
            file.read_to_string(&mut key_content)?;
            
            let recipient = x25519::Recipient::from_str(key_content.trim())
                .expect("Failed parse recepients");

            Encryptor::with_recipients(iter::once(&recipient as _))
                .expect("Failed initial encryptor")
        };
       
        let encrypt_stream = encryptor.wrap_output(buffered_writer)?;

        let Some(file_stem) = entry.file_stem().and_then(|x| x.to_str()) else {
            return Err(io::Error::new(
                ErrorKind::InvalidInput, 
                "filename or path is invalid"
            ));
        };

        // archive folder into tar file
        let mut archive = Builder::new(encrypt_stream);
        let entry_archive = entry.clone();
        archive.append_dir_all(file_stem, entry_archive)?;

        // encrypt tar file stream
        let encrypted_stream = archive.into_inner()?;
        encrypted_stream.finish()?;
        main_pb.println(format!("{:<12} {}", "Encrypted:", entry.display()));

        if options.clean{
            fs::remove_dir_all(entry.display().to_string())?;
            main_pb.println(format!("{:<12} {}\n", "Deleted:", entry.display()));
        }

        main_pb.println("  ---");
    }
    
    Ok(())
}

pub fn decrypt(options: DecryptFilterArgs) -> io::Result<()>{
    let path = options.path;
    let target = path.clone();
    let dirs: Vec<PathBuf> =  WalkDir::new(target)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        // filter with path is ok()
        .filter_map(|f| f.ok())
        // filter only file with .age extension
        .filter(|f| f.path().extension().is_some_and(|e| e == "age"))
        .filter(|f|{
            // filter if there include/exclude path
            if let Some(dir_name) = f.file_name().to_str(){
                if !options.exclude.is_empty() && options.exclude.iter().any(|x| x == dir_name){
                    return false;
                }

                if !options.include.is_empty() && !options.include.iter().any(|x| x == dir_name){
                    return false;
                }
            }
            true
        })
        .map(|f| f.path().to_path_buf())
        .collect();

    confirm_process(&dirs)?;

    let passphrase: Option<String> = if options.identity.is_empty(){
        let pass = Password::new()
            .with_prompt("Passphrase")
            .interact()
            .unwrap();
        Some(pass)
    }else{
        None
    };

    let mp = MultiProgress::new();
    let main_pb = mp.add(ProgressBar::new(dirs.len() as u64));
    main_pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40}] {pos}/{len} folder ({percent}%) | Processsing: {msg}")
        .unwrap()
        .progress_chars("██░"));

    for entry in main_pb.wrap_iter(dirs.iter()){
        let last_path = entry.file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("Unknown");
        main_pb.set_message(last_path.to_string());

        if entry.extension().is_some_and(|e| e == "age"){
            let entry_file = entry.clone();
            let input_file = File::open(entry_file)?;
            let buffered_reader = BufReader::new(input_file);
            
            let decryptor = Decryptor::new(buffered_reader)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            let decrypted_stream = if options.identity.is_empty(){
                if let Some(pass) = passphrase.clone(){
                    let identity = age::scrypt::Identity::new(
                        SecretString::from(pass)
                    );

                    let stream = decryptor.decrypt(
                        std::iter::once(&identity as &dyn age::Identity)
                    )
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                    stream
                }else{
                    return Err(io::Error::new(ErrorKind::Other, "Please insert passphrase"));
                }
            }else{
                let identity_file = age::IdentityFile::from_file(options.identity.clone())
                    .map_err(|_| io::Error::new(ErrorKind::InvalidInput, "Failed parse identity"))?;
                let identities = identity_file
                    .into_identities()
                    .map_err(|e| io::Error::new(ErrorKind::Other, format!("Failed read identity: {}", e)))?;

                let stream = decryptor
                    .decrypt(identities.iter().map(|i| {
                        let identity: &dyn age::Identity = i.as_ref();
                        identity
                    }))
                    .map_err(|e| io::Error::new(ErrorKind::Other, format!("Error: {}", e)))?;
                stream
            };

            let mut archive = Archive::new(decrypted_stream);

            archive.unpack(&path)?;
            main_pb.println(format!("{:<12} {}", "Decrypted:", entry.display()));

            if options.clean{
                fs::remove_file(entry.display().to_string())?;
                main_pb.println(format!("{:<12} {}", "Deleted:", entry.display()));
            }
            main_pb.println("  ---");
        }
    }

    Ok(())
}
