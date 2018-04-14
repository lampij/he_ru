#[macro_use]
extern crate serde_derive;

extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate wincolor;

use wincolor::{Color, Console, Intense};
use std::io;

mod hero;
use hero::hero_mod::*;

mod monster;
use monster::monster_mod::*;

fn main() {
    display_greeting();

    //TODO: Try loading a hero first, then if that fails, prompt to create a new one.
    let mut player_hero = load_hero();

    match player_hero.age {
        0 => player_hero = get_hero_details_from_user(),
        _ => println!("Welcome {}", player_hero.name),
    };

    save_hero(&player_hero);

    let playing: bool = true;
    while playing {
        let modifier = pick_training_spot();
        let fight: bool = true;

        while (fight) {
            let this_monster = monster_factory(&player_hero, modifier);
            println!(
                "{} {} {} {} {} {}",
                this_monster.health,
                this_monster.level,
                this_monster.strength,
                this_monster.luck,
                this_monster.dexterity,
                this_monster.intelligence
            );
        }
    }
}

pub fn display_greeting() {
    let mut con = Console::stdout().unwrap();
    con.fg(Intense::Yes, Color::Cyan).unwrap();
    println!();
    println!("Welcome to he_ru!");
    con.reset().unwrap();
    println!("This is an idle game for managing a hero.");
    println!();
}

pub fn pick_training_spot() -> u8 {
    println!();
    println!("Pick your training area;");
    println!("1. Easy");
    println!("2. Intermediate");
    println!("3. Advanced");

    let mut selection = String::new();

    io::stdin().read_line(&mut selection).unwrap();
    selection.trim().parse().unwrap()
}

pub fn fight(player: &mut Hero, monster: &mut Monster) {}
