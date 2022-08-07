use lazy_static::lazy_static;
use regex::Regex;

use crate::warhammer_data::ability::Ability;
use crate::warhammer_data::profile::ProfileValue;
use crate::warhammer_data::ros_parser::ros_parser::{Characteristics, Selection};

use super::ros_parser::ros_parser::Profile;

#[derive(Debug, Clone, Copy)]
pub enum WeaponType {
    Pistol,
    Assault,
    Heavy,
    RapidFire,
    Grenade,
    Melee,
}

#[derive(Debug, Clone)]
pub struct WeaponStrength {
    pub value: u8,
    pub strength_type: WeaponStrengthType,
}

#[derive(Debug, Clone)]
pub enum WeaponStrengthType {
    Addition,
    Multiply,
    Flat,
}

#[derive(Debug, Clone)]
pub struct WeaponProfile {
    pub abilities: Option<Vec<Ability>>,
    pub range: Option<u8>,
    pub weapon_type: WeaponType,
    pub attacks: Option<ProfileValue>,
    pub strength: WeaponStrength,
    pub armour_penetration: i8,
    pub damage: ProfileValue,
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub profile: WeaponProfile,
    pub number: u8,
}

impl WeaponProfile {
    pub fn from_characterics(weapon_characteristics: &Characteristics) -> Result<Self, String> {
        // Temp weapon to return
        let mut weapon = Self {
            abilities: None,
            range: None,
            weapon_type: WeaponType::Melee,
            attacks: None,
            strength: WeaponStrength {
                value: 0,
                strength_type: WeaponStrengthType::Addition,
            },
            armour_penetration: 0,
            damage: ProfileValue {
                dice_value: None,
                flat_value: None,
            },
        };

        lazy_static! {
            // Regex to parse out distance value
            // There is a question mark after the " because lasguns are stupid
            static ref DIST_RE: Regex = Regex::new(r#"(\d+)"?"#).unwrap();

            static ref STRENGTH_RE: Regex = Regex::new(r#"(\+|x)(\d+)"#).unwrap();
        }

        // Iterate through characteristics
        for characteristic in &weapon_characteristics.characteristics {
            // Match on characteristic name
            match characteristic.name.as_str() {
                // Parse range which is a distance
                "Range" => {
                    weapon.range = match DIST_RE.captures(characteristic.value.as_ref().unwrap()) {
                        Some(_) => Some(
                            DIST_RE
                                .captures(characteristic.value.as_ref().unwrap())
                                .unwrap()[1]
                                .parse()
                                .unwrap(),
                        ),
                        None => None,
                    }
                }
                // Parse type which has 2 string, first being the type and second being the number of shots
                "Type" => {
                    let s = characteristic.value.as_ref().unwrap().split(" ");
                    let sv: Vec<&str> = s.collect();

                    // Get weapon type and assign enum. The first entry in sv vector is the weapon type as a string
                    match *(sv.get(0).unwrap()) {
                        "Assault" => weapon.weapon_type = WeaponType::Assault,
                        "Heavy" => weapon.weapon_type = WeaponType::Heavy,
                        "Rapid" => weapon.weapon_type = WeaponType::RapidFire,
                        "Grenade" => weapon.weapon_type = WeaponType::Grenade,
                        "Pistol" => weapon.weapon_type = WeaponType::Pistol,
                        "Melee" => weapon.weapon_type = WeaponType::Melee,
                        &_ => {
                            return Err(
                                "Unsupported weapon type: ".to_owned() + *(sv.get(0).unwrap())
                            )
                        }
                    }

                    // Get weapon type and create the number of attacks.
                    match *(sv.get(0).unwrap()) {
                        "Assault" | "Heavy" | "Grenade" | "Pistol" => {
                            weapon.attacks = Some(ProfileValue::from_str(sv.get(1).unwrap()))
                        }
                        // Rapid is in its own since it is "Rapid Fire {num}" which gets split into 3 strings.
                        "Rapid" => {
                            weapon.attacks = Some(ProfileValue::from_str(sv.get(2).unwrap()))
                        }
                        // Melee uses models attacks.
                        "Melee" => weapon.attacks = None,
                        &_ => {
                            return Err(
                                "Unsupported weapon type: ".to_owned() + *(sv.get(0).unwrap())
                            )
                        }
                    }
                }
                // Parse Strength
                // For melee, this is added to model strength and can be added(+) or multiplied(x)
                "S" => {
                    if characteristic.value.as_ref().unwrap().eq("*") {
                        weapon.strength = WeaponStrength {
                            value: 0,
                            strength_type: WeaponStrengthType::Flat,
                        }
                    } else {
                        weapon.strength = {
                            match STRENGTH_RE.captures(characteristic.value.as_ref().unwrap()) {
                                Some(captures) => WeaponStrength {
                                    value: captures.get(2).unwrap().as_str().parse().unwrap(),
                                    strength_type: match captures.get(1).unwrap().as_str() {
                                        "+" => WeaponStrengthType::Addition,
                                        "x" => WeaponStrengthType::Multiply,
                                        _ => {
                                            return Err(
                                                "Unknown specifier for weapon strength".to_string()
                                            )
                                        }
                                    },
                                },
                                None => WeaponStrength {
                                    value: characteristic.value.as_ref().unwrap().parse().unwrap(),
                                    strength_type: WeaponStrengthType::Flat,
                                },
                            }
                        }
                    }
                }
                // Parse armour pen
                "AP" => {
                    if characteristic.value.as_ref().unwrap().eq("*") {
                        weapon.armour_penetration = 0
                    } else {   
                        weapon.armour_penetration =
                        characteristic.value.as_ref().unwrap().parse().unwrap()
                    }
                }
                // Parse damage which can be a dice value
                "D" => {
                    if characteristic.value.as_ref().unwrap().eq("*") {
                        weapon.damage = ProfileValue {
                            dice_value: None,
                            flat_value: Some(0),
                        }
                    } else {
                        weapon.damage = ProfileValue::from_str(characteristic.value.as_ref().unwrap())
                    }
                }
                // Parse ability.
                "Abilities" => {
                    if characteristic.value.as_ref().unwrap() != "-" {
                        match weapon.abilities {
                            // Push new ability if vector has already been created
                            Some(ref mut f) => f.push(Ability {
                                value: characteristic.value.as_ref().unwrap().to_string(),
                                name: "".to_string(),
                            }),
                            // Create vector and add ability to it since abilities is None and wants to stay None if there isn't any
                            None => {
                                weapon.abilities = Some(vec![Ability {
                                    value: characteristic.value.as_ref().unwrap().to_string(),
                                    name: "".to_string(),
                                }])
                            }
                        }
                    }
                }
                _ => {
                    return Err("Unknown characteristicfor unit: ".to_owned()
                        + characteristic.name.as_str())
                }
            }
        }

        Ok(weapon)
    }
}

impl Weapon {
    pub fn from_selection(weapon_selection: &Selection) -> Result<Self, String> {
        // Create and return weapon
        Ok(Self {
            name: weapon_selection.profiles.as_ref().unwrap().profiles[0]
                .name
                .to_string(),
            profile: WeaponProfile::from_characterics(
                &weapon_selection.profiles.as_ref().unwrap().profiles[0].characteristics,
            )
            .unwrap(),
            number: weapon_selection.number,
        })
    }

    pub fn from_profile(weapon_profile: &Profile) -> Result<Self, String> {
        Ok(Self {
            name: weapon_profile.name.clone(),
            profile: WeaponProfile::from_characterics(&weapon_profile.characteristics).unwrap(),
            number: 1,
        })
    }
}
