use lazy_static::lazy_static;
use regex::Regex;

use super::ability::{parse_abilities, Ability};
use super::profile::ProfileValue;
use super::ros_parser::ros_parser::{Profile, Selection};
use super::weapon::{Weapon};

/// Profile information from a unit's datasheet.
#[derive(Debug, Clone)]
pub struct ModelProfile {
    pub movement: u8,
    pub min_movement: u8,
    pub weapon_skill: u8,
    pub ballistic_skill: u8,
    pub strength: u8,
    pub toughness: u8,
    pub wounds: u8,
    pub attacks: ProfileValue,
    pub leadership: u8,
    pub save: u8,
}

/// Model information from a unit's datasheet. Includes model profiles.
#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    pub profiles: Vec<ModelProfile>,
    pub weapons: Vec<Weapon>,
    pub number: u8,
    pub keywords: Vec<String>,
}

/// Unit information from a unit's datasheet. Includes models.
#[derive(Debug)]
pub struct Unit {
    pub name: String,
    pub keywords: Vec<String>,
    pub abilities: Vec<Ability>,
    pub invulnable_save: Option<u8>,
    pub models: Vec<Model>,
    pub rules: Vec<String>,
    pub points: f32,
}

impl ModelProfile {
    pub fn from_profile(profile: &Profile) -> Result<Self, String> {
        // Iterator for characteristics
        let iter = profile.characteristics.characteristics.iter();

        // Temp profile to return
        let mut model_profile = Self {
            movement: 0,
            min_movement: 0,
            weapon_skill: 0,
            ballistic_skill: 0,
            strength: 0,
            toughness: 0,
            wounds: 0,
            attacks: ProfileValue {
                dice_value: None,
                flat_value: None,
            },
            leadership: 0,
            save: 0,
        };

        lazy_static! {
            // Regex to parse out '+' from some strings
            static ref UP_RE: Regex = Regex::new(r"(\d)\+").unwrap();
            // Regex to parse distance values
            static ref DIST_RE: Regex = Regex::new(r#"(\d+)-?(\d*)""#).unwrap();
            // Regex to parse range vaues
            static ref RANGE_RE: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
            // Regex to parse bracketed unit profiles since wounds show as N/A
            static ref W_REM_RE: Regex = Regex::new(r"\[(\d+)\] \((\d+)[\+-](\d*) Wounds Remaining\)").unwrap();
        }

        // Iterate through characteristics
        for characteristic in iter {
            // Match on characteristic name
            match characteristic.name.as_str() {
                // Parse movement
                "M" | "Movement" => {
                    // Movement can be a distance or * or -
                    match DIST_RE.captures(characteristic.value.as_ref().unwrap()) {
                        // Get normal value from regex
                        Some(m) => {
                            if m.get(2).unwrap().as_str() == "" {
                                model_profile.movement =
                                    m.get(1).unwrap().as_str().parse().unwrap();
                            } else {
                                model_profile.min_movement =
                                    m.get(1).unwrap().as_str().parse().unwrap();
                                model_profile.movement =
                                    m.get(2).unwrap().as_str().parse().unwrap();
                            }
                        }
                        // Set movement to 0 since either unit cannot move or it is in stat block and we can add from there
                        None => {
                            model_profile.min_movement = 0;
                            model_profile.movement = 0
                        }
                    }
                }
                // Parse weapon skill
                "WS" => {
                    // Weapon skill can be a distance or * or -
                    model_profile.weapon_skill =
                        match UP_RE.captures(characteristic.value.as_ref().unwrap()) {
                            // Get normal value from regex
                            Some(m) => m.get(1).unwrap().as_str().parse().unwrap(),
                            // Set weapon skill to 0 since either unit cannot attack or it is in stat block and we can add from there
                            None => 0,
                        }
                }
                // Parse ballistic skill
                "BS" => {
                    // Ballistic skill can be a distance or * or -
                    model_profile.ballistic_skill =
                        match UP_RE.captures(characteristic.value.as_ref().unwrap()) {
                            // Get normal value from regex
                            Some(m) => m.get(1).unwrap().as_str().parse().unwrap(),
                            // Set ballistic skill to 0 since either unit cannot attack or it is in stat block and we can add from there
                            None => 0,
                        }
                }
                // Parse strength
                "S" => {
                    model_profile.strength = characteristic
                        .value
                        .as_ref()
                        .unwrap()
                        .parse()
                        .map_err(|_| "Profile strength is not a number: ".to_string())?;
                }
                // Parse toughness
                "T" => {
                    model_profile.toughness = match characteristic.value.as_ref().unwrap().parse() {
                        Ok(s) => s,
                        Err(s) => {
                            return Err("Model profile toughness is not a number: ".to_owned()
                                + &s.to_string())
                        }
                    }
                }
                // Parse wounds
                "W" => {
                    model_profile.wounds = match characteristic.value.as_ref().unwrap().parse() {
                        Ok(s) => s,
                        // Wounds can be N/A may be helpful to get remaining wounds from name?
                        Err(s) => {
                            match W_REM_RE.captures(&profile.name) {
                                // capture[1] = order of profile, does this matter?
                                // 2 = first value in range or the value in {int}+ string
                                // 3 = sencond value in range or ""
                                // We want to take higher value of 2.
                                Some(captures) => {
                                    if captures.get(3).unwrap().as_str() == "" {
                                        // We should never get here because when this block matches on profile name with {int}+
                                        // and the wound for that profile will always have max
                                        captures.get(2).unwrap().as_str().parse().unwrap()
                                    } else {
                                        captures.get(3).unwrap().as_str().parse().unwrap()
                                    }
                                }
                                None => {
                                    return Err("Model profile wounds is not a number: ".to_owned()
                                        + &s.to_string())
                                }
                            }
                        }
                    }
                }
                // Parse attacks
                "A" | "Attacks" => {
                    model_profile.attacks =
                        ProfileValue::from_str(characteristic.value.as_ref().unwrap())
                }
                // Parse leadership
                "Ld" => {
                    model_profile.leadership = match characteristic.value.as_ref().unwrap().parse()
                    {
                        Ok(s) => s,
                        Err(s) => {
                            return Err("Model profile leadership is not a number: ".to_owned()
                                + &s.to_string())
                        }
                    }
                }
                // Parse save which has an up value
                "Save" => {
                    model_profile.save =
                        match UP_RE.captures(characteristic.value.as_ref().unwrap()) {
                            Some(captures) => captures.get(1).unwrap().as_str().parse().unwrap(),
                            None => return Err("Remaining W is not a ranged value".to_owned()),
                        }
                }
                // Parse remaining wounds
                // This is a range and we will take the higher number
                "Remaining W" => {
                    model_profile.wounds =
                        match RANGE_RE.captures(characteristic.value.as_ref().unwrap()) {
                            Some(m) => m.get(2).unwrap().as_str().parse().unwrap(),
                            // None case is probably not possible unless Remaining W can be a single value
                            None => return Err("Remaining W is not a ranged value".to_owned()),
                        }
                }
                _ => {
                    return Err("Unknown characteristic for model: ".to_owned()
                        + characteristic.name.as_str())
                }
            }
        }

        Ok(model_profile)
    }
}

impl Model {
    pub fn append_weapon(&mut self, weapon: &Weapon) {
        self.weapons.push(weapon.clone());
    }

    pub fn from_selection(model_selection: &Selection) -> Result<Model, String> {
        let mut weapons: Vec<Weapon> = Vec::new();
        let mut model_profiles: Vec<ModelProfile> = Vec::new();
        let mut keywords: Vec<String> = Vec::new();

        match &model_selection.profiles {
            Some(profiles) => {
                for profile in &profiles.profiles {
                    match profile.type_name.as_str() {
                        // This unwrap is safe because will return profile or Err
                        "Unit" => {
                            model_profiles.push(ModelProfile::from_profile(&profile).unwrap())
                        }
                        "Abilities" => (),
                        "Explosion" => (), //TODO: whatever to do with this
                        _ => {
                            return Err("Unknown type name for profile: ".to_owned()
                                + profile.type_name.as_str())
                        }
                    }
                }
            }
            // No model profile?
            None => return Err("Model does not have a profile".to_owned()),
        };

        for selection in &model_selection.selections.as_ref().unwrap().selections {
            // This covers cases where the weapons are in another selection.
            if !selection.selections.is_none() {
                for s in &selection.selections.as_ref().unwrap().selections {
                    match s.profiles.as_ref().unwrap().profiles.get(0) {
                        Some(profile) => match profile.type_name.as_str() {
                            "Abilities" => (),
                            "Weapon" => weapons.push(Weapon::from_selection(&s).unwrap()),
                            _ => {
                                return Err(
                                    "Unknown profile type in selection selection.".to_owned()
                                )
                            }
                        },
                        None => return Err("Selection did not have a profile".to_owned()),
                    }
                }
            }

            // This covers keywords (i.e. Warlord) and makes sure not the throw errors below
            // Should these keywords get bubbled up to unit?
            if selection.profiles.is_none() {
                match &selection.categories {
                    Some(categories) => {
                        for category in &categories.categories {
                            keywords.push(category.name.clone());
                        }
                    },
                    None => ()
                }
                continue;
            }

            match selection.profiles.as_ref().unwrap().profiles.get(0) {
                Some(profile) => {
                    match profile.type_name.as_str() {
                        "Abilities" => (),
                        // Push weapon
                        "Weapon" => weapons.push(Weapon::from_selection(&selection).unwrap()),
                        // There are probably more of this type
                        "Stat Damage - M/BS/A" => {
                            let orig_profile = model_profiles.pop().unwrap();
                            for profile in &selection.profiles.as_ref().unwrap().profiles {
                                let tmp_profile = ModelProfile::from_profile(&profile).unwrap();
                                model_profiles.push(ModelProfile {
                                    movement: orig_profile.movement + tmp_profile.movement,
                                    min_movement: orig_profile.min_movement
                                        + tmp_profile.min_movement,
                                    weapon_skill: orig_profile.weapon_skill
                                        + tmp_profile.weapon_skill,
                                    ballistic_skill: orig_profile.ballistic_skill
                                        + tmp_profile.ballistic_skill,
                                    strength: orig_profile.strength,
                                    toughness: orig_profile.toughness,
                                    wounds: tmp_profile.wounds,
                                    attacks: {
                                        // We can assume that if both dice damage and flat damage are None, then the attacks come from tmp
                                        // Also since this block is for stat damage: M/BS/A but still good to have check
                                        if orig_profile.attacks.dice_value.is_none()
                                            && orig_profile.attacks.flat_value.is_none()
                                        {
                                            tmp_profile.attacks
                                        } else {
                                            orig_profile.attacks
                                        }
                                    },
                                    leadership: orig_profile.leadership,
                                    save: orig_profile.save,
                                })
                            }
                        }
                        _ => return Err("Unknown profile type in selection.".to_owned()),
                    }
                }
                None => return Err("Selection did not have a profile".to_owned()),
            }
        }

        // Go through weapons and check if there are multiple enries of the same weapon
        let mut unmultiple_weapons: Vec<Weapon> = Vec::new();
        'outer: for i in 0..weapons.len() {
            for k in &unmultiple_weapons {
                if weapons[i].name == k.name {
                    continue 'outer;
                }
            }

            let mut temp = weapons[i].clone();

            for j in i + 1..weapons.len() {
                if weapons.get(i).unwrap().name == weapons.get(j).unwrap().name {
                    temp.number = temp.number + weapons[j].number;
                }
            }
            unmultiple_weapons.push(temp);
        }

        // Create and return model
        Ok(Self {
            name: model_selection.name.to_owned(),
            profiles: model_profiles,
            weapons: unmultiple_weapons,
            number: model_selection.number,
            keywords,
        })
    }
}

impl Unit {
    fn get_total_points(unit_selection: &Selection) -> f32 {
        let mut points: f32 = 0.0;

        match &unit_selection.selections {
            Some(selections) => {
                for selection in &selections.selections {
                    points += Self::get_total_points(&selection);
                }
            },
            None => (),
        }
        

        match &unit_selection.costs {
            Some(costs) => {
                for cost in &costs.costs {
                    if cost.name == "pts" {
                        points += cost.value;
                    }
                }
            },
            None => (),
        }

        points
    }

    pub fn from_selection(unit_selection: &Selection) -> Result<Self, String> {
        let mut keywords: Vec<String> = Vec::new();
        let mut rules: Vec<String> = Vec::new();
        let mut unit_weapons: Vec<Weapon> = Vec::new();
        let mut models: Vec<Model> = Vec::new();

        // Some units have weapons in the unit 
        match &unit_selection.profiles {
            Some(profiles) => {
                for profile in &profiles.profiles {
                    match profile.type_name.as_str() {
                        "Weapon" => {
                            unit_weapons.push(Weapon::from_profile(&profile).unwrap());
                        },
                        "Unit" => (),
                        "Abilities" => (),
                        _ => return Err("Unknown unit profile type: ".to_string() + &profile.type_name)
                    }
                }
            },
            None => (),
        }

        // Parse on the selection type
        match unit_selection.r#type.as_str() {
            // Selection is the model which has selections of only weapons and abilities
            "model" => models.push(Model::from_selection(unit_selection).unwrap()),
            // Selection is the unit which has selections of models
            "unit" => {
                for model in &unit_selection.selections.as_ref().unwrap().selections {
                    match model.profiles {
                        // Push model into unit
                        Some(_) => {
                            let mut m = Model::from_selection(&model).unwrap();
                            for weapon in &unit_weapons {
                                m.append_weapon(weapon);
                            }
                            models.push(m);

                        },
                        None => (),
                    }
                    // Get more keywords from the selections
                    match &model.categories {
                        Some(categories) => {
                            for category in &categories.categories {
                                keywords.push(category.name.to_owned());
                            }
                        }
                        None => (),
                    }
                    // Get rules from models
                    match &model.rules {
                        Some(r) => {
                            for rule in &r.categories {
                                rules.push(rule.name.to_owned());
                            }
                        }
                        None => (),
                    }
                }
            }
            _ => {
                return Err(
                    "Unknown type name for unit: ".to_owned() + unit_selection.r#type.as_str()
                )
            }
        };

        for category in &unit_selection.categories.as_ref().unwrap().categories {
            keywords.push(category.name.to_owned())
        }

        keywords.sort_unstable();
        keywords.dedup();

        rules.sort_unstable();
        rules.dedup();

        Ok(Unit {
            name: unit_selection.name.to_owned(),
            abilities: parse_abilities(unit_selection),
            keywords,
            invulnable_save: None,
            models,
            rules,
            points: Self::get_total_points(unit_selection),
        })
    }
}
