use std::{ env, fs::File };
use file::{handle_file::{
    get_file,
    get_exif,
    get_metadata,
},
    utils_metadata
};
mod file;

/*
    Display jpg / png exif
    Display metadata for jpg/png/bmp/gif
*/
fn main() {
    let argv: std::iter::Skip<env::Args> = env::args().skip(2);

    for i in argv {
        let res_file: Result<File, std::io::Error> = get_file(&i);

        match res_file {
            Ok(file) => {
                let res_metadata: Result<std::fs::Metadata, std::io::Error> = get_metadata(&file);
                dbg!(res_metadata);
                //utils_metadata::show_metadata(&res_metadata);
                //should I check content-type's file
                //let res_exif: Result<exif::Exif, exif::Error> = get_exif(&file);
                //utils_metadata::show_exif(&res_exif);
            },
            Err(e) => {
                eprintln!("Error: {e}");
            },
        }
    }
}
