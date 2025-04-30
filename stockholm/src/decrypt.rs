pub mod desinfect;
use std::fs::{
    read_dir,
    ReadDir
};
use std::path::PathBuf;
use crate::tools::Flags;
use desinfect::{
    desinfect
};

pub fn get_infection_path(path: &String) -> Result<ReadDir, std::io::Error> {
    let env = env!("HOME");
    let mut path_user: String = String::from(path);
    path_user.insert_str(0, env);
    println!("Folder infection path : {}", path_user);
    return read_dir(path_user);
}
/* Check infection folder existence */
pub fn folder_exist(res_dir: &Result<ReadDir, std::io::Error>) -> bool {
    return match res_dir {
        Ok(_) => {
            true
        },
        Err(e) => {
            eprintln!("{e}");
            false
        }
    }
}

/*
 * Check extension .ft
 * then decrypt
 */
fn start_desinfect(dir: ReadDir, list: &Flags) {
    for i in dir {
         if let Ok(a) = i {
            let path: PathBuf = a.path();
            if let Some(extension) = path.extension() {
                if let Some(extension_str) = extension.to_str() {
                    if extension_str == "ft" {
                        if desinfect(list, &path) {
                            println!("Decrypted");
                        } else {
                            eprintln!("Decryption failed");
                        }
                    }
                }
            }
        }
    }
}

/*
 * Get infection folder from $HOME
 * Check folder
 * Start decrypt
 */
pub fn start_decrypt(list: &Flags) -> bool {
    let res_dir = get_infection_path(&"/infection".to_owned());
    let ret: bool = folder_exist(&res_dir);

    if !ret {
        return ret;
    }
    return match res_dir {
        Ok(dir) => {
            start_desinfect(dir, list);
            true
        },
        Err(e) => {
            eprintln!("{e}");
            false
        }
    }
}
