pub mod monster_mod {
    use hero::hero_mod::*;

    pub struct Monster {
        pub level: u8,
        pub health: u64,
        pub strength: u16,
        pub dexterity: u16,
        pub intelligence: u16,
        pub luck: u16,
    }

    pub fn monster_factory(player: &Hero, modifier: u8) -> Monster {
        Monster {
            level: player.level + modifier,
            health: player.health * (modifier as u64),
            strength: player.strength + (modifier as u16),
            dexterity: player.dexterity + (modifier as u16),
            intelligence: player.intelligence + (modifier as u16),
            luck: player.luck + (modifier as u16),
        }
    }
}
