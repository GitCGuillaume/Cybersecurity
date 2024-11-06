use std::any::Any;
use std::fs::Metadata;
use std::time::{
    SystemTime,
    Duration,
    SystemTimeError,
    UNIX_EPOCH
};
use chrono::{
    DateTime,
    Local,
    Utc
};

fn show_content_type(metadata: &Metadata) {
    let content = metadata.file_type();

    dbg!(content);
}

fn show_len(metadata: &Metadata) {
    let value: u64 = metadata.len();

    println!("File length: {value} bytes");
}

fn show_permissions(metadata: &Metadata) {
    let value = metadata.permissions();

    if value.readonly() {
        println!("File is read only.");
    } else {
        println!("Read and write access.");
    }
}

fn show_datetime(duration: &Duration) {
    let seconds: i64 = duration.as_secs() as i64;
    let timestamp: Option<DateTime<Utc>> = DateTime::from_timestamp(seconds, 0);

    match timestamp {
        Some(ts) => {
            let convert_tz: DateTime<Local> = ts.with_timezone(&Local);
        
            println!("{}", convert_tz);
        },
        None => {},
    }
}

/* 
 For Windows and Unix
 */
fn show_timer(timer: Result<SystemTime, std::io::Error>, _str: &str) {
    match timer {
        Ok(val) => {
            let years: Result<Duration, SystemTimeError> = val.duration_since(UNIX_EPOCH);
            
            match years {
                Ok(y) => {
                    print!("{}", _str);
                    show_datetime(&y);
                },
                Err(_) => {},
            }
        },
        Err(e) => {
            println!("Couldn't access last modified time");
            eprintln!("{e}");
        },
    }
}

pub fn show_metadata(metadata: &Result<Metadata, std::io::Error>) {
    match metadata {
        Ok(res) => {
            show_content_type(res);
            show_len(res);
            show_permissions(res);
            show_timer(res.modified(), "Last time modified: ");
            show_timer(res.accessed(), "Last time accessed: ");
            show_timer(res.created(), "Time creation: ");
        },
        Err(e) => {
            eprintln!("Error: {e}");
        },
    }
}