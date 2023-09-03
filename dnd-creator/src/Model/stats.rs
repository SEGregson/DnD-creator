use rand::Rng;

pub fn roll_d(num: u8) -> u8 {
    rand::thread_rng().gen_range(1..num)
}




trait Attribute  {
    fn roll(&self) -> i8;
    fn get_type() -> String;
}

#[derive(Clone)]
struct Stat {
    // modifier
    modi: i8,
    // base role
    stat: u8,
}


impl Stat {
    fn new(roll: u8) -> Stat {
        let mut modi: i8 = roll.try_into().unwrap();
        modi = (modi-10)/2;
        Stat {
            modi,
            stat: roll,
        }
    }
}

impl Attribute for Stat {
    fn roll(&self) -> i8 {
        let roll: i8 = (roll_d(self.stat)).try_into().unwrap();
        roll  + self.modi
    }

    fn get_type() -> String {
        "stat".to_string()
    }
}
#[derive(Clone)]
struct Skill {
    stat: Stat,
    prof: bool,
}

impl Skill {
    fn new(stat:Stat, prof: bool) -> Skill {
        Skill {
            stat,
            prof,
        }
        
    }
}
impl Attribute for Skill {
    fn roll(&self) -> i8 {
        let mut roll: i8 = (roll_d(self.stat.stat)).try_into().unwrap();
        roll += self.stat.modi;

        return if self.prof {
            // need to add prof bonus
            roll
        } else {
            roll
        }
    }

    fn get_type() -> String {
        "skill".to_string()
    }
}

#[derive(Clone)]
pub struct Character {
    pub name: String,

    level: u8,
    ac: u8,
    init: u8,
    spd: u8,

    // Stats
    stre: Stat,
    dex: Stat,
    con: Stat,
    int: Stat,
    wis: Stat,
    cha: Stat,

    // Skills
    acro:           Skill,
    animal_hand:    Skill,
    arcana:         Skill,
    athle:          Skill,
    decept:         Skill,
    his:            Skill,
    ins:            Skill,
    intim:          Skill,
    invest:         Skill,
    med:            Skill,
    nat:            Skill,
    percept:        Skill,
    perf:           Skill,
    pers:           Skill,
    relig:          Skill,
    slight_hand:    Skill,
    stealth:        Skill,
    surv:           Skill,
}

impl Character {
    //fn new() -> Character {}

    fn skill_check(skill: impl Attribute, adv: Option<bool>) -> i8 {
        let roll = match adv {
                Some(adv) => {
                    let roll1 = skill.roll();
                    let roll2 = skill.roll();
                    
                     if adv {std::cmp::max(roll1, roll2)} else {std::cmp::min(roll1, roll2)}
                },
                None => skill.roll(),
            };
        return roll
    }
}