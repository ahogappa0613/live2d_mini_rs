#![allow(non_snake_case)]

use serde::*;

#[derive(Debug, Deserialize)]
pub struct PoseJson {
    pub Type: String,
    pub FadeInTime: f32,
    pub Groups: Vec<Vec<Group>>,
}

#[derive(Debug, Deserialize)]

pub struct Group {
    pub Id: String,
    pub Link: Vec<String>,
}
