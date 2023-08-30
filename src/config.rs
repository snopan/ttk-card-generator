use serde::{Deserialize, Serialize, de};
use std::{fs::File, io::{BufReader, self}, collections::HashMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    pub name: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub name: String,
    pub health: u32,
    pub monarch: bool,
    pub male: bool,
    pub kingdom: String,
    pub skills: Vec<Skill>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Characters {
    pub list: Vec<Character>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Health {
    pub kingdomless: String,
    pub shu: String,
    pub wei: String,
    pub wu: String,
    pub zhu: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Frames {
    pub kingdomless_zhu: String,
    pub kingdomless: String,
    pub shu_zhu: String,
    pub shu: String,
    pub wei_zhu: String,
    pub wei: String,
    pub wu_zhu: String,
    pub wu: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Assets {
    pub health: Health,
    pub frames: Frames,
    pub characters: HashMap<String, String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NameOutlineColors {
    pub zhu: [u8; 4],
    pub kingdomless: [u8; 4],
    pub shu: [u8; 4],
    pub wei: [u8; 4],
    pub wu: [u8; 4],
}
pub type SkillBoxColors = NameOutlineColors;
#[derive(Serialize, Deserialize, Debug)]
pub struct Styles {
    pub name_outline_colors: NameOutlineColors,
    pub skill_box_colors: SkillBoxColors
}

#[derive(Debug)]
pub enum Error {
    OpenFileFail(io::Error),
    ConvertJsonFail(serde_json::Error),
}

pub fn load_characters(path: String) -> Result<Characters, Error> {
    open_and_load_json::<Characters>(path)
}

pub fn load_assets(path: String) -> Result<Assets, Error> {
    open_and_load_json::<Assets>(path)
}

pub fn load_styles(path: String) -> Result<Styles, Error> {
    open_and_load_json::<Styles>(path)
}

fn open_and_load_json<T>(path: String) -> Result<T, Error> 
where T: Serialize + de::DeserializeOwned 
{
    let file = match File::open(path) {
        Ok(f) => f,
        Err(error) => return Err(Error::OpenFileFail(error))
    };

    let buf_reader = BufReader::new(file);

    let config: T = match serde_json::from_reader::<_, T>(buf_reader) {
        Ok(c) => c,
        Err(error) => return Err(Error::ConvertJsonFail(error))
    };

    Ok(config)
}

