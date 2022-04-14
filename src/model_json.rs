#![allow(non_snake_case)]
use serde::*;

#[derive(Debug, Deserialize)]
pub struct ModelJson {
    pub Version: i32,
    pub FileReferences: FileReferences,
    pub Groups: Option<Vec<Group>>,
    pub HitAreas: Option<Vec<HitArea>>,
}

#[derive(Debug, Deserialize)]
pub struct FileReferences {
    pub Moc: String,
    pub Textures: Vec<String>,
    pub Physics: Option<String>,
    pub Pose: Option<String>,
    pub UserData: Option<String>,
    pub DisplayInfo: String,
    pub Motions: Option<Motions>,
}

#[derive(Debug, Deserialize)]
pub struct Motions {
    pub Idle: Vec<Motion>,
    pub TapBody: Vec<Motion>,
}

#[derive(Debug, Deserialize)]
pub struct Motion {
    pub File: String,
    pub FadeInTime: f32,
    pub FadeOutTime: f32,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub Target: String,
    pub Name: String,
    pub Ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct HitArea {
    pub Id: String,
    pub Name: String,
}
