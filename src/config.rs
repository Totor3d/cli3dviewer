use crate::screen::ScreenPosition;
use crate::vectors::Vector3;
use std::error::Error;



#[derive(Debug)]
pub enum ParseErrors {
    NumberParseError,
    NotEnoughArgumetAfterKey,
    NoModelPath,
}

pub struct Config {
    pub model_path : String,
    pub scale : f64,
    pub screen_offset : ScreenPosition,
    pub position : Vector3,
    pub delay_millis : u64,
    pub rotation : Vector3,
    pub white_char : char,
}
impl Config {
    pub fn from_args(args : Vec<String>) -> Result<Self, ParseErrors> {
        if args.len() < 2 {
            return Err(ParseErrors::NoModelPath);
        }
        let model_path = args[1].clone();
        let mut scale : f64 = 2.0;
        let mut screen_offset = ScreenPosition{x: 0, y: 0};
        let mut position : Vector3 = Vector3::new(0f64, 0f64, 3f64);
        let mut delay_millis : u64 = 30;
        let mut rotation : Vector3 = Vector3::new(0.02, 0.03, 0.01);
        let mut white_char : char = '#';
        for i in 2..args.len() {
            if args[i].chars().next().unwrap() == '-' {
                match args[i].as_str() {
                    "-s" => {
                        if args.len()-1<i+1 {
                            return Err(ParseErrors::NotEnoughArgumetAfterKey);
                        }
                        scale = {match args[i+1].parse() {Ok(a) => a, Err(_)=>return Err(ParseErrors::NumberParseError)}}
                    }
                    "-o" => {
                        if args.len()-1<i+2 {
                            return Err(ParseErrors::NotEnoughArgumetAfterKey);
                        }
                        screen_offset.x = {match args[i+1].parse() {Ok(a) => a, Err(_)=>return Err(ParseErrors::NumberParseError)}};
                        screen_offset.y = {match args[i+2].parse() {Ok(a) => a, Err(_)=>return Err(ParseErrors::NumberParseError)}}
                    }
                    "-p" => {
                        if args.len()-1<i+3 {
                            return Err(ParseErrors::NotEnoughArgumetAfterKey);
                        }
                        position.x = {match args[i+1].parse() {Ok(a) => a, Err(_)=>return Err(ParseErrors::NumberParseError)}};
                        position.y = {match args[i+2].parse() {Ok(a) => a, Err(_)=>return Err(ParseErrors::NumberParseError)}};
                        position.z = {match args[i+3].parse() {Ok(a) => a, Err(_)=>return Err(ParseErrors::NumberParseError)}}
                    }
                    "-d" => {
                        if args.len()-1<i+1 {
                            return Err(ParseErrors::NotEnoughArgumetAfterKey);
                        }
                        delay_millis = {match args[i+1].parse() {Ok(a) => a, Err(_)=>return Err(ParseErrors::NumberParseError)}}
                    }
                    "-r" => {
                        if args.len()-1<i+3 {
                            return Err(ParseErrors::NotEnoughArgumetAfterKey);
                        }
                        rotation.x = {match args[i+1].parse() {Ok(a) => a, Err(_)=>return Err(ParseErrors::NumberParseError)}};
                        rotation.y = {match args[i+2].parse() {Ok(a) => a, Err(_)=>return Err(ParseErrors::NumberParseError)}};
                        rotation.z = {match args[i+3].parse() {Ok(a) => a, Err(_)=>return Err(ParseErrors::NumberParseError)}}
                    }
                    "-c" => {
                        if args.len()-1<i+1 {
                            return Err(ParseErrors::NotEnoughArgumetAfterKey);
                        }
                        white_char = args[i+1].chars().next().unwrap()
                    }
                    _ => ()
                }
            } 
        }
        Ok(Config { model_path, scale: scale, screen_offset: screen_offset, position: position, delay_millis: delay_millis, rotation: rotation, white_char : white_char})
    }
}