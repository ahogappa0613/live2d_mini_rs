#![allow(non_snake_case)]

use serde::*;

#[derive(Debug, Deserialize)]
pub struct MotionJson {
    pub Version: i32,
    pub Meta: Meta,
    pub Curves: Vec<Curve>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub Duration: f32,
    pub Fps: f32,
    pub Loop: bool,
    pub AreBeziersRestricted: bool,
    pub CurveCount: usize,
    pub TotalSegmentCount: i32,
    pub TotalPointCount: i32,
    pub UserDataCount: i32,
    pub TotalUserDataSize: i32,
}

#[derive(Debug, Deserialize)]
pub struct Curve {
    pub Target: String,
    pub Id: String,
    pub Segments: Vec<f32>,
}


