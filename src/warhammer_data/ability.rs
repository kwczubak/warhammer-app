
use super::ros_parser::ros_parser::{Selection, Profile};

/// Profile for what an ability changes
#[derive(Debug, Clone)]
pub struct Ability {
    // Need to implement this later to actually change other things
    pub value: String,
    pub name: String,
}

impl Ability {
    pub fn from_profile(profile: &Profile) -> Result<Self, String> {
        if profile.type_name != "Abilities" {
            return Err("Tried to parse ability with non ability profile".to_string());
        }

        for characteristic in &profile.characteristics.characteristics {
            match characteristic.name.as_str() {
                "Description" => return Ok(Self { value: characteristic.value.as_ref().unwrap().to_string(), name: characteristic.name.to_owned() }),
                _ => return Err("Unknown characteritic for ability".to_string() + &characteristic.name),
            };
        }

        Err("No description for ability".to_string())
    }
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
