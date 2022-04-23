extern crate ini;
use ini::Ini;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CuttedClip {
    pub id: String,
    pub start_time: String,
    pub end_time: String,
    pub comment: String,
    pub file_belong: String,
}

pub fn parse_file(path: String) -> Vec<CuttedClip> {
    let conf = Ini::load_from_file(path).unwrap();
    let mut cutted_clips = Vec::new();
    for (sec, prop) in &conf {
        let file_name = match sec {
            Some(sec) => sec,
            None => "error",
        };
        for a in 0..prop.len() / 2 {
            let start_postion = match prop.get(format!("{}_begin", a)) {
                Some(start_postion) => start_postion,
                None => "error",
            };

            let comment = match prop.get(format!("{}_comment", a)) {
                Some(comment) => comment,
                None => "error",
            };

            let end_postion = match prop.get(format!("{}_end", a)) {
                Some(end_postion) => end_postion,
                None => "error",
            };
            let clip = CuttedClip {
                id: (a as u16).to_string(),
                start_time: start_postion.to_string(),
                end_time: end_postion.to_string(),
                comment: comment.to_string(),
                file_belong: file_name.to_string(),
            };
            cutted_clips.push(clip);
        }
    }

    cutted_clips
}
