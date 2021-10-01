use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn check_balance() {
    if Path::new(".objects/balance.txt").exists() == false {
        create_balance().unwrap();
    }
}

pub fn create_balance() -> std::io::Result<()> {
    fs::create_dir_all(".objects")?;
    fs::create_dir_all(".objects/items")?;
    let mut bal = fs::File::create(".objects/balance.txt")?;
    bal.write_all(b"500")?;
    Ok(())
}

pub fn get_balance() -> u32 {
    let dir = ".objects/balance.txt";
    let balance = fs::read_to_string(dir).expect("Error al abrir el balance");
    let value = balance.parse::<u32>().unwrap();

    return value;
}

pub fn add_balance(amount: u32) {
    let dir = ".objects/balance.txt";
    let balance = fs::read_to_string(dir).expect("Error al abrir el balance");
    let value = balance.parse::<u32>().unwrap();
    let result = value + amount;
    let balstr = result.to_string();

    fs::write(dir, balstr).expect("Error al escribir el balance");
}

pub fn remove_balance(amount: u32) {
    let dir = ".objects/balance.txt";
    let balance = fs::read_to_string(dir).expect("Error al abrir el balance");
    let value = balance.parse::<u32>().unwrap();
    let result = value - amount;
    let balstr = result.to_string();

    fs::write(dir, balstr).expect("Error al escribir el balance");
}
