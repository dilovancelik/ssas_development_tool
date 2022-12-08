use std::io::Read;
use std::fs::File;
use std::env;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let data = read_file(file_path);

    typed_example(data)
        .map_err(|err| println!("{:?}", err))
        .ok();
}

fn read_file(file_path: &String) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents
}

fn typed_example(data: String) -> Result<()> {
    let mut p: Person = serde_json::from_str(data.as_str())?;
    p.name = "Dilovan".to_string();
    println!("Please call {} at the number {}", p.name, p.phones[1]);

    Ok(())
}
