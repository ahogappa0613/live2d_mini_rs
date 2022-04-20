use std::collections::HashMap;

use crate::motion_json;

#[derive(Debug, Clone, Copy)]
/// アニメーションカーブの種類
pub enum AnimationCurveType {
    Linear(AnimationPoint, AnimationPoint),
    Bezier(
        AnimationPoint,
        AnimationPoint,
        AnimationPoint,
        AnimationPoint,
    ),
    Stepped(AnimationPoint, AnimationPoint),
    InverseStepped(AnimationPoint, AnimationPoint),
}

#[derive(Debug, Clone, Copy)]
/// アニメーションカーブの種類
pub enum AnimationSegment {
    Linear,
    Bezier,
    Stepped,
    InverseStepped,
}

#[derive(Debug, Clone, Copy)]
/// カーブで変化する値の種類
pub enum AnimationType {
    ModelAnimationCurve,
    ParameterAnimationCurve,
    PartOpacityAnimationCurve,
}

#[derive(Debug, Clone, Copy)]
pub struct AnimationPoint {
    pub time: f32,
    pub value: f32,
}

/// アニメーション一つの情報全て
#[derive(Debug, Clone)]
pub struct AnimationCurve {
    pub curve_type: AnimationType,
    pub segments: Vec<AnimationCurveType>,
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub duration: f32, // animationの最大時間
    pub is_loop: bool,
    pub curve_count: usize,

    pub curves: HashMap<String, AnimationCurve>,
}

/// これなんとかしたい
pub fn parse_segments(segments_vec: &Vec<f32>) -> Vec<AnimationCurveType> {
    let mut ret = vec![];

    let mut index = 2;
    let mut index_d = 0;
    // 最初の点はどのcurve typeも固定
    let mut last_point = AnimationPoint {
        time: segments_vec[0],
        value: segments_vec[1],
    };

    loop {
        if segments_vec.get(index).is_none() {
            break;
        }

        match segments_vec[index] {
            1.0 => {
                index_d = 7;
                ret.push(AnimationCurveType::Bezier(
                    last_point,
                    AnimationPoint {
                        time: segments_vec[index + 1],
                        value: segments_vec[index + 2],
                    },
                    AnimationPoint {
                        time: segments_vec[index + 3],
                        value: segments_vec[index + 4],
                    },
                    AnimationPoint {
                        time: segments_vec[index + 5],
                        value: segments_vec[index + 6],
                    },
                ));

                last_point = AnimationPoint {
                    time: segments_vec[index + 5],
                    value: segments_vec[index + 6],
                };
            }
            0.0 => {
                index_d = 3;
                ret.push(AnimationCurveType::Linear(
                    last_point,
                    AnimationPoint {
                        time: segments_vec[index + 1],
                        value: segments_vec[index + 2],
                    },
                ));

                last_point = AnimationPoint {
                    time: segments_vec[index + 1],
                    value: segments_vec[index + 2],
                };
            }
            _ => panic!(),
        };

        index += index_d;
    }

    ret
}

impl Animation {
    pub fn new(json: &motion_json::MotionJson) -> Animation {
        let duration = json.Meta.Duration;
        let curve_count = json.Meta.CurveCount;
        let is_loop = json.Meta.Loop;
        let mut curves: HashMap<String, AnimationCurve> = HashMap::new();

        for curve in json.Curves.iter() {
            let id = curve.Id.clone();
            let segments = match &*curve.Target {
                "Parameter" => AnimationCurve {
                    curve_type: AnimationType::ParameterAnimationCurve,
                    segments: parse_segments(&curve.Segments),
                },
                "PartOpacity" => AnimationCurve {
                    curve_type: AnimationType::PartOpacityAnimationCurve,
                    segments: parse_segments(&curve.Segments),
                },
                _ => panic!(),
            };

            curves.insert(id, segments);
        }

        Animation {
            duration,
            is_loop,
            curve_count,
            curves,
        }
    }
}

/// ある時間のアニメーションをmodel, parametor, opacityをそれぞれ実行する
pub fn evaluate_animation(
    model: &live2d_mini::model::Live2DModel,
    animation: &Animation,
    time: f32,
) {
    for (id, curve) in animation.curves.iter() {
        let value = curve.evaluate_curve(time);

        match curve.curve_type {
            AnimationType::ModelAnimationCurve => todo!(),
            AnimationType::ParameterAnimationCurve => {
                let target = model
                    .iter_mut_parameters()
                    .find(|part| part.id() == id)
                    .expect("not find parameter");
                *target.value = value;
            }
            AnimationType::PartOpacityAnimationCurve => {
                let target = model
                    .iter_mut_parts()
                    .find(|part| part.id() == id)
                    .expect("not find part");
                *target.opacitiy = value;
            }
        }
    }
}

impl AnimationCurve {
    pub fn evaluate_curve(&self, time: f32) -> f32 {
        let target_segment = self.segments.iter().find(|segment| match segment {
            AnimationCurveType::Linear(p0, p1) => time >= p0.time && time <= p1.time,
            AnimationCurveType::Bezier(p0, _, _, p3) => time >= p0.time && time <= p3.time,
            AnimationCurveType::Stepped(p0, p1) => time >= p0.time && time <= p1.time,
            AnimationCurveType::InverseStepped(p0, p1) => time >= p0.time && time <= p1.time,
        }).expect("not find segment");

        target_segment.evaluate(time)
    }
}

impl AnimationCurveType {
    pub fn evaluate(&self, time: f32) -> f32 {
        match self {
            AnimationCurveType::Linear(p0, p1) => {
                let t = (time - p0.time) / (p1.time - p0.time);
                p0.value + ((p1.value - p0.value) * t)
            }
            AnimationCurveType::Bezier(p0, p1, p2, p3) => {
                let t = (time - p0.time) / (p3.time - p0.time);

                let p01 = Self::lerp_points(&p0, &p1, t);
                let p12 = Self::lerp_points(&p1, &p2, t);
                let p23 = Self::lerp_points(&p2, &p3, t);

                let p012 = Self::lerp_points(&p01, &p12, t);
                let p123 = Self::lerp_points(&p12, &p23, t);

                Self::lerp_points(&p012, &p123, t).value
            }
            AnimationCurveType::Stepped(p0, _p1) => p0.value,
            AnimationCurveType::InverseStepped(_p0, p1) => p1.value,
        }
    }

    #[inline]
    fn lerp_points(a: &AnimationPoint, b: &AnimationPoint, t: f32) -> AnimationPoint {
        AnimationPoint {
            time: a.time + ((b.time - a.time) * t),
            value: a.value + ((b.value - a.value) * t),
        }
    }
}
