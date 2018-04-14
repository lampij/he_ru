pub mod monster_mod {
    use hero::hero_mod::*;
    use rand::{thread_rng, Rng};

    pub struct Monster {
        pub level: u8,
        pub health: i64,
        pub strength: u16,
        pub dexterity: u16,
        pub intelligence: u16,
        pub luck: u16,
    }

    impl Monster {
        pub fn die(&self){
            println!("Monster has died!");
        }
    }

    pub fn monster_factory(player: &Hero, modifier: u8) -> Monster {
        let mut rng = thread_rng();
        Monster {
            level: player.level + modifier,
            health: player.health + ((rng.gen_range(0, modifier * 5) as i64) * 5) as i64,
            strength: player.strength + (rng.gen_range(0, modifier * 5) as u16),
            dexterity: player.dexterity + (rng.gen_range(0, modifier * 5) as u16),
            intelligence: player.intelligence + (rng.gen_range(0, modifier * 5) as u16),
            luck: player.luck + (rng.gen_range(0, modifier * 5) as u16),
        }
    }
}
