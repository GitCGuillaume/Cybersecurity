#[cfg(test)]
mod tests {
    use crate::tools::Flags;
    use crate::parse::find_flags;

    #[test]
    fn test_flags_r() {
        let mut s_flags = Flags {
            key: String::default(),
            reverse_key: String::default(),
            help: false,
            version: false,
            silent: false
        };

        find_flags(&String::from("-r abc"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "abc");
        s_flags.reverse_key.clear();
        find_flags(&String::from("-sr abc"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "abc");
        s_flags.reverse_key.clear();
        find_flags(&String::from("-rabc"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "abc");
        find_flags(&String::from("-srabc"), &mut s_flags);
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

        find_flags(&String::from("--reverse abc"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "abc");
        s_flags.reverse_key.clear();
        find_flags(&String::from("-s --reverse abc"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "abc");
        s_flags.reverse_key.clear();
        find_flags(&String::from("-s --reverseabc"), &mut s_flags);
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

        find_flags(&String::from("--reverse abc -sr def"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "def");
        find_flags(&String::from("--reverse abc -r def"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "def");
        s_flags.reverse_key.clear();
        find_flags(&String::from("-sr abc --reverse def"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "def");
        find_flags(&String::from("-r abc --reverse def"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "def");
        s_flags.reverse_key.clear();
        find_flags(&String::from("-rabc --reversedef"), &mut s_flags);
        assert_eq!(s_flags.reverse_key, "def");
    }
}
