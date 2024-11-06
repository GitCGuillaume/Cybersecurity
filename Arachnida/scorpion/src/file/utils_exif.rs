use exif::{
    Exif,
    Error
};

pub fn show_exif(exif: &Result<Exif, Error>) {
    match exif {
        Ok(ex) => {
            let it = ex.fields();

            for i in it {
                println!("{0}: {1}", i.tag, i.display_value().with_unit(()));
            }
        },
        Err(e) => {
            eprintln!("Exif error: {e}");
        },
    }
}