use std::error::Error;

use crate::ros_parser;

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
    pub number: u8,
    pub dice_type: DiceType,
}

#[derive(Debug)]
pub struct Profile {
    pub movement: u8,
    pub weapon_skill: u8,
    pub ballistic_skill: u8,
    pub strength: u8,
    pub toughness: u8,
    pub wounds: u8,
    pub attacks: u8,
    pub leadership: u8,
    pub save: u8,
}

#[derive(Debug)]
pub struct Ability {
    // Need to implement this later to actually change other things
    pub value: String,
}

#[derive(Debug)]
pub struct Weapon {
    pub abilities: Vec<Ability>,
    pub range: Option<u8>,
    pub weapon_type: WeaponType,
    pub dice_attacks: Option<DiceRoll>,
    pub flat_attacks: Option<u8>,
    pub strength: u8,
    pub armour_penetration: u8,
    pub dice_damage: Option<DiceRoll>,
    pub flat_damage: u8,
    pub number: u8,
}

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub profiles: Vec<Profile>,
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

pub fn ParseProfile(characteristics: &ros_parser::Characteristics) -> Result<Profile, String> {
    let iter = characteristics.characteristics.iter();
    let mut profile = Profile {
        movement: 0,
        weapon_skill: 0,
        ballistic_skill: 0,
        strength: 0,
        toughness: 0,
        wounds: 0,
        attacks: 0,
        leadership: 0,
        save: 0,
    };

    for characteristic in iter {
        match characteristic.name.as_str() {
            "M" => profile.movement = characteristic.value.as_ref().unwrap().parse().unwrap(), // todo error checking
            "WS" => profile.weapon_skill = characteristic.value.as_ref().unwrap().parse().unwrap(),
            "BS" => profile.ballistic_skill = characteristic.value.as_ref().unwrap().parse().unwrap(),
            "S" => profile.strength = characteristic.value.as_ref().unwrap().parse().unwrap(),
            "T" => profile.toughness = characteristic.value.as_ref().unwrap().parse().unwrap(),
            "W" => profile.wounds = characteristic.value.as_ref().unwrap().parse().unwrap(),
            "A" => profile.attacks = characteristic.value.as_ref().unwrap().parse().unwrap(),
            "Ld" => profile.leadership = characteristic.value.as_ref().unwrap().parse().unwrap(),
            "Save" => profile.save = characteristic.value.as_ref().unwrap().parse().unwrap(),
            _ => return Err("Unknown characteristicfor unit: ".to_owned() + characteristic.name.as_str())
        }
    }

    Ok(profile)
}

// pub fn ParseRoster(roster: Roster) ->` Army {

// }
