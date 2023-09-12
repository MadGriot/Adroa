pub struct CharacterStats {
    pub attributes: attr_parameters,
    skills: skills_parameters,
    psychic_skills: psychic_skills_parameters,
    background: BackgroundType,
    class: ClassType,
    foci: FocusType,
}

pub struct attr_parameters {
    pub strength: f32,
    pub dexterity: f32,
    pub constitution: f32,
    pub intelligence: f32,
    pub wisdom: f32,
    pub charisma: f32,
}

pub struct skills_parameters {
    administer: f32,
    connect: f32,
    exert: f32,
    fix: f32,
    heal: f32,
    know: f32,
    lead: f32,
    notice: f32,
    perform: f32,
    pilot: f32,
    program: f32,
    punch: f32,
    shoot: f32,
    sneak: f32,
    stab: f32,
    survive: f32,
    talk: f32,
    trade: f32,
    work: f32,
}

pub struct psychic_skills_parameters {
    biopsionics: f32,
    metapsionics: f32,
    precognition: f32,
    telekinesis: f32,
    telepathy: f32,
    teleportation: f32,
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