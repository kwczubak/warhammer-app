use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Roster {
    pub id: String,
    pub name: String,
    pub battle_scribe_version: String,
    pub game_system_id: String,
    pub game_system_name: String,
    pub game_system_revision: String,
    pub costs: Costs,
    pub forces: Forces
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Costs {
    #[serde(rename = "$value")]
    pub costs: Vec<Cost>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Forces{
    #[serde(rename = "$value")]
    pub forces: Vec<Force>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    pub name: String,
    pub type_id: String,
    pub value: f32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Force {
    pub id: String,
    pub name: String,
    pub entry_id: String,
    pub catalogue_id: String,
    pub catalogue_revision: String,
    pub catalogue_name: String,
    pub selections: Selections,
    pub publications: Publications,
    pub rules: Option<Rules>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Selections {
    #[serde(rename = "$value")]
    pub selections: Vec<Selection>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Selection {
    pub id: String,
    pub name: String,
    pub entry_id: String,
    pub number: u8,
    pub r#type: String,
    pub entry_group_id: Option<String>,
    pub publication_id: Option<String>,
    pub selections: Option<Selections>,
    pub costs: Option<Costs>,
    pub profiles: Option<Profiles>,
    pub categories: Option<Categories>,
    pub rules: Option<Rules>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Profiles {
    #[serde(rename = "$value")]
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub type_id: String,
    pub type_name: String,
    pub characteristics: Characteristics,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Characteristics {
    #[serde(rename = "$value")]
    pub characteristics: Vec<Characteristic>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Characteristic {
    pub name: String,
    pub type_id: String,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Categories {
    #[serde(rename = "$value")]
    pub categories: Vec<Category>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub name: String,
    pub entry_id: String,
    pub primary: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Rules {
    #[serde(rename = "$value")]
    pub categories: Vec<Rule>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub publication_id: Option<String>,
    pub page: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Publications {
    #[serde(rename = "$value")]
    pub publications: Vec<Publication>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    pub id: String,
    pub name: String,
}
