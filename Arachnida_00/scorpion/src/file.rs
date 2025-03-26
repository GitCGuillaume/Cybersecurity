pub mod utils_metadata;
pub mod utils_exif;

pub mod handle_file {
    use std::{fs::File,
        io::{
            Read,
            Seek
        }
    };
    use image;
    use std::io::BufReader;
    use exif::{
        Exif,
        Error
    };

    /*
     * Show type file from File
     */
    pub fn show_content_type(file: &File) -> bool {
        let mut vec: Vec<u8> = Vec::new();
        let mut clone_file: &File = file.to_owned();
        let res_size: Result<usize, std::io::Error> = clone_file.read_to_end(&mut vec);
        let res_rewind = clone_file.rewind();

        if let Err(err) = res_rewind {
            eprintln!("Couldn't rewind file buffer: {err}");
            return false;
        }
        match res_size {
            Ok(size) => {
                if 0 < size {
                    let content_type = infer::get(&vec);

                    match content_type {
                        Some(mime) => {
                             println!("Content-type/Mime: {}", mime.mime_type());
                        },
                        None => {
                         eprintln!("No Mime/Content-type");
                        },
                    }
                } else {
                    eprintln!("File has not bytes, couldn't show content-type.");
                }
            },
            Err(err) => {
                eprintln!("Coudln't read file: {err}");
            }
        }
        true
    }

    pub fn show_image_dimension(path: &String) {
        let res_dimensions:Result<(u32, u32), image::ImageError>   = image::image_dimensions(path);

        match res_dimensions {
            Ok(dim) => {
                println!("Width: {0} Height: {1}", dim.0, dim.1);
            },
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }

    pub fn get_metadata(file: &File) -> Result<std::fs::Metadata, std::io::Error> {
        let meta: Result<std::fs::Metadata, std::io::Error> = file.metadata();

        meta
    }

    pub fn get_exif(file: &File) -> Result<Exif, Error> {
        let bytes: &mut BufReader<&File>
                                    = &mut BufReader::new(file);
        let exif: Result<Exif, Error>
                                    = exif::Reader::new().read_from_container(bytes);

        exif
    }

    pub fn get_file(path: &String) -> Result<File, std::io::Error> {
        let file: Result<File, std::io::Error> = File::open(path);

        file
    }
}