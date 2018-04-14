#[macro_use]
extern crate serde_derive;

extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate wincolor;

use wincolor::{Color, Console, Intense};
use std::io;
use std::{thread, time};
use rand::{thread_rng, Rng};

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
        let fighting: bool = true;

        while (fighting) {
            let mut this_monster = monster_factory(&player_hero, modifier);

            fight(&mut player_hero, &mut this_monster);

            let mut rng = thread_rng();
            let ten_millis = time::Duration::from_secs(rng.gen_range(0, 3));
            let now = time::Instant::now();

            thread::sleep(ten_millis);
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

pub fn fight(player: &mut Hero, monster: &mut Monster) {
    let player_power = calc_player_fight_stats(player);
    let monster_power = calc_monster_fight_stats(monster);
}

pub fn calc_player_fight_stats(player: &Hero) -> u64{
    100 as u64
}
pub fn calc_monster_fight_stats(player: &Monster) -> u64{
    100 as u64
}

