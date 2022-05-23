use bevy_ecs::system::{Res, ResMut};
use rand::seq::SliceRandom;

use crate::core::pawn::resources::UsedNames;

const MALE_FIRST_NAMES: &[&str] = &[
    "Anakin", "Angel", "Abel", "Artemis", "Arthur", "Bastian", "Cullen", "Emmett", "Falkor",
    "Faramir", "Fox", "Gandalf", "Gaius", "Geordi", "Grant", "Han", "Harry", "Hugo", "Idris",
    "Jareth", "Joffrey", "John", "Kael", "Logan", "Ludo", "Mary", "Milo", "Odo", "Ronan", "Rory",
    "Rowan", "Rylan", "Sauron", "Septimus", "Spike", "Spock", "Sulu", "Tyrian", "Westley",
    "William", "Xavier", "Zack", "Yarian", "Zorrish", "Samlan", "Wylran", "Elldrick", "Archiah",
    "Sorkkon", "Xiah", "Yazan", "Ryland", "Kaiton", "Aidken", "Gideon", "Kieran", "Ureem", "Malax",
    "Kalban", "Wavarek", "Rex", "Yariq", "Tariq", "Finriel", "Israel", "Xumir", "Irivan", "Samion",
    "Finnec", "Falko", "Waverek", "Parker", "Ronias", "Orby", "Tiran", "Steve", "Steven", "Kalett",
    "Yarwick", "Jango", "Brolek", "Xavian", "Sorkku", "Ignazlan", "Lyrikkon", "Kelslow", "Iklan",
    "Zannik", "Fenncom", "Emsen", "Kartan", "Yazan", "Blayden", "Kellek", "Wayven", "Pakon",
    "Kenmon", "Graygal", "Bobba", "Cadael", "Xantry", "Bengorn", "Yaddu", "Ikev", "Lokesh", "Wolf",
    "Falco",
];
const FEMALE_FIRST_NAMES: &[&str] = &[
    "Aeryn",
    "Amelia",
    "Anastasia",
    "Aquila",
    "Arya",
    "Astrid",
    "Padme",
    "Aurora",
    "Aurra",
    "Auryn",
    "Buttercup",
    "Cherlindrea",
    "Clara",
    "Sharon",
    "Cora",
    "Danan",
    "Diana",
    "Donna",
    "Echo",
    "Elora",
    "Eowyn",
    "Olya",
    "Caroline",
    "Felicity",
    "Fleur",
    "Galadriel",
    "Glinda",
    "Emma",
    "Luna",
    "Charlotte",
    "Yvette",
    "Dominique",
    "Kelly",
    "Boxy",
    "Isabaeu",
    "Sabina",
    "Rebecka",
    "Kamala",
    "Kara",
    "Kathryn",
    "Anno",
    "Katniss",
    "Nyota",
    "Nancy",
    "Padme",
    "Peggy",
    "Raven",
    "Renesmee",
    "Ripley",
    "River",
    "Samantha",
    "Sarah",
    "Sonya",
    "Sorsha",
    "Tauriel",
    "Teyla",
    "Valerian",
    "Willow",
    "Krystal",
    "Katelyne",
    "Weronika",
    "Berenika",
];
const LAST_NAMES: &[&str] = &[
    "Voight",
    "Barick",
    "Nicolau",
    "Cantos",
    "Tian",
    "Carthen",
    "McRaven",
    "Foxwell",
    "Fett",
    "Albach",
    "Amidala",
    "Steward",
    "Woldt",
    "Cedeno",
    "Catlow",
    "Kinton",
    "Zahra",
    "Castillion",
    "Nyseth",
    "Rhyne",
    "Malik",
    "Sonoda",
    "Avison",
    "Philips",
    "Sarratt",
    "Zechiel",
    "Callahan",
    "Chrysalis",
    "Nadir",
    "Corona",
    "Rahman",
    "Alastair",
    "Haskovo",
    "Vitality",
    "Sharjah",
    "Khepri",
    "Raptor",
    "Colfax",
    "Moondust",
    "Atrius",
    "Dianthus",
    "Kelmis",
    "Bani-Mazar",
    "Alpheus",
    "Skywalker",
    "Kenobi",
    "Maul",
    "McCloud",
    "O'Donnell",
    "Lombardi",
];

pub fn get_full_name(gender: bool, unique: bool, used_names: &Res<UsedNames>) -> String {
    let rng = &mut rand::thread_rng();

    let first_name = match gender {
        true => MALE_FIRST_NAMES,
        false => FEMALE_FIRST_NAMES,
    }.choose(rng).unwrap();

    let full_name = format!("{first_name} {}", LAST_NAMES.choose(rng).unwrap());

    if unique && used_names.names.contains_key(&full_name) {
        get_full_name(gender, unique, used_names)
    } else {
        full_name
    }
}

pub fn get_dummy_name(used_names: &mut ResMut<UsedNames>) -> String {
    let return_name = format!("Dummy {}", used_names.dummy_i);

    used_names.dummy_i += 1;

    return_name
}
