#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod hero;
use hero::hero_mod::*;


fn main() {
    //TODO: Try loading a hero first, then if that fails, prompt to create a new one.
    let player_hero = hero_factory("Ben", 26, Gender::Male);
    save_hero(&player_hero);
}
