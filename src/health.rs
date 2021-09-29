use std::{fs, io::Write, path::Path};

pub fn check_hp() {
    if Path::new("objects/health").exists() == false {
        create_hp().unwrap();
    }
}

pub fn create_hp() -> std::io::Result<()> {
    fs::create_dir_all("objects")?;
    let mut hp = fs::File::create("objects/health")?;
    hp.write_all(b"100")?;
    Ok(())
}

pub fn get_hp() -> i32 {
    let dir = "objects/health";
    let health = fs::read_to_string(dir).expect("Error al abrir la vida");
    let value = health.parse::<i32>().unwrap();

    return value;
}

pub fn add_health(amount: i32) {
    let dir = "objects/health";
    let health = fs::read_to_string(dir).expect("Error al abrir la salud");
    let value = health.parse::<i32>().unwrap();
    let result = value + amount;
    let hpstr = result.to_string();

    fs::write(dir, hpstr).expect("Error al escribir la salud");
}

pub fn remove_health(amount: i32) {
    let dir = "objects/health";
    let health = fs::read_to_string(dir).expect("Error al abrir la salud");
    let value = health.parse::<i32>().unwrap();
    let result = value - amount;
    let hpstr = result.to_string();

    fs::write(dir, hpstr).expect("Error al escribir la salud");
}
