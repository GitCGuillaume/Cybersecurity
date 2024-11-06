pub mod utils_metadata;
pub mod utils_exif;

pub mod handle_file {
    use std::fs::File;

    pub fn get_metadata(file: &File) -> Result<std::fs::Metadata, std::io::Error> {
        let meta: Result<std::fs::Metadata, std::io::Error> = file.metadata();

        meta
    }

    pub fn get_exif(file: &File) -> Result<exif::Exif, exif::Error> {
        let bytes: &mut std::io::BufReader<&File>
                                                = &mut std::io::BufReader::new(file);
        let exif: Result<exif::Exif, exif::Error>
                                                = exif::Reader::new().read_from_container(bytes);
    
        exif
    }

    pub fn get_file(path: &String) -> Result<File, std::io::Error> {
        let file: Result<File, std::io::Error> = File::open(path);

        file
    }
}