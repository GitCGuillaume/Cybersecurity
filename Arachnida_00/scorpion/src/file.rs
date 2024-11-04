pub mod handle_file {
    use std::fs::File;

    /*pub fn get_exif(bytes: &Vec<u8>) -> Result<exif::Exif, exif::Error> {
        let exif_raw: Result<exif::Exif, exif::Error> = exif::Reader::new().read_raw(bytes.to_vec());

        exif_raw
    }
    pub fn get_file(path: &String) -> Result<Vec<u8>, std::io::Error> {
        let exif: Result<Vec<u8>, std::io::Error> = fs::read(path);

        exif
    }*/
    pub fn get_exif(file: &File) -> Result<exif::Exif, exif::Error> {
        let exif: Result<exif::Exif, exif::Error> = exif::Reader::new().read_from_container(&mut std::io::BufReader::new(file));
    
        exif
    }

    pub fn get_file(path: &String) -> Result<File, std::io::Error> {
        let file: Result<File, std::io::Error> = File::open(path);

        file
    }
}