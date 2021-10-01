use std::{fs, io::Write, path::Path};

use crate::balance::{get_balance, remove_balance};

pub fn check_hp() {
    if Path::new(".objects/health").exists() == false {
        create_hp().unwrap();
    }
}

pub fn check_gapple() {
    if Path::new(".objects/items/gapple").exists() == false {
        create_gapple().unwrap();
    }
}

pub fn create_hp() -> std::io::Result<()> {
    fs::create_dir_all(".objects")?;
    let mut hp = fs::File::create(".objects/health")?;
    hp.write_all(b"100")?;
    Ok(())
}

pub fn get_hp() -> i32 {
    let dir = ".objects/health";
    let health = fs::read_to_string(dir).expect("Error al abrir la vida");
    let value = health.parse::<i32>().unwrap();

    return value;
}

pub fn add_health(amount: i32) {
    let dir = ".objects/health";
    let health = fs::read_to_string(dir).expect("Error al abrir la salud");
    let value = health.parse::<i32>().unwrap();
    let result = value + amount;
    let hpstr = result.to_string();

    fs::write(dir, hpstr).expect("Error al escribir la salud");
}

pub fn remove_health(amount: i32) {
    let dir = ".objects/health";
    let health = fs::read_to_string(dir).expect("Error al abrir la salud");
    let value = health.parse::<i32>().unwrap();
    let result = value - amount;
    let hpstr = result.to_string();

    fs::write(dir, hpstr).expect("Error al escribir la salud");
}

pub fn set_hp(amount: i32) {
    let dir = ".objects/health";
    let result = amount;
    let hpstr = result.to_string();

    fs::write(dir, hpstr).expect("Error al escribir la salud");
}

pub fn get_gapple() -> i32 {
    let dir = ".objects/items/gapple";
    let health = fs::read_to_string(dir).expect("Error al abrir la gapple");
    let value = health.parse::<i32>().unwrap();

    return value;
}

pub fn set_gp(amount: i32) {
    let dir = ".objects/items/gapple";
    let result = amount;
    let hpstr = result.to_string();

    fs::write(dir, hpstr).expect("Error al escribir la gapple");
}

pub fn remove_gapple() {
    let dir = ".objects/items/gapple";
    let gapp = fs::read_to_string(dir).expect("Error al abrir la gapple");
    let value = gapp.parse::<i32>().unwrap();
    let result = value - 1;
    let hpstr = result.to_string();

    fs::write(dir, hpstr).expect("Error al escribir la gapple");
}

pub fn add_gapple() {
    let dir = ".objects/items/gapple";
    let gapp = fs::read_to_string(dir).expect("Error al abrir la gapple");
    let value = gapp.parse::<i32>().unwrap();
    let result = value + 1;
    let hpstr = result.to_string();

    fs::write(dir, hpstr).expect("Error al escribir la gapple");
}

pub fn create_gapple() -> std::io::Result<()> {
    let mut bal = fs::File::create(".objects/items/gapple")?;
    bal.write_all(b"1")?;
    Ok(())
}

pub fn buy_gapple() {
    let amount = get_balance();
    if amount >= 1000 {
        if Path::new(".objects/items/gapple").exists() == false {
            create_gapple().unwrap();
            println!("Compraste una gapple");
            remove_balance(1000);
        } else {
            remove_balance(1000);
            add_gapple();
            println!("Compraste una gapple");
        }
    } else {
        println!("Dinero Insuficiente");
    }
}
