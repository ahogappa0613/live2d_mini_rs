use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::animation::*;
use crate::model_json;
use crate::motion_json;
use crate::pose_json;

use image::RgbaImage;

use crate::model_resource::Live2DModelResource;

pub struct Live2DModel {
    pub resource: Live2DModelResource,
    pub animations: Vec<Animation>,
    pub textures: Vec<RgbaImage>,

    animation_index: Option<usize>,
}

impl Live2DModel {
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

        let resource = Live2DModelResource::new(current_dir.join(model_json.FileReferences.Moc));
        // let file =
        //     File::open(current_dir.join(model_json.FileReferences.Pose.expect(""))).expect("");
        // let reader = BufReader::new(file);
        // let u: pose_json::PoseJson = serde_json::from_reader(reader).expect("");
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
        self.resource.update();
    }

    pub fn get_animation(&self) -> Option<&Animation> {
        self.animations
            .get(self.animation_index.expect("no set animation"))
    }

    pub fn get_mut_animation(&mut self) -> Option<&mut Animation> {
        self.animations
            .get_mut(self.animation_index.expect("no set animation"))
    }

    /// indexを設定した値にし
    /// 再生時間を0にする
    pub fn reset_animation(&mut self, index: usize) {
        self.animation_index = Some(index);
        self.animation(0.0);
        self.resource.update();
    }
}
