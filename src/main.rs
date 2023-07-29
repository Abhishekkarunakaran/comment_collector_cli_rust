// use std::fs;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
mod data;

fn main() {

    let pattern = r"^\/\/.*";
    let regex = Regex::new(pattern).expect("Invalid regex pattern");
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];
    let file = File::open(file_path).expect("Error in file reading");
    let reader = io::BufReader::new(&file);

    let mut line_number = 1;

    for line in reader.lines(){
        if let Ok(ip) = line {
            if regex.is_match(&ip){
                println!("{}: {}",line_number,ip);
            }
            line_number += 1;
        }
    }

    
    // let contents= fs::read_file_lines(file_path).expect("Error");
    // print!("{:?}",file);

    drop(file);

    let data = crate::data::get_data();

    for datum in data{
        dbg!(datum.single);
    }
    // print!("{:#?}\n",data);

}
