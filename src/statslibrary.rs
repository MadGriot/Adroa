pub struct CharacterStats {
    attributes: attr_parameters,
    skills: skills_parameters,
    psychic_skills: phychic_skills_parameters,
    background: BackgroundType,
    class: ClassType,
    foci: FocusType,
}

pub struct attr_parameters {
    strength: f64,
    dexterity: f64,
    constitution: f64,
    intelligence: f64,
    wisdom: f64,
    charisma: f64,
}

pub struct skills_parameters {
    administer: f64,
    connect: f64,
    exert: f64,
    fix: f64,
    heal: f64,
    know: f64,
    lead: f64,
    notice: f64,
    perform: f64,
    pilot: f64,
    program: f64,
    punch: f64,
    shoot: f64,
    sneak: f64,
    stab: f64,
    survive: f64,
    talk: f64,
    trade: f64,
    work: f64,
}

pub struct phychic_skills_parameters {
    biopsionics: f64,
    metapsionics: f64,
    precognition: f64,
    telekinesis: f64,
    telepathy: f64,
    teleportation: f64,
}

pub enum BackgroundType {
    Barbarian,
    Clergy,
    Courtesan,
    Criminal,
    Dilettante,
    Entertainer,
    Merchant,
    Noble,
    Official,
    Peasant,
    Physician,
    Pilot,
    Politician,
    Scholar,
    Soldier,
    Spacer,
    Technician,
    Thug,
    Vagabond,
    Worker,
}

pub enum ClassType {
    Expert {level: i8},
    Psychic {level: i8},
    Warrior {level: i8},
    Adventurer {level: i8},
}

pub enum FocusType {
    Alert {level: i8 },
    Armsman {level: i8 },
    Assassin {level: i8},
    Authority {level: i8},
    CloseCombat {level: i8},
    Connected {level: i8},
    DieHard {level: i8},
    Diplomat {level: i8},
    Gunslinger {level: i8},
    Hacker {level: i8},
    Healer {level: i8},
    Ironhide {level: i8},
    PsychicTraining {level: i8},
    SavageFray {level: i8},
    Sniper {level: i8},
    Specialist {level: i8},
    StarCaptain {level: i8},
    Starfarer {level: i8},
    Tinker {level: i8},
    UnarmedCombat {level: i8},
    Wanderer {level: i8},
    WildPsychicTalent {level: i8},
}