#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate wincolor;

use wincolor::{Console, Color, Intense};

mod hero;
use hero::hero_mod::*;

fn main() {
    display_greeting();
    
    //TODO: Try loading a hero first, then if that fails, prompt to create a new one.
    let mut player_hero = load_hero();
    
    match player_hero.age {
        0 => player_hero = get_hero_details_from_user(),
        _ => println!("Welcome {}", player_hero.name),
    };

    save_hero(&player_hero);
}

pub fn display_greeting() {
    let mut con = Console::stdout().unwrap();
    con.fg(Intense::Yes, Color::Cyan).unwrap();
    println!();
    println!("Welcome to he_ru!");
    con.reset().unwrap();
    println!("This is an idle game for managing a hero.");
}
