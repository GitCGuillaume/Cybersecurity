use std::{ env, fs::File };
use file::handle_file::{ get_file, get_exif };

mod file;
mod metadata;

/*
    Display jpg / png exif
    Display metadata for jpg/png/bmp/gif
*/
fn main() {
    let argv: std::iter::Skip<env::Args> = env::args().skip(2);

    for i in argv {
        let res_bytes = get_file(&i);

        match res_bytes {
            Ok(bytes) => {
                let res_exif = get_exif(&bytes);

                match res_exif {
                    Ok(ex) => {
                        for i in ex.fields() {
                            dbg!(i);
                        }
                    },
                    Err(e) => {
                        eprintln!("Error: {e}");
                    },
                }
            },
            Err(e) => {
                eprintln!("Error: {e}");
            },
        }
    }
}
