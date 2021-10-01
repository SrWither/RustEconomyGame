use std::{fs, io::Write, path::Path};

pub fn check_enemy() {
    if Path::new(".objects/enemy").exists() == false {
        create_enemy().unwrap();
    }
}

pub fn create_enemy() -> std::io::Result<()> {
    fs::create_dir_all(".objects")?;
    let mut hp = fs::File::create(".objects/enemy")?;
    hp.write_all(b"100")?;
    Ok(())
}

pub fn get_enemyhp() -> i32 {
    let dir = ".objects/enemy";
    let health = fs::read_to_string(dir).expect("Error al abrir la vida");
    let value = health.parse::<i32>().unwrap();

    return value;
}

pub fn add_enemyhp(amount: i32) {
    let dir = ".objects/enemy";
    let enemyhp = fs::read_to_string(dir).expect("Error al abrir la salud del enemigo");
    let value = enemyhp.parse::<i32>().unwrap();
    let result = value + amount;
    let hpstr = result.to_string();

    fs::write(dir, hpstr).expect("Error al escribir la salud del enemigo");
}

pub fn remove_enemyhp(amount: i32) {
    let dir = ".objects/enemy";
    let enemyhp = fs::read_to_string(dir).expect("Error al abrir la salud del enemigo");
    let value = enemyhp.parse::<i32>().unwrap();
    let result = value - amount;
    let hpstr = result.to_string();

    fs::write(dir, hpstr).expect("Error al escribir la salud del enemigo");
}
