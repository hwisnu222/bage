use std::{fs::File, io::{self, BufReader, BufWriter, ErrorKind}, path::{Path, PathBuf}};

use age::{Decryptor, Encryptor, secrecy::SecretString};
use dialoguer::{Confirm, Password};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tar::{Archive, Builder};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Options{
    pub include: Vec<String>,
    pub exclude: Vec<String>
}

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

    println!("Processsing directories...");
    Ok(())
}

pub fn encrypt(path: String, options: Options) -> io::Result<()>{
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

    let passphrase = Password::new()
        .with_prompt("Passphrase")
        .interact()
        .unwrap();
    let confirm_passphrase = Password::new()
        .with_prompt("Confirm Passphrase")
        .interact()
        .unwrap();

    if passphrase != confirm_passphrase{
        return Err(io::Error::new(
            ErrorKind::InvalidInput, 
            "Passphrase is not match"
        ));
    }

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

        let passphrase_file = passphrase.clone();
        let output_path = format!("{}.tar.age", entry.display().to_string());
        let output_file = File::create(output_path)?;
        
        let buffered_writer = BufWriter::new(output_file);

        let encryptor = Encryptor::with_user_passphrase(age::secrecy::SecretString::new(passphrase_file.into()));
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
    }
    
    Ok(())
}

pub fn decrypt(path: String, options: Options) -> io::Result<()>{
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

    let passphrase = Password::new()
        .with_prompt("Passphrase")
        .interact()
        .unwrap();

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

        let passphrase_file = passphrase.clone();

        if entry.extension().is_some_and(|e| e == "age"){
            let entry_file = entry.clone();
            let input_file = File::open(entry_file)?;
            let buffered_reader = BufReader::new(input_file);
            
            let decryptor = Decryptor::new(buffered_reader)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            let identity = age::scrypt::Identity::new(
                SecretString::from(passphrase_file)
            );
            let decrypted_stream = decryptor.decrypt(
                std::iter::once(&identity as &dyn age::Identity)
            )
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            let mut archive = Archive::new(decrypted_stream);

            archive.unpack(&path)?;
        }
    }

    Ok(())
}
