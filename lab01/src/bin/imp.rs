// module for reading command line arguments and opening files
use std::env;
use std::fs;

// module for querying filesystem for all files
use glob::glob;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vec = Vec::new();

    // &args[1] = glob path - "**/*.txt"
    // &args[2] = regex string - "\d+"
    let glob_path = &args[1];               
    let reg_string = &args[2];
    
    for entry in glob(glob_path).expect("Invalid glob path") {
        if let Ok(path) = entry {
            let contents = fs::read_to_string(&path)    // path.clone() if does not work
                .expect("Should have been able to read the file");
            

            let re = Regex::new(reg_string).expect("Invalid regex string");    
            
            //re = Regex::new(reg_string).unwrap();
            
            if re.is_match(&contents) == true {
                //println!("{:?}", path.display());
                vec.push(path.clone());
            }
        }
    }
    vec.sort();
    for x in &vec {
        println!("{}", x.display());
    }
}
