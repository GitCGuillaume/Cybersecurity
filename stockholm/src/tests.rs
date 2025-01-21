#[cfg(test)]
mod tests {
    use crate::tools::get_flags;
    use crate::tools::Flags;
    use crate::tools::parse::find_reverse_key;
    use crate::encrypt;
    use std::fs::ReadDir;
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
        let dir = encrypt::get_infection_folder().unwrap();
        let ret: bool = encrypt::folder_exist(dir);

        assert_eq!(ret, true);
    }

    fn test_encrypt_rename() {

    }

    fn test_file_already_ft() {

    }
}
