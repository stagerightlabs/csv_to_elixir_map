extern crate rand;

use rand::Rng;
use rand::distributions::Alphanumeric;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::collections::HashMap;
use std::fs::File;
use std::process;
use std::io::prelude::*;
use std::vec::Vec;


type Record = HashMap<String, String>;

// This method represents the bulk of our program. It will
// return an error to main if something goes wrong.
fn run() -> Result<(), Box<dyn Error>> {

    // Prepare a string to serve as an output buffer of sorts
    let mut output : String = "[\n".to_string();

    // Read the CSV file
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;
    let headers = create_header_map(rdr.headers()?.clone());

    // Determine our output path
    let output_path = get_second_arg();
    let mut output_file = match File::create(&output_path) {
        Err(why) => panic!("couldn't create {}: {}", output_path, why),
        Ok(output_file) => output_file,
    };

    // Loop over each record and add it to our output file
    for result in rdr.deserialize() {
        let record: Record = result?;
        let mut row : String = "%{".to_string();

        // Loop over each key value pair in the record to assemble an elixir map
        for key in &headers {
            row.push_str(key);
            row.push_str(": ");

            match record.get(key) {
                Some(value) => row.push_str(maybe_encapsulate(value.to_string()).as_str()),
                None => row.push_str("\"\""),
            }

            row.push_str(", ");
        }

        // remove the trailing comma from our row
        row.pop();
        row.pop();

        // Close the map with a bracket and comma
        row = row + "},";

        // Add our row to the output string
        output.push_str("  ");
        output.push_str(row.as_str());
        output.push_str("\n");
    }

    // remove the trailing comma from our set of rows
    output.pop();
    output.pop();

    // Append our closing list bracket
    output.push_str("\n]");

    // Attempt to write the output content to the output file
    match output_file.write_all(output.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", output_path, why),
        Ok(_) => println!("successfully wrote to {}", output_path),
    }

    Ok(())
}

// Returns the first positional argument sent to this process
// If there are none, it returns an error
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("Expected 1 argument but got none")),
        Some(file_path) => Ok(file_path)
    }
}

// Determine whether the users has provided a name for the
// output file. If not we will generate a random one.
fn get_second_arg() -> String {
    match env::args_os().nth(2) {
        None => random_ascii_string(10).to_string() + &".txt".to_string(),
        Some(file_path) => file_path.into_string().unwrap()
    }
}

// Generate a random ascii string
// https://stackoverflow.com/a/54277357
fn random_ascii_string(len: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect();
    return s;
}

// If a value can be parsed as a floating point number we can
// assume that doesn't need to be encapsulated in the output.
// Otherwise we will surround it with quotes.
fn maybe_encapsulate(value: String) -> String {
    match value.parse::<f64>() {
        Ok(_) => value,
        Err(_) => {
            let mut ret = String::from("\"");

            ret.push_str(value.as_str());
            ret.push_str("\"");

            return ret;
        },
    }
}

fn create_header_map(headers: csv::StringRecord) -> Vec<String> {
    let mut map: Vec<String> = vec![];
    let mut n = 0;
    let mut done = false;

    while !done {
        match headers.get(n) {
            Some(value) => {
                map.push(value.to_string());
                n = n + 1;
            },
            None => {
                done = true;
            },
        }
    }

    return map;
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
