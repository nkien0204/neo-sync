use std::{
    fs::{self, File},
    io::Write,
};

pub fn create_ns_home(dir_path: &str) -> std::io::Result<()> {
    if let Err(_) = fs::metadata(&dir_path) {
        // Create the folder if it does not exist
        fs::create_dir_all(&dir_path)?;
        println!("{}, folder created.", dir_path);
    }
    Ok(())
}

pub fn save_to_file(filename: &str, gist_id: String) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(gist_id.as_bytes())?;
    Ok(())
}
