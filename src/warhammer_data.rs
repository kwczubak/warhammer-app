use crate::ros_parser::{self, Characteristic, Selection};
use regex::Regex;

#[derive(Debug)]
pub enum WeaponType {
    Pistol,
    Assault,
    Heavy,
    RapidFire,
    Grenade,
    Meele,
}

#[derive(Debug)]
pub enum DiceType {
    D3,
    D6,
}

#[derive(Debug)]
pub struct DiceRoll {
    pub number: Option<u8>,
    pub dice_type: DiceType,
}

#[derive(Debug)]
pub struct ProfileValue {
    pub dice_value: Option<DiceRoll>,
    pub flat_value: Option<u8>, 
}

impl ProfileValue {
    pub fn from_str(s: &str) -> Self {
        let flat_value : Option<u8>;
        let mut dice_value : Option<DiceRoll> = None;
        let dice_re = Regex::new(r"(\d?)D?(\d?)\+?(\d?)").unwrap();

        let caps = dice_re.captures(s).unwrap();
        if caps.get(2).unwrap().as_str() == "" { // There is no dice value
            flat_value = Some(caps.get(1).unwrap().as_str().parse().unwrap());
        }
        else { // There is dice value
            flat_value = match caps.get(3).unwrap().as_str() {
                "" => None,
                _ => Some(caps.get(3).unwrap().as_str().parse().unwrap()),
            };
            
            let number : Option<u8>;
            match caps.get(1).unwrap().as_str() {
                "" => number = None,
                _ => number = Some(caps.get(1).unwrap().as_str().parse().unwrap()),
            }
            dice_value = match caps.get(2).unwrap().as_str() {
                "3" => Some(DiceRoll{dice_type: DiceType::D3, number: number}),
                "6" => Some(DiceRoll{dice_type: DiceType::D6, number: number}),
                _ => None,
            }
        }
        Self { dice_value: dice_value, flat_value: flat_value }
    }
}

#[derive(Debug)]
pub struct ModelProfile {
    pub movement: u8,
    pub weapon_skill: u8,
    pub ballistic_skill: u8,
    pub strength: u8,
    pub toughness: u8,
    pub wounds: u8,
    pub attacks: ProfileValue,
    pub leadership: u8,
    pub save: u8,
}

#[derive(Debug)]
pub struct Ability {
    // Need to implement this later to actually change other things
    pub value: String,
}

#[derive(Debug)]
pub struct WeaponProfile {
    pub abilities: Option<Vec<Ability>>,
    pub range: Option<u8>,
    pub weapon_type: WeaponType,
    pub attacks: Option<ProfileValue>,
    pub strength: u8,
    pub armour_penetration: i8,
    pub damage: ProfileValue,
}

#[derive(Debug)]
pub struct Weapon {
    pub name: String,
    pub profile: WeaponProfile,
    pub number: u8,
}

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub profiles: Vec<ModelProfile>,
    pub weapons: Vec<Weapon>,
    pub abilities: Vec<Ability>,
    pub number: u8,
}

#[derive(Debug)]
pub struct Unit {
    pub name: String,
    pub abilities: Vec<Ability>,
    pub keywords: Vec<String>,
    pub invulnable_save: Option<u8>,
    pub models: Vec<Model>,
}

#[derive(Debug)]
pub struct Detachment {
    pub name: String,
    pub abilities: Vec<Ability>,
    pub units: Vec<Unit>,
}

#[derive(Debug)]
pub struct Army {
    pub detachments: Vec<Detachment>,
    pub cp: u8,
}

pub fn parse_unit_profile(characteristics: &ros_parser::Characteristics) -> Result<ModelProfile, String> {
    let iter = characteristics.characteristics.iter();
    let mut profile = ModelProfile {
        movement: 0,
        weapon_skill: 0,
        ballistic_skill: 0,
        strength: 0,
        toughness: 0,
        wounds: 0,
        attacks: ProfileValue { dice_value: None, flat_value: None },
        leadership: 0,
        save: 0,
    };

    let up_re = Regex::new(r"(\d)\+").unwrap();
    let dist_re = Regex::new(r#"(\d+)""#).unwrap();

    for characteristic in iter {
        match characteristic.name.as_str() {
            "M" => profile.movement = dist_re.captures(characteristic.value.as_ref().unwrap()).unwrap()[1].parse().unwrap(), // todo error checking
            "WS" => profile.weapon_skill = up_re.captures(characteristic.value.as_ref().unwrap()).unwrap()[1].parse().unwrap(),
            "BS" => profile.ballistic_skill = up_re.captures(characteristic.value.as_ref().unwrap()).unwrap()[1].parse().unwrap(),
            "S" => profile.strength = characteristic.value.as_ref().unwrap().parse().unwrap(),
            "T" => profile.toughness = characteristic.value.as_ref().unwrap().parse().unwrap(),
            "W" => profile.wounds = characteristic.value.as_ref().unwrap().parse().unwrap(), // Can be N/A
            "A" => profile.attacks = ProfileValue::from_str(characteristic.value.as_ref().unwrap()),
            "Ld" => profile.leadership = characteristic.value.as_ref().unwrap().parse().unwrap(),
            "Save" => profile.save = up_re.captures(characteristic.value.as_ref().unwrap()).unwrap()[1].parse().unwrap(),
            _ => return Err("Unknown characteristicfor unit: ".to_owned() + characteristic.name.as_str())
        }
    }

    Ok(profile)
}

pub fn parse_weapon_profile(weapon_characteristics: &ros_parser::Characteristics) -> Result<WeaponProfile, String> {
    let iter = weapon_characteristics.characteristics.iter();
    let mut weapon = WeaponProfile {
        abilities: None,
        range: None,
        weapon_type: WeaponType::Meele,
        attacks: None,
        strength: 0,
        armour_penetration: 0,
        damage: ProfileValue { dice_value: None, flat_value: None },
    };

    let dist_re = Regex::new(r#"(\d+)""#).unwrap();

    for characteristic in iter {
        match characteristic.name.as_str() {
            "Range" => weapon.range = match dist_re.captures(characteristic.value.as_ref().unwrap()) {
                Some(_) => Some(dist_re.captures(characteristic.value.as_ref().unwrap()).unwrap()[1].parse().unwrap()),
                None => None,
            },
            "Type" => {
                let s = characteristic.value.as_ref().unwrap().split(" ");
                let sv: Vec<&str> = s.collect();

                match *(sv.get(0).unwrap()) {
                    "Assault" => weapon.weapon_type = WeaponType::Assault,
                    "Heavy" => weapon.weapon_type = WeaponType::Heavy,
                    "Rapid" => weapon.weapon_type = WeaponType::RapidFire,
                    "Grenade" => weapon.weapon_type = WeaponType::Grenade,
                    "Pistol" => weapon.weapon_type = WeaponType::Pistol,
                    "Meele" => weapon.weapon_type = WeaponType::Meele,
                    &_ => return Err("Unsupported weapon type".to_owned()),
                }

                match *(sv.get(0).unwrap()) {
                    "Assault" | "Heavy" | "Grenade" | "Pistol" => weapon.attacks = Some(ProfileValue::from_str(sv.get(1).unwrap())),
                    "Rapid" => weapon.attacks = Some(ProfileValue::from_str(sv.get(2).unwrap())),
                    "Meele" => weapon.attacks = None,
                    &_ => return Err("Unsupported weapon type".to_owned()),
                }
            },
            "S" => weapon.strength = characteristic.value.as_ref().unwrap().parse().unwrap(),
            "AP" => weapon.armour_penetration = characteristic.value.as_ref().unwrap().parse().unwrap(),
            "D" => weapon.damage = ProfileValue::from_str(characteristic.value.as_ref().unwrap()),
            "Abilities" => if characteristic.value.as_ref().unwrap() != "-" {
                match weapon.abilities {
                    Some(ref mut f) => f.push(Ability{value: characteristic.value.as_ref().unwrap().to_string()}),
                    None => weapon.abilities = Some(vec![Ability{value: characteristic.value.as_ref().unwrap().to_string()}]),
                }
            },
            _ => return Err("Unknown characteristicfor unit: ".to_owned() + characteristic.name.as_str())
        }
    }

    Ok(weapon)
}

pub fn parse_weapon(weapon_selection: Selection) -> Weapon {
    let profile = parse_weapon_profile(&weapon_selection.profiles.as_ref().unwrap().profiles[0].characteristics);

    Weapon { name: weapon_selection.name, profile: profile.unwrap(), number: weapon_selection.number }
}

// pub fn ParseRoster(roster: Roster) ->` Army {

// }
