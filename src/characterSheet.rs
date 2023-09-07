use statslibrary::*;
mod statslibrary;

pub struct Character {
    name: String,
    stats: statslibrary::CharacterStats,
    species: Species,
}

pub enum Species {
    Human,
    Andriod,
}

