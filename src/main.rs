use std::collections::HashMap;
use std::io::*;

fn main() {
    let mut csv_file_name = String::new();
    let csv_file_name = standard_in("CSV file to read:",  &mut csv_file_name);

    println!("Trying to read {:?}", csv_file_name);
    let morse_code_dict = read_csv(&csv_file_name).expect("ERROR: CSV read failed!");

    let mut input_string = String::new();
    let input_string = standard_in("String to decode/encode:", &mut input_string);

    let mut decode_or_encode = String::new();
    let decode_or_encode = standard_in("Decode Or Encode? (dec/enc):", &mut decode_or_encode);
    

    if decode_or_encode.to_lowercase() == "dec" {
        println!("{}", decode_morse(&input_string, &morse_code_dict)); // not populated yet
    } else if decode_or_encode.to_lowercase() == "enc" {
        println!("{}", encode_morse(&input_string, &morse_code_dict)); // not populated yet
    } else {
        println!("ERROR: Invalid input!");
    }
}

fn read_csv(input: &str) -> Result<HashMap<String, String>> {
    return match std::fs::exists(input) { // Sanity check.
        Ok(_) => {
            let mut morse_data = HashMap::new(); //?: Init the hashmap to hold all the CSV data for the moment.
            for line in std::fs::read_to_string(input)?.lines() {
                let split = line
                    .split(',') // Split each line...
                    .collect::<Vec<&str>>(); // ...and put them into a Vec...
                morse_data.insert(split[0].to_string(), split[1].to_string()); // ...and insert both values of the Vec into an entry of the HashMap.
            }
            Ok(morse_data)
        },
        Err(_) => {
            Err(Error::new(ErrorKind::NotFound, "File Not Found!"))
        },
    }
}

fn standard_in(msg: &str, mut var: &mut String) -> String {
    println!("{}",&msg);
    stdin().read_line(&mut var).expect("ERROR: StdIn failed!");
    println!("Sanity check 1: {:?}",&var);
    let var = var.trim_end().to_string(); //?: get rid of the pesky \r\n
    println!("Sanity check 2: {:?}",&var);
    return var
}

fn encode_morse(input: &str, dict: &HashMap<String, String>) -> String {
    let mut output = String::new();
    for char in input.chars() { //?: Iterate through all the characters of the input string.
        if char.is_whitespace() {
            output.push_str("/ ")
        } else {
            match dict.get(&char.to_uppercase().to_string()) {
                None => output.push_str("# "),
                Some(value) => {
                    output.push_str([value, " "].concat().as_str());
                }
            }
        }
    }
    return output; // todo: populate
}

fn decode_morse(input: &str, dict: &HashMap<String, String>) -> String {
    return "todo".to_string(); // todo: populate
}

