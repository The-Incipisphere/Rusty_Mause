use std::collections::HashMap;
use std::io::*;

fn main() {
    println!("CSV file to read:");
    let mut name_of_file_to_be_read = String::new();
    match stdin().read_line(&mut name_of_file_to_be_read) {
        Ok(_) => {}
        Err(error) => {
            println!("Stdin Error: {}", error);
        }
    }
    read_csv(&name_of_file_to_be_read).expect("Failed to read file!");
}

fn read_csv(input: &str) -> Result<HashMap<String, String>> {
    return match std::fs::exists(input) {
        Ok(_) => {
            let mut map = HashMap::new();
            for line in std::fs::read_to_string(input)?.lines() {
                let split = line.split(',').collect::<Vec<&str>>();
                map.insert(split[0].to_string(), split[1].to_string());
            }
            Ok(map)
        },
        Err(_) => {
            Err(Error::new(ErrorKind::NotFound, "File not Found!"))
        },
    }
}

/*
fn encode_morse(input: &str) -> String {
   return REPLACEME;
}

fn decode_morse(input: &str) -> String {
    return REPLACEME;
}
*/