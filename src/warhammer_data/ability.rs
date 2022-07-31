
use super::ros_parser::ros_parser::Selection;

/// Profile for what an ability changes
#[derive(Debug, Clone)]
pub struct Ability {
    // Need to implement this later to actually change other things
    pub value: String,
    pub name: String,
}

pub fn parse_abilities(selection: &Selection) -> Vec<Ability> {
    let mut v: Vec<Ability> = Vec::new();

    match &selection.selections {
        Some(s) => {
            for selection in &s.selections {
                v.append(&mut parse_abilities(&selection));
            }
            if selection.profiles != None {
                for profile in &selection.profiles.as_ref().unwrap().profiles {
                    if profile.type_name == "Abilities" {
                        for characteristic in profile.characteristics.characteristics.to_owned() {
                            v.push(Ability {
                                value: characteristic.value.unwrap(),
                                name: profile.name.to_owned(),
                            })
                        }
                    }
                }
            }
        }
        None => {
            if selection.profiles != None {
                for profile in &selection.profiles.as_ref().unwrap().profiles {
                    if profile.type_name == "Abilities" {
                        for characteristic in profile.characteristics.characteristics.to_owned() {
                            v.push(Ability {
                                value: characteristic.value.unwrap(),
                                name: profile.name.to_owned(),
                            })
                        }
                    }
                }
            }
        }
    }

    v
}
