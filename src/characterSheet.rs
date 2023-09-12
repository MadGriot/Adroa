use statslibrary::*;
mod statslibrary;
mod systems;

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

struct StatRolls;
impl StatRolls {
    pub fn roll_stats(&mut self, character: &mut Character) {

        character.stats.attributes.strength = 3.0 * systems::DiceRolls.d_6() as f32;
        character.stats.attributes.dexterity = 3.0 * systems::DiceRolls.d_6() as f32;
        character.stats.attributes.constitution = 3.0 * systems::DiceRolls.d_6() as f32;
        character.stats.attributes.intelligence = 3.0 * systems::DiceRolls.d_6() as f32;
        character.stats.attributes.wisdom = 3.0 * systems::DiceRolls.d_6() as f32;
        character.stats.attributes.charisma = 3.0 * systems::DiceRolls.d_6() as f32;

    }
    
}
