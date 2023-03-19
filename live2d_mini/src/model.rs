use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::animation::*;
use crate::model_json;
use crate::motion_json;
use crate::physic_json;

use image::RgbaImage;

use crate::model_resource::Live2DModelResource;
use crate::physic::Physics;

#[derive(Debug)]
pub struct Live2DModel {
    pub resource: Live2DModelResource,
    pub animations: Vec<Animation>,
    pub textures: Vec<RgbaImage>,
    pub physics: Option<Physics>,

    /// 再生するアニメーションの番号
    animation_index: Option<usize>,
}

impl<'a> Live2DModel {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let path = Path::new(path.as_ref());

        let current_dir = path.parent().expect("cannot find parents");

        let file = File::open(path).expect(&format!("cannot open file: {:?}", path.to_str()));
        let reader = BufReader::new(file);

        let model_json: model_json::ModelJson =
            serde_json::from_reader(reader).expect("deselialize error");

        let textures = model_json
            .FileReferences
            .Textures
            .iter()
            .map(|path| {
                image::io::Reader::open(current_dir.join(path))
                    .expect("not find image")
                    .decode()
                    .expect("decode faild")
                    .flipv()
                    .to_rgba8()
            })
            .collect::<Vec<RgbaImage>>();

        let resource = Live2DModelResource::new(current_dir.join(model_json.FileReferences.Moc))
            .expect("moc load error");
        // let file =
        //     File::open(current_dir.join(model_json.FileReferences.Pose.expect(""))).expect("");
        // let reader = BufReader::new(file);
        // let u: pose_json::PoseJson = serde_json::from_reader(reader).expect("");

        let physics = if let Some(physics_path) = model_json.FileReferences.Physics {
            let file = File::open(current_dir.join(physics_path)).expect("open error");
            let reader = BufReader::new(file);
            let physic_json: physic_json::PhysicJson =
                serde_json::from_reader(reader).expect("load error");
            let mut raw_physics = Physics::new(physic_json);
            raw_physics.initialize();
            Some(raw_physics)
        } else {
            None
        };

        let motions = model_json
            .FileReferences
            .Motions
            .expect("")
            .Idle
            .iter()
            .map(|idle| {
                let file = File::open(current_dir.join(&idle.File)).expect("file open error");
                let reader = BufReader::new(file);
                serde_json::from_reader(reader).expect("deselialize error")
            })
            .collect::<Vec<motion_json::MotionJson>>();

        let animations = motions
            .iter()
            .map(|motion| Animation::new(motion))
            .collect::<Vec<Animation>>();

        Live2DModel {
            resource,
            animations,
            textures,
            physics,
            animation_index: None,
        }
    }

    pub fn animation(&mut self, time: f32) {
        if let Some(anime) = self
            .animations
            .get_mut(self.animation_index.expect("no set animation"))
        {
            anime.evaluate_animation(&mut self.resource, time)
        } else {
            panic!("not find animation")
        }
    }

    pub fn get_animation(&self) -> Option<&Animation> {
        self.animations
            .get(self.animation_index.expect("no set animation"))
    }

    pub fn get_mut_animation(&mut self) -> Option<&mut Animation> {
        self.animations
            .get_mut(self.animation_index.expect("no set animation"))
    }

    pub fn evaluate_physic(&mut self, delta_time: f32) {
        if let Some(physic) = self.physics.as_mut() {
            physic.evaluate(&mut self.resource, delta_time)
        }
    }

    /// indexを設定した値にし
    /// 再生時間を0にする
    pub fn reset_animation(&mut self, index: usize) {
        self.animation_index = Some(index);
        self.animation(0.0);
        self.replace_default_values();
        self.resource.update();
    }

    fn replace_default_values(&self) {
        for (value, default_value) in self
            .resource
            .csm_get_mut_parameter_values()
            .iter_mut()
            .zip(self.resource.csm_get_parameter_default_values())
        {
            *value = *default_value;
        }
    }
}
