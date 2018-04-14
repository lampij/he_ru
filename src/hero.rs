pub mod hero_mod {
    use std::fmt;
    use std::fs::File;
    use std::io::prelude::*;
    use serde_json;
    use std::io;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Hero {
        pub name: String,
        pub age: u8,
        pub gender: Gender,
        pub level: u8,
        pub health: i64,
        pub strength: u16,
        pub dexterity: u16,
        pub intelligence: u16,
        pub luck: u16,
        pub kills: u64,
    }

    pub fn hero_factory(char_name: &str, char_age: u8, char_gender: Gender) -> Hero {
        Hero {
            name: String::from(char_name),
            age: char_age,
            gender: char_gender,
            level: 0,
            health: 100,
            strength: 5,
            dexterity: 5,
            intelligence: 5,
            luck: 5,
            kills: 0,
        }
    }

    pub fn get_hero_details_from_user() -> Hero {
        let mut input_from_user = String::new();
        let mut ret_hero = hero_factory("", 0, Gender::Male);

        println!("Please enter your hero's name: ");
        io::stdin().read_line(&mut input_from_user).unwrap();
        ret_hero.name = input_from_user;

        let mut input_from_user = String::new();

        println!("Please enter your hero's age: ");
        io::stdin().read_line(&mut input_from_user).unwrap();
        ret_hero.age = input_from_user.trim().parse().unwrap();
        let mut input_from_user = String::new();

        println!("Please enter your hero's gender(male/female): ");
        io::stdin().read_line(&mut input_from_user).unwrap();

        let _male = String::from("MALE");
        let _female = String::from("FEMALE");

        if _male.eq_ignore_ascii_case(&input_from_user){
            ret_hero.gender = Gender::Male;
        }
        else if _female.eq_ignore_ascii_case(&input_from_user){
            ret_hero.gender = Gender::Female;
        }
        else{
            ret_hero.gender = Gender::Other;
        }

        ret_hero
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Gender {
        Male,
        Female,
        Other
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
        match serde_json::from_str(&deserialized_hero) {
            Ok(hero_to_return) => {
                let hero_to_return: Hero = hero_to_return;
                println!("Loaded {}, level {}, with {} kills.", hero_to_return.name, hero_to_return.level, hero_to_return.kills);
                return hero_to_return
                },
            Err(_message) => return hero_factory("", 0, Gender::Male),
        }
    }

    //TODO: Don't panic when file can't be opened.
    pub fn get_config_file(mode: FileMode) -> File {
        match mode {
            FileMode::Load => match File::open("he_ru.config") {
                Ok(file_to_return) => return file_to_return,
                Err(_error) => {
                    File::create("he_ru.config").unwrap();
                    return get_config_file(FileMode::Load);
                }
            },
            FileMode::Save => match File::create("he_ru.config") {
                Ok(file_to_return) => return file_to_return,
                Err(_error) => panic!("Couldn't Save!"),
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
