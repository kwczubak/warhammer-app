use druid::{piet::TextStorage, text::selection};

use super::{
    ability::Ability,
    army::{Army, Detachment},
    profile,
    psyker::{PsychicPower, PsykerProfile},
    ros_parser::ros_parser::{Category, Force, Profile, Profiles, Roster, Selection},
    unit::{Model, ModelProfile, Unit},
    weapon::Weapon,
};

pub fn army_from_roster(roster: &Roster) -> Result<Army, String> {
    let mut cp = 0.0;
    let mut pl = 0.0;
    let mut pts = 0.0;
    let mut detachments: Vec<Detachment> = Vec::new();

    // Get data from costs
    for cost in &roster.costs.costs {
        match cost.type_id.as_str() {
            "e356-c769-5920-6e14" => pl = cost.value,
            "2d3b-b544-ad49-fb75" => cp = cost.value,
            "points" => pts = cost.value,
            _ => return Err("Unknown cost type from roster.".to_string()),
        }
    }

    // Get detachements from forces
    for force in &roster.forces.forces {
        detachments.push(detachment_from_force(force)?);
    }

    let name: String = roster.name.to_owned();

    Ok(Army {
        detachments,
        cp,
        pl,
        pts,
        name,
    })
}

pub fn detachment_from_force(force: &Force) -> Result<Detachment, String> {
    // Forces only have selections and publications
    // We do not care about publications since they are only references

    let mut abilities: Vec<Ability> = Vec::new();
    let mut units: Vec<Unit> = Vec::new();

    for selection in &force.selections.selections {
        let mut categories: Vec<String> = Vec::new();

        // A selection can be an upgrade, unit or model types.
        match selection.r#type.as_str() {
            "upgrade" => {
                // This can be configurations, strategems or abilities. The way to check is by categories
                if selection.categories.is_some() {
                    for category in &selection.categories.as_ref().unwrap().categories {
                        categories.push(category.name.to_owned());
                    }
                } else {
                    // What will this be? Ability?
                    return Err(
                        "Unknown upgrade selection which doesnt have a category".to_string()
                    );
                }

                // Check if this selection is a configuration
                if categories.contains(&"Configuration".to_string()) {
                    match categories.get(0).unwrap().as_str() {
                        "Configuration" => {
                            // Configuration can have an ability
                            if selection.profiles.is_some() {
                                // Configurations shouldn't have profiles but if it does then it needs to be implemented
                                return Err(
                                    "Profiles in configurations are not implemented.".to_string()
                                );
                            }

                            match &selection.selections {
                                // The selection may be an ability or not.
                                Some(selections) => {
                                    for inner_selection in &selections.selections {
                                        match inner_selection.r#type.as_str() {
                                            // Only matching here to get extra functionality later
                                            "upgrade" => {
                                                // Most likely this is an ability
                                                // Can there be another selection here?
                                                match &inner_selection.profiles {
                                                    Some(profiles) => {
                                                        // If there is a profile it can be an ability
                                                        for profile in &profiles.profiles {
                                                            match  profile.type_name.as_str() {
                                                                "Abilities" | "Allegiance Oath" => abilities.push(Ability::from_profile(profile)?),
                                                                // Keeping this to implement later
                                                                _ => return Err("Unimplemented profile in inner selection for configuration: ".to_string() + &profile.type_name)
                                                            }
                                                        }
                                                    }
                                                    // We probably don't care about this if it doesn't have a profile
                                                    None => (),
                                                }
                                            }
                                            _ => {
                                                return Err(
                                                    "Unknown inner selection type".to_string()
                                                )
                                            }
                                        }
                                    }
                                }
                                // Dont really care about this
                                // Examples are detachment cost or battle size
                                None => (),
                            }
                        }
                        _ => {
                            return Err("Unknown category for configuration detachment selection"
                                .to_string())
                        }
                    }
                }

                // Check if this selection is a strategem
                if categories.contains(&"Stratagems".to_string()) {
                    // Strategems may have a profile which hcan contain an ability
                    match &selection.profiles {
                        Some(profiles) => {
                            for profile in &profiles.profiles {
                                match  profile.type_name.as_str() {
                                    "Abilities" | "Allegiance Oath" => abilities.push(Ability::from_profile(profile)?),
                                    // Keeping this to implement later
                                    _ => return Err("Unimplemented profile in inner selection for configuration: ".to_string() + &profile.type_name)
                                }
                            }
                        }
                        // We do not care about strategems that do not have a profile
                        None => (),
                    }

                    if selection.selections.is_some() {
                        // Configurations shouldn't have profiles but if it does then it needs to be implemented
                        return Err("Selectios in strategems are not implemented.".to_string());
                    }
                }
            }
            "model" => units.push(Unit {
                name: selection.name.to_owned(),
                models: vec![model_from_selecton(selection)?],
                points: Unit::get_total_points(selection),
            }),
            "unit" => {}
            _ => return Err("Unknown selection type from force".to_string()),
        }
    }

    Ok(Detachment {
        name: force.name.to_owned(),
        abilities,
        units,
    })
}

// pub struct Unit {
//     pub name: String,
//     pub keywords: Vec<String>,
//     pub abilities: Vec<Ability>,
//     pub invulnable_save: Option<u8>,
//     pub models: Vec<Model>,
//     pub rules: Vec<String>,
//     pub points: f32,
// }

// pub struct Model {
//     pub name: String,
//     pub profiles: Vec<ModelProfile>,
//     pub weapons: Vec<Weapon>,
//     pub number: u8,
//     pub keywords: Vec<String>,
//     pub psyker_profile: Option<PsykerProfile>,
//     pub psyker_powers: Option<Vec<PsychicPower>>,
// }

pub fn model_from_selecton(selection: &Selection) -> Result<Model, String> {
    let mut profiles: Vec<ModelProfile> = Vec::new();
    let mut weapons: Vec<Weapon> = Vec::new();
    let mut keywords: Vec<String> = Vec::new();
    let mut psyker_powers: Option<Vec<PsychicPower>> = None;
    let mut psyker_profile: Option<PsykerProfile> = None;
    let mut abilities: Option<Vec<Ability>> = None;

    // Model can be either a unit of 1 or multiple models that are the same in a singular unit
    // Model keywords come from the categories
    match &selection.categories {
        Some(categories) => {
            for category in &categories.categories {
                keywords.push(category.name.to_owned());
            }
        }
        // There should always be keywords for models
        None => return Err("No categories (keywords) for model selection".to_string()),
    }

    if selection.rules.is_some() {
        // For now just add it to the abilities without a value
        for rule in &selection.rules.as_ref().unwrap().categories {
            match abilities {
                // Push new ability if vector has already been created
                Some(ref mut f) => f.push(Ability {
                    value: "".to_string(),
                    name: rule.name.to_owned(),
                }),
                // Create vector and add ability to it since abilities is None and wants to stay None if there isn't any
                None => {
                    abilities = Some(vec![Ability {
                        value: "".to_string(),
                        name: rule.name.to_owned(),
                    }])
                }
            }
        }
    }

    match &selection.profiles {
        Some(selection_profiles) => {
            // Profiles can be the unit profile, psyker profile or ability
            for profile in &selection_profiles.profiles {
                match profile.type_name.as_str() {
                    "Unit" => {
                        profiles.push(ModelProfile::from_profile(profile)?);
                    }
                    "Psyker" => {
                        psyker_profile = Some(PsykerProfile::from_profile(profile)?);
                    }
                    "Abilities" => {
                        // Is there a better way to do this?
                        // Don't like the fact that there are 2 unwraps
                        match abilities {
                            // Push new ability if vector has already been created
                            Some(ref mut f) => f.push(Ability {
                                value: profile
                                    .characteristics
                                    .characteristics
                                    .get(0)
                                    .unwrap()
                                    .value
                                    .as_ref()
                                    .unwrap()
                                    .to_string(),
                                name: profile.name.to_owned(),
                            }),
                            // Create vector and add ability to it since abilities is None and wants to stay None if there isn't any
                            None => {
                                abilities = Some(vec![Ability {
                                    value: profile
                                        .characteristics
                                        .characteristics
                                        .get(0)
                                        .unwrap()
                                        .value
                                        .as_ref()
                                        .unwrap()
                                        .to_string(),
                                    name: profile.name.to_owned(),
                                }])
                            }
                        }
                    }
                    _ => return Err("Unhandled profile type ".to_string() + &profile.type_name),
                }
            }
        }
        // This probably can't happen but adding error to implement if it can
        None => return Err("No Profile for model".to_string() + &selection.name),
    }

    // If there isn't any selections then we don't need to do anything
    if selection.selections.is_some() {
        for model_selection in &selection.selections.as_ref().unwrap().selections {
            // Only doing match here to check if there will be other types
            match model_selection.r#type.as_str() {
                "upgrade" => {
                    // Upgrades can be weapons, psychic powers or abilities
                    match &model_selection.profiles {
                        Some(profiles) => {
                            for profile in &profiles.profiles {
                                match profile.type_name.as_str() {
                                    "Weapon" => weapons.push(Weapon::from_profile(profile)?),
                                    "Psychic Power" => match psyker_powers {
                                        Some(ref mut f) => f.push(PsychicPower::from_profile(profile)?),
                                        None => {
                                            psyker_powers = Some(vec![PsychicPower::from_profile(profile)?])
                                        }
                                    },
                                    "Abilities" => match abilities {
                                        // Push new ability if vector has already been created
                                        Some(ref mut f) => f.push(Ability {
                                            value: profile
                                                .characteristics
                                                .characteristics
                                                .get(0)
                                                .unwrap()
                                                .value
                                                .as_ref()
                                                .unwrap()
                                                .to_string(),
                                            name: profile.name.to_owned(),
                                        }),
                                        // Create vector and add ability to it since abilities is None and wants to stay None if there isn't any
                                        None => {
                                            abilities = Some(vec![Ability {
                                                value: profile
                                                    .characteristics
                                                    .characteristics
                                                    .get(0)
                                                    .unwrap()
                                                    .value
                                                    .as_ref()
                                                    .unwrap()
                                                    .to_string(),
                                                name: profile.name.to_owned(),
                                            }])
                                        }
                                    },
                                    _ => return Err("Unknown model profile type ".to_string() + &profile.type_name)
                                }
                            }
                        },
                        None => return Err("Model selection did not have a profile: ".to_string() + &model_selection.name),
                    }
                },
                _ => return Err("Unknown model selection ".to_string() + &model_selection.r#type)
            }
        }
    }

    Ok(Model {
        name: selection.name.to_owned(),
        profiles,
        weapons,
        number: selection.number,
        keywords,
        psyker_profile,
        psyker_powers,
        abilities,
        invulnable_save: None,
    })
}
