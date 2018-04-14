#[macro_use]
extern crate serde_derive;

extern crate rand;
extern crate serde;
extern crate serde_json;

use std::io;
use std::{thread, time};
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

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
        let mut menu: bool = true;
        let mut fighting: bool = false;
        let mut modifier: u8 = 1;

        while menu {
            let selection = display_options();
            match selection {
                1 => { display_character_stats(&player_hero);},
                2 => {
                    modifier = pick_training_spot();
                    menu = false;
                    fighting = true;
                }
                _ => {},
            };
        }

        

        while fighting {
            //Important that this is before fight algorithm, creates a weird delay after killing a monster if not.
            let mut rng = thread_rng();
            let ten_millis = time::Duration::from_secs(rng.gen_range(3, 5));
            thread::sleep(ten_millis);

            let mut this_monster = monster_factory(&player_hero, modifier);

            fight(&mut player_hero, &mut this_monster);

            //TODO: Do something when player dies.
            match 0.cmp(&player_hero.health) {
                Ordering::Equal => {
                    player_hero.die();
                    fighting = false;
                }
                Ordering::Greater => {
                    player_hero.die();
                    fighting = false;
                }
                Ordering::Less => {
                    player_hero.kills += 1;
                }
            };

            //TODO: Do something when monster dies.
            match 0.cmp(&this_monster.health) {
                Ordering::Equal => {
                    this_monster.die();
                    fighting = false;
                }
                Ordering::Greater => {
                    this_monster.die();
                    fighting = false;
                }
                Ordering::Less => {},
            }
            save_hero(&player_hero);
        }
    }
}

pub fn display_greeting() {
    println!();
    println!("Welcome to he_ru!");
    println!("This is an idle game for managing a hero.");
    println!();
}

pub fn display_options() -> u8{
    println!();
    println!("What would you like to do:");
    println!("1. Character Stats");
    println!("2. Train");
    println!("3. Manage Inventory (Not Yet Implemented)");

    let mut selection = String::new();

    io::stdin().read_line(&mut selection).unwrap();
    selection.trim().parse().unwrap()
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
    while player.health >= 0 && monster.health >= 0 {
        let player_power = calc_player_fight_stats(player);
        let monster_power = calc_monster_fight_stats(monster);
        player.health -= monster_power;
        monster.health -= player_power;
    }
}

pub fn calc_player_fight_stats(_player: &Hero) -> i64 {
    //FIXME: For now, we are just making all heros calculate damage the same,
    //but in the future, when classes are implemented, this will need to be reworked.
    let raw_damage = _player.strength as i64;

    let mut rng = thread_rng();
    let roll_d_100 = rng.gen_range(0, 100);

    match _player.luck.cmp(&roll_d_100) {
        Ordering::Equal => {
            return apply_crit_modifyer(&raw_damage);
        }
        Ordering::Greater => {
            return apply_crit_modifyer(&raw_damage);
        }
        Ordering::Less => {
            return raw_damage;
        }
    }
}
pub fn calc_monster_fight_stats(_monster: &Monster) -> i64 {
    //FIXME: For now, we are just making all heros calculate damage the same,
    //but in the future, when classes are implemented, this will need to be reworked.
    let raw_damage = (_monster.strength / 2) as i64;

    let mut rng = thread_rng();
    let roll_d_100 = rng.gen_range(0, 100);

    match _monster.luck.cmp(&roll_d_100) {
        Ordering::Equal => {
            return apply_crit_modifyer(&raw_damage);
        }
        Ordering::Greater => {
            return apply_crit_modifyer(&raw_damage);
        }
        Ordering::Less => {
            return raw_damage;
        }
    }
}

pub fn apply_crit_modifyer(raw_damage: &i64) -> i64 {
    (raw_damage * 2) as i64
}

pub fn display_character_stats(player: &Hero) {
    println!();
    println!("=============================");
    println!("Hero Name: {}", player.name);
    println!("Level: {}", player.level);
    println!("Kills: {}", player.kills);
    println!("=============================");
    println!("Max Health: {}", player.max_health);
    println!("Strength: {}", player.strength);
    println!("Dexterity: {}", player.dexterity);
    println!("Intelligence: {}", player.intelligence);
    println!("Luck: {}", player.luck);


}