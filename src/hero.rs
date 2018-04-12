pub mod hero_mod {
    use std::fmt;
    use std::fs::File;
    use std::io::prelude::*;
    use serde_json;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Hero {
        pub name: String,
        pub age: u8,
        pub gender: Gender,
    }

    pub fn hero_factory(char_name: &str, char_age: u8, char_gender: Gender) -> Hero {
        Hero {
            name: String::from(char_name),
            age: char_age,
            gender: char_gender,
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Gender {
        Male,
        Female,
    }

    impl fmt::Display for Gender {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub fn save_hero(hero_to_save: &Hero) {
        let mut config_file = get_config_file(FileMode::Save);
        let contents = serde_json::to_string(&hero_to_save).unwrap();
        config_file
            .write_all(contents.as_bytes())
            .expect("Saved Hero.");
    }

    pub fn load_hero() -> Hero {
        let mut deserialized_hero = String::new();
        let mut config_file = get_config_file(FileMode::Load);
        config_file.read_to_string(&mut deserialized_hero).unwrap();
        serde_json::from_str(&deserialized_hero).expect("Loaded hero!")
    }

    pub fn get_config_file(mode: FileMode) -> File {
        match mode {
            FileMode::Load => match File::open("he_ru.config") {
                Ok(file_to_return) => return file_to_return,
                Err(_error) => panic!("Couldn't open file!"),
            },
            FileMode::Save => match File::create("he_ru.config") {
                Ok(file_to_return) => return file_to_return,
                Err(_error) => panic!("Couldn't save file!"),
            },
        }
    }

    pub enum FileMode {
        Save,
        Load,
    }


#[test]
fn hero_test_save_load() {
    let mut hero = hero_factory("Ben", 26, Gender::Male);
    save_hero(&hero);
    hero = load_hero();

    assert_eq!(hero.name, "Ben");
    assert_eq!(hero.age, 26);
    assert_eq!(hero.gender.to_string(), "Male");

    hero = hero_factory("Charlie", 16, Gender::Female);
    save_hero(&hero);
    hero = load_hero();

    assert_eq!(hero.name, "Charlie");
    assert_eq!(hero.age, 16);
    assert_eq!(hero.gender.to_string(), "Female");
}
}
