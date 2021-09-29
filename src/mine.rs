use std::{fs, io::Write, path::Path};

use rand::Rng;

use crate::balance::{add_balance, get_balance, remove_balance};
use crate::commands::sleep;

pub fn buy_pico() {
    let amount = get_balance();
    if amount >= 5000 {
        if Path::new("objects/pico").exists() == false {
            add_pico().unwrap();
            println!("Compraste un pico");
            remove_balance(5000);
        } else {
            println!("Ya tienes un pico");
        }
    } else {
        println!("Dinero Insuficiente");
    }
}

pub fn add_pico() -> std::io::Result<()> {
    let mut bal = fs::File::create("objects/pico")?;
    bal.write_all(b"1")?;
    Ok(())
}

pub fn get_pico_lvl() -> u32 {
    let dir = "objects/pico";
    let pico = fs::read_to_string(dir).expect("Error al abrir el nivel del pico");
    let lvl = pico.parse::<u32>().unwrap();

    return lvl;
}

pub fn upgrade_pico_lvl() {
    let lvl = get_pico_lvl();
    let dir = "objects/pico";
    let result = lvl + 1;
    let lvlstr = result.to_string();

    fs::write(dir, lvlstr).expect("Error al escribir el nivel");
}

pub fn mine() {
    if Path::new("objects/pico").exists() == true {
        let lvl = get_pico_lvl();
        let mut rng = rand::thread_rng();
        let min = lvl * 500;
        let max = lvl * 1000;
        let amount = rng.gen_range(min..max);

        add_balance(amount);
        println!("Minaste con tu pico nivel {} y ganaste ${}", lvl, amount);
        sleep();
    } else {
        println!("No posees un pico");
        sleep();
    }
}

pub fn price_pico() -> String {
    if Path::new("objects/pico").exists() == true {
        let lvl = get_pico_lvl();
        let price = lvl * 5000;
        let amountstr = price.to_string();
        let mut pricestr: String = "$".to_owned();
        let amount: &str = &amountstr;
        pricestr.push_str(amount);
        return pricestr.to_string();
    } else {
        return "No tienes un pico".to_string();
    }
}

pub fn upgrade_pico() {
    if Path::new("objects/pico").exists() == true {
        let amount = get_balance();
        let lvl = get_pico_lvl();
        let price = lvl * 5000;
        if amount >= price {
            upgrade_pico_lvl();
            println!("Mejoraste un pico");
            remove_balance(price);
        } else {
            println!("Dinero Insuficiente");
        }
    } else {
        println!("No tienes un pico");
    }
}
