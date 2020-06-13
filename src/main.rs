extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("Expected one argument, but got none.")),
        Some(file_path) => Ok(file_path),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .from_path(path)
        .unwrap();
    
    let head = rdr.headers()?;
    let len = &head.len();
    let head2 = head.clone();
    let mut num_entries = 0;
    for result in rdr.records() {
        let record = result?;
        //println!("{:?}", record);
        for i in 1..*len {
            if &record[i] == "" {
                continue
            }
            //                       elev, rain, temp
            println!("VoronoiInfo::new({}, {}, {}),", &record[i], &record[0], &head2[i]);
            num_entries += 1;
        }
    }
    println!("num of entries = {}", num_entries);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
