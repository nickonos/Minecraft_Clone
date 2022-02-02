use bincode;
use serde::{Serialize, Deserialize};

use winit::event::VirtualKeyCode;
use std::fs;
use std::path::{Path, PathBuf};
use std::fs::File;
use winapi::shared::basetsd::INT8;
use std::io::Write;
use std::env;

#[derive(Serialize, Deserialize)]
pub struct KeyMappings{
    pub forward : VirtualKeyCode,
    pub left : VirtualKeyCode,
    pub backward : VirtualKeyCode,
    pub right : VirtualKeyCode,
    pub menu : VirtualKeyCode
}

impl KeyMappings{
    pub fn default()-> KeyMappings{
        KeyMappings{
            forward: VirtualKeyCode::W,
            left: VirtualKeyCode::A,
            backward: VirtualKeyCode::S,
            right: VirtualKeyCode::D,
            menu: VirtualKeyCode::Escape,
        }
    }

    pub fn write_to_file(&self){
        let mut file : File;

        let home = env::var("USERPROFILE")
            .expect("Home environment variable not found");

        let dir = home + "/rustcraft";
        if !Path::new(&dir).exists(){
            fs::create_dir(&dir);
        }
        let path = dir + "/settings.dat";

        if !Path::new(&path).exists(){
            file = File::create(path)
                .expect("Unable to create key mapping file");
        }else {
            file = File::open(path)
                .expect("Unable to open key mapping file")
        }


        let content = bincode::serialize(&self).unwrap();
        file.write_all(content.as_slice())
            .expect("Could not write to file")
    }

    pub fn read_from_file() -> Option<KeyMappings> {
        let home = env::var("USERPROFILE")
            .expect("Home environment variable not found");

        let path = home + "/rustcraft/settings.dat";

        if !Path::new(&path).exists(){
            return None
        }

        let content = fs::read(path)
            .expect("unable to read keymappings");

        Some(bincode::deserialize(&content).unwrap())
    }
}