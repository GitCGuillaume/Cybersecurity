use std::{ env, fs::File };
use exif::Exif;
use file::{handle_file::{
    show_content_type,
    show_image_dimension,
    get_file,
    get_exif,
    get_metadata,
},
    utils_metadata,
    utils_exif
};
mod file;

/*
    To generate png exif: https://products.groupdocs.app/metadata/png
    Display jpg / png exif
    Display metadata for jpg/png/bmp/gif
*/
fn main() {
    let argv: Vec<String> = env::args().skip(1).collect();

    if argv.len() == 0 {
        eprintln!("Program need a path.");
        return ;
    }
    for i in argv {
        let res_file: Result<File, std::io::Error> = get_file(&i);

        match res_file {
            Ok(file) => {
                show_content_type(&i);
                show_image_dimension(&i);
                let res_metadata: Result<std::fs::Metadata, std::io::Error> = get_metadata(&file);

                utils_metadata::show_metadata(&res_metadata);
                let res_exif: Result<Exif, exif::Error> = get_exif(&file);

                utils_exif::show_exif(&res_exif);
                println!("------");
            },
            Err(e) => {
                eprintln!("Error: {e}");
            },
        }
    }
}
