use statslibrary::*;
mod statslibrary;

pub struct Character {


    name: String,
    stats: statslibrary::CharacterStats,
    species: Species,
}
impl Character {
    fn skill_check(&self) {

    
    }
    
    fn saving_throws(&self) {
    
    
    }
    
    fn combat_rounds(&self) {
    
        
    
    }

}

pub enum Species {
    Human,
    Andriod,
}

