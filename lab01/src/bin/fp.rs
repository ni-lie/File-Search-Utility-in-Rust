// module for reading command line arguments and opening files
use std::env;
use std::fs;
//use std::path::PathBuf;

// module for querying filesystem for all files
use glob::glob;

use regex::Regex;



fn main(){
    let args: Vec<String> = env::args().collect();
    
    let glob_path = &args[1];
    let reg_string = &args[2];

    let global = glob(glob_path).expect("Invalid glob path");
    let re = Regex::new(reg_string).expect("Invalid regex string"); 

    // let global1 = global.map(|path| path);
    // println!("{:?}", global1);

    let vec_path: Vec<_> = global
        .map(|path| path.unwrap())
        .collect();

    // println!("{:?}", vecPath);

    let filtered_vec_path:Vec<_> = vec_path
        .into_iter()
        .filter(|path| re.is_match(&fs::read_to_string(path.clone().into_os_string().into_string().unwrap()).unwrap()) == true)
        .collect();

    filtered_vec_path
    .into_iter()
    .for_each(|x| println!("{:?}", x.display()));
    
    // println!("{:?}", filtered_vecPath);
}
