use std::{env,fs,path::Path};

pub fn load_file_from_args()-> Result<String,String>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        return Err("Please provide a file".to_string());
    }

    let path = Path::new(&args[1]);
    if !path.exists() || !path.is_file(){

        return Err("Ïnvalid file path".to_string());
    }
    match fs::read_to_string(path){
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Failed to read file: {}",e)),
    }
}
