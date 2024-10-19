/*fn  parse_url(value: String) {

}*/

fn  parse_flags(_options: &mut [i8; 256], value: String) {
    let mut it = value.chars();
    let mut idx: Option<usize> = None;
    if value.len() == 1 {
        eprintln!("Please provide value after - [./spider [-rlp] URL]");
        std::process::exit(1);
    }
    it.next();
    for i in it {
        match i {
            'r' => {
                _options[114] = 1;
                println!("r updated");
            },
            'l' => {
                _options[108] = 1;
                println!("l");
                idx = value.find("l ");
                break ;
            },
            'p' => {
                _options[112] = 1;
                println!("p");
                idx = value.find("p ");
                break ;
            },
            _ => {
                eprintln!("Please provide correct options values [./spider [-rlp] URL]");
                std::process::exit(1);
            }
        }
    }
    dbg!(idx);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut _options: [i8; 256] = [-1; 256];

    for i in args {
        if i.find('-') == Some(0) {
            parse_flags(&mut _options, i);
        }
    }
    for i in _options {
        print!("{i} ");
    }
}
