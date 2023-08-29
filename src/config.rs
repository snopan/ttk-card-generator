use serde::{Deserialize, Serialize};
use std::{fs::File, io::{BufReader, Read, self}};

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    pub name: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub name: String,
    pub picture: String,
    pub health: usize,
    pub monarch: bool,
    pub male: bool,
    pub kingdom: String,
    pub skills: Vec<Skill>
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub assets: Assets,
    pub characters: Vec<Character>
}

#[derive(Debug)]
pub enum Error {
    OpenFileFail(io::Error),
    ReadToStringFail(io::Error),
    ConvertJsonFail(serde_json::Error),
}

pub fn load_config(path: String) -> Result<Config, Error> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(error) => return Err(Error::OpenFileFail(error))
    };

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    match buf_reader.read_to_string(&mut contents) {
        Err(error) => return Err(Error::ReadToStringFail(error)),
        _ => (),
    };


    let config: Config = match serde_json::from_str(contents.as_str()) {
        Ok(c) => c,
        Err(error) => return Err(Error::ConvertJsonFail(error))
    };

    Ok(config)
}