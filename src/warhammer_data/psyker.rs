use super::ros_parser::ros_parser::Profile;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct PsychicPower {
    pub name: String,
    pub warp_charge: u8,
    pub range: u8,
    pub details: String,
}

#[derive(Debug, Clone)]
pub struct PsykerProfile {
    pub num_casts: u8,
    pub num_deny: u8,
    // Maybe add this later. Dont think it is neccassary because battlecribe makes sure there isnt more powers
    // pub powers_known: u8,
}

impl PsykerProfile {
    pub fn from_profile(profile: &Profile) -> Result<Self, String> {
        let mut num_casts = 0;
        let mut num_deny = 0;
        for characteristic in &profile.characteristics.characteristics {
            match characteristic.name.as_str() {
                "Cast" => num_casts = characteristic.value.as_ref().unwrap().parse().unwrap(),
                "Deny" => num_deny = characteristic.value.as_ref().unwrap().parse().unwrap(),
                "Powers Known" => (),
                "Other" => {
                    if characteristic.value.is_some() {
                        return Err("Other not implemented for pysker profile".to_string());
                    }
                }
                _ => {
                    return Err(
                        "Unknown psyker profile characteristic ".to_string() + &characteristic.name
                    )
                }
            }
        }

        Ok(Self {
            num_casts,
            num_deny,
        })
    }
}

impl PsychicPower {
    pub fn from_profile(profile: &Profile) -> Result<Self, String> {
        let mut range = 0;
        let mut warp_charge = 0;
        let mut details = "".to_string();

        lazy_static! {
            // Regex to parse out distance value
            // There is a question mark after the " because lasguns are stupid
            static ref DIST_RE: Regex = Regex::new(r#"(\d+)"?"#).unwrap();
        }

        for characteristic in &profile.characteristics.characteristics {
            match characteristic.name.as_str() {
                "Range" => {
                    range = match DIST_RE.captures(characteristic.value.as_ref().unwrap()) {
                        Some(_) => DIST_RE
                            .captures(characteristic.value.as_ref().unwrap())
                            .unwrap()[1]
                            .parse()
                            .unwrap(),
                        None => 0,
                    }
                }
                "Warp Charge" => warp_charge = characteristic.value.as_ref().unwrap().parse().unwrap(),
                "Details" => details = characteristic.value.as_ref().unwrap().to_string(),
                _ => {
                    return Err("Unknown characteristic in psychic power".to_string()
                        + &characteristic.name)
                }
            }
        }

        Ok(Self {
            range,
            warp_charge,
            details,
            name: profile.name.to_owned(),
        })
    }
}
