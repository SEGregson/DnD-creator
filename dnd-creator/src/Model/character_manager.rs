use super::stats::Character;

// later replace with database
pub struct CharacterManager {
    chars: Vec<Character>
}

impl CharacterManager {
    pub fn new() -> CharacterManager {
        CharacterManager {
            chars: vec![]
        }
    }

    pub fn add(&mut self, new: Character) {
        self.chars.push(new);
    }

    pub fn get(&self, name: String) -> Option<&Character> {
        for i in &self.chars {
            if i.name == name {
                return Some(i)
            }
        }
        None
    }

    pub fn get_mut(&mut self, name: String) -> Option<&Character> {
        for i in &mut self.chars {
            if i.name == name {
                return Some(i)
            }
        }
        None
    }

    pub fn get_names(&self) -> Vec<String> {
        let mut out: Vec<String> = vec![];
        for i in &self.chars {
            out.push(i.name.clone());
        }
        out
    }
}