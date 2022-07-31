use lazy_static::lazy_static;
use regex::Regex;

/// Types of dice used in the game.
#[derive(Debug, Clone, Copy)]
pub enum DiceType {
    D3,
    D6,
}

/// DiceRoll holds information on how many and what type of dice to roll.
#[derive(Debug, Clone, Copy)]
pub struct DiceRoll {
    pub number: u8,
    pub dice_type: DiceType,
}

/// Value for characteristics that can be a flat value, dice value or both.
#[derive(Debug, Clone, Copy)]
pub struct ProfileValue {
    pub dice_value: Option<DiceRoll>,
    pub flat_value: Option<u8>,
}

impl ProfileValue {
    pub fn from_str(s: &str) -> Self {
        let mut flat_value: Option<u8> = None;
        let mut dice_value: Option<DiceRoll> = None;
        lazy_static! {
            static ref DICE_RE: Regex = Regex::new(r"(\d*)D?(\d?)\+?(\d*)").unwrap();
        }

        let caps = DICE_RE.captures(s).unwrap();
        if caps.get(2).unwrap().as_str() == "" {
            // There is no dice value
            if caps.get(1).unwrap().as_str() != "" {
                // Need to check for this since the string could be just a '*'
                flat_value = Some(caps.get(1).unwrap().as_str().parse().unwrap());
            }
        } else {
            // There is dice value
            flat_value = match caps.get(3).unwrap().as_str() {
                "" => None,
                _ => Some(caps.get(3).unwrap().as_str().parse().unwrap()),
            };

            let number: u8;
            match caps.get(1).unwrap().as_str() {
                "" => number = 1,
                _ => number = caps.get(1).unwrap().as_str().parse().unwrap(),
            }
            dice_value = match caps.get(2).unwrap().as_str() {
                "3" => Some(DiceRoll {
                    dice_type: DiceType::D3,
                    number: number,
                }),
                "6" => Some(DiceRoll {
                    dice_type: DiceType::D6,
                    number: number,
                }),
                _ => None,
            }
        }
        Self {
            dice_value,
            flat_value,
        }
    }
}
