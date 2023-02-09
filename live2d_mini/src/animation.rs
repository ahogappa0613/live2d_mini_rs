use std::collections::HashMap;

use crate::model_resource::Live2DModelResource;
use crate::motion_json;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl AnimationCurveType {
    #[inline]
    pub fn last_point(&self) -> AnimationPoint {
        match self {
            AnimationCurveType::Linear(_, l) => l.clone(),
            AnimationCurveType::Bezier(_, _, _, l) => l.clone(),
            AnimationCurveType::Stepped(_, l) => l.clone(),
            AnimationCurveType::InverseStepped(_, l) => l.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// アニメーションカーブの種類
pub enum AnimationSegment {
    Linear,
    Bezier,
    Stepped,
    InverseStepped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// カーブで変化する値の種類
pub enum AnimationType {
    ModelAnimationCurve,
    ParameterAnimationCurve,
    PartOpacityAnimationCurve,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnimationPoint {
    pub time: f32,
    pub value: f32,
}

/// アニメーション一つの情報全て
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationCurve {
    pub curve_type: AnimationType,
    pub segments: Vec<AnimationCurveType>,
    /// どこまで再生したか
    pub evaluated_index: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Animation {
    /// animationの最大時間
    pub duration: f32,
    /// アニメーションがループするか
    pub is_loop: bool,
    /// カーブの個数
    pub curve_count: usize,

    pub curves: HashMap<String, AnimationCurve>,
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
                    evaluated_index: 0,
                },
                "PartOpacity" => AnimationCurve {
                    curve_type: AnimationType::PartOpacityAnimationCurve,
                    segments: parse_segments(&curve.Segments),
                    evaluated_index: 0,
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

    /// TODO
    pub fn add(json: &motion_json::MotionJson) {
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
                    evaluated_index: 0,
                },
                "PartOpacity" => AnimationCurve {
                    curve_type: AnimationType::PartOpacityAnimationCurve,
                    segments: parse_segments(&curve.Segments),
                    evaluated_index: 0,
                },
                _ => panic!(),
            };

            curves.insert(id, segments);
        }

        // Animation {
        //     duration,
        //     is_loop,
        //     curve_count,
        //     curves,
        // }
    }

    /// ある時間のアニメーションをmodel, parametor, opacityをそれぞれ実行する
    pub fn evaluate_animation(&mut self, model: &Live2DModelResource, time: f32) {
        // dbg!(&animation.curves);
        for (id, curve) in self.curves.iter_mut() {
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

    pub fn reset_evaluate_indeies(&mut self) {
        self.curves
            .iter_mut()
            .for_each(|curve| curve.1.evaluated_index = 0);
    }
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

impl AnimationCurve {
    pub fn evaluate_curve(&mut self, time: f32) -> f32 {
        let (evaluate_index, target_segment) = self
            .segments
            .iter()
            .skip(self.evaluated_index)
            .enumerate()
            .find(|(_, segment)| match segment {
                AnimationCurveType::Linear(p0, p1) => time >= p0.time && time <= p1.time,
                AnimationCurveType::Bezier(p0, _, _, p3) => time >= p0.time && time <= p3.time,
                AnimationCurveType::Stepped(p0, p1) => time >= p0.time && time <= p1.time,
                AnimationCurveType::InverseStepped(p0, p1) => time >= p0.time && time <= p1.time,
            })
            .expect("not find segment");

        self.evaluated_index = evaluate_index;
        target_segment.evaluate(time)
    }
}

impl AnimationCurveType {
    pub fn evaluate(&self, time: f32) -> f32 {
        match self {
            AnimationCurveType::Linear(p0, p1) => {
                let mut t = (time - p0.time) / (p1.time - p0.time);
                if t < 0.0 {
                    t = 0.0;
                }

                p0.value + ((p1.value - p0.value) * t)
            }
            AnimationCurveType::Bezier(p0, p1, p2, p3) => {
                // 以下は古い方式
                // let mut t = (time - p0.time) / (p3.time - p0.time);
                // if t < 0.0 {
                //     t = 0.0;
                // }

                // let p01 = Self::lerp_points(&p0, &p1, t);
                // let p12 = Self::lerp_points(&p1, &p2, t);
                // let p23 = Self::lerp_points(&p2, &p3, t);

                // let p012 = Self::lerp_points(&p01, &p12, t);
                // let p123 = Self::lerp_points(&p12, &p23, t);

                // Self::lerp_points(&p012, &p123, t).value

                // 最新方式
                let x = time;
                let x1 = p0.time;
                let x2 = p3.time;
                let cx1 = p1.time;
                let cx2 = p2.time;

                let a = x2 - 3.0 * cx2 + 3.0 * cx1 - x1;
                let b = 3.0 * cx2 - 6.0 * cx1 + 3.0 * x1;
                let c = 3.0 * cx1 - 3.0 * x1;
                let d = x1 - x;

                let t = Self::cardano_algorithm_for_bezier(a, b, c, d);

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

    fn quadration_equation(a: f32, b: f32, c: f32) -> f32 {
        if a.abs() < std::f32::EPSILON {
            if b.abs() < std::f32::EPSILON {
                return -c;
            }
            return -c / b;
        }

        return -(b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    }

    fn cardano_algorithm_for_bezier(a: f32, b: f32, c: f32, d: f32) -> f32 {
        if a.abs() < std::f32::EPSILON {
            return Self::quadration_equation(b, c, d).clamp(0.0, 1.0);
        }

        let ba = b / a;
        let ca = c / a;
        let da = d / a;

        let p = (3.0 * ca - ba * ba) / 3.0;
        let p3 = p / 3.0;
        let q = (2.0 * ba * ba * ba - 9.0 * ba * ca + 27.0 * da) / 27.0;
        let q2 = q / 2.0;
        let discriminant = q2 * q2 + p3 * p3 * p3;

        let center = 0.5;
        let threshold = center + 0.01;

        if discriminant < 0.0 {
            let mp3 = -p / 3.0;
            let mp33 = mp3 * mp3 * mp3;
            let r = mp33.sqrt();
            let t = -q / (2.0 * r);
            let cosphi = t.clamp(-1.0, 1.0);
            let phi = cosphi.acos();
            let crtr = r.cbrt();
            let t1 = 2.0 * crtr;

            let root1 = t1 * (phi / 3.0).cos() - ba / 3.0;
            if (root1 - center).abs() < threshold {
                return root1.clamp(0.0, 1.0);
            }

            let root2 = t1 * ((phi + 2.0 * std::f32::consts::PI) / 3.0).cos() - ba / 3.0;
            if (root2 - center).abs() < threshold {
                return root2.clamp(0.0, 1.0);
            }

            let root3 = t1 * ((phi + 4.0 * std::f32::consts::PI) / 3.0).cos() - ba / 3.0;
            return root3.clamp(0.0, 1.0);
        }

        if discriminant == 0.0 {
            let u1 = if q2 < 0.0 { (-q2).cbrt() } else { -(q2.cbrt()) };

            let root1 = 2.0 * u1 - ba / 3.0;
            if (root1 - center).abs() < threshold {
                return root1.clamp(0.0, 1.0);
            }

            let root2 = -u1 - ba / 3.0;
            if (root2 - center).abs() < threshold {
                return root2.clamp(0.0, 1.0);
            }
        }

        let sd = discriminant.sqrt();
        let u1 = (sd - q2).cbrt();
        let v1 = (sd + q2).cbrt();
        let root1 = u1 - v1 - ba / 3.0;

        root1.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn lerp_points_test() {
        use super::*;

        let a = AnimationPoint {
            time: 0.0,
            value: 18.0,
        };
        let b = AnimationPoint {
            time: 0.210999995,
            value: 18.0,
        };

        let t = AnimationCurveType::lerp_points(&a, &b, -0.0);

        assert_eq!(
            t,
            AnimationPoint {
                time: 0.0,
                value: 18.0
            }
        );

        let a = AnimationPoint {
            time: 0.210999995,
            value: 18.0,
        };
        let b = AnimationPoint {
            time: 0.421999991,
            value: 0.0,
        };

        let t = AnimationCurveType::lerp_points(&a, &b, -0.0);

        assert_eq!(
            t,
            AnimationPoint {
                time: 0.210999995,
                value: 18.0
            }
        );

        let a = AnimationPoint {
            time: 0.0,
            value: 1.0,
        };
        let b = AnimationPoint {
            time: 0.333000004,
            value: 1.0,
        };

        let t = AnimationCurveType::lerp_points(&a, &b, 0.00000154972076);

        assert_eq!(
            t,
            AnimationPoint {
                time: 5.16057014e-7,
                value: 1.0
            }
        );
    }
}
