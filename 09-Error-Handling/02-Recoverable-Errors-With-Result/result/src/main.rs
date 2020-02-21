use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: \n{:#?}", e),
            },
            other_error => panic!("Problem opening the file: \n{:#?}", other_error),
        },
    };
    let f = File::open("hello.txt").unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => File::create("hello.txt").unwrap_or_else(|error| {
            panic!("Problem creating the file: \n{:#?}", error);
        }),
        other_error => {
            panic!("Problem opening the file: \n{:#?}", other_error);
        }
    });
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // let f = File::open("hello01.txt").unwrap();
    let f = File::open("hello01.txt").expect("Failed to open hello01.txt");

    let f = File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn even_shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn unbelievably_short_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
