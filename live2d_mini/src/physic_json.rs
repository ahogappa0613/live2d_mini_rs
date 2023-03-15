#![allow(non_snake_case)]

use serde::*;

#[derive(Debug, Deserialize)]
pub struct PhysicJson {
    pub Version: i32,
    pub Meta: Meta,
    /// このvectorの数はSubRigCountとして扱われる
    pub PhysicsSettings: Vec<PhysicsSetting>,
}

#[derive(Debug, Deserialize)]
pub struct Vec2 {
    pub X: f32,
    pub Y: f32,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub PhysicsSettingCount: i32,
    pub TotalInputCount: i32,
    pub TotalOutputCount: i32,
    pub VertexCount: i32,
    pub EffectiveForces: EffectiveForces,
    pub PhysicsDictionary: Vec<PhysicsDiction>,
    pub Fps: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct EffectiveForces {
    pub Gravity: Vec2,
    pub Wind: Vec2,
}

#[derive(Debug, Deserialize)]
pub struct PhysicsDiction {
    pub Id: String,
    pub Name: String,
}

#[derive(Debug, Deserialize)]
pub struct PhysicsSetting {
    pub Id: String,
    pub Input: Vec<InputParam>,
    /// output_count
    pub Output: Vec<OutputParam>,
    pub Vertices: Vec<Vertice>,
    pub Normalization: Normalize,
}

#[derive(Debug, Deserialize)]
pub struct InputParam {
    pub Source: InputSource,
    pub Weight: f32,
    pub Type: String,
    pub Reflect: bool,
}

#[derive(Debug, Deserialize)]
pub struct InputSource {
    pub Target: String,
    pub Id: String,
}

#[derive(Debug, Deserialize)]
pub struct OutputParam {
    pub Destination: OutputDestination,
    pub VertexIndex: usize,
    pub Scale: f32,
    pub Weight: f32,
    pub Type: String,
    pub Reflect: bool,
}

#[derive(Debug, Deserialize)]
pub struct OutputDestination {
    pub Target: String,
    pub Id: String,
}

#[derive(Debug, Deserialize)]
pub struct Vertice {
    pub Position: Vec2,
    pub Mobility: f32,
    pub Delay: f32,
    pub Acceleration: f32,
    pub Radius: f32,
}

#[derive(Debug, Deserialize)]
pub struct Normalize {
    pub Position: NormalizePosition,
    pub Angle: NormalizeAngle,
}

#[derive(Debug, Deserialize)]
pub struct NormalizePosition {
    pub Minimum: f32,
    pub Default: f32,
    pub Maximum: f32,
}

#[derive(Debug, Deserialize)]
pub struct NormalizeAngle {
    pub Minimum: f32,
    pub Default: f32,
    pub Maximum: f32,
}
