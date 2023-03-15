use std::{
    f32::consts::PI,
    ops::{Add, Div, DivAssign, Mul, MulAssign, Sub},
};

use crate::{model_resource::Live2DModelResource, physic_json};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct CubismVector2 {
    x: f32,
    y: f32,
}

impl CubismVector2 {
    pub fn normalize(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y).powf(0.5f32);

        self.clone() / length
    }

    pub fn to_radian(&self, to: &CubismVector2) -> f32 {
        let q1 = to.y.atan2(to.x);
        let q2 = self.y.atan2(self.x);

        let mut ret = q1 - q2;
        while ret < -PI {
            ret += PI * 2.0;
        }

        while ret > PI {
            ret -= PI * 2.0;
        }

        ret
    }
}

impl Add for CubismVector2 {
    type Output = CubismVector2;

    fn add(self, rhs: Self) -> Self::Output {
        CubismVector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for CubismVector2 {
    type Output = CubismVector2;

    fn sub(self, rhs: Self) -> Self::Output {
        CubismVector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for CubismVector2 {
    type Output = CubismVector2;

    fn mul(self, rhs: f32) -> Self::Output {
        CubismVector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for CubismVector2 {
    type Output = CubismVector2;

    fn div(self, rhs: f32) -> Self::Output {
        CubismVector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl MulAssign<f32> for CubismVector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign<f32> for CubismVector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

/**
 * @brief 物理演算の適用先の種類
 *
 * 物理演算の適用先の種類。
 */
#[derive(Debug, Clone, PartialEq)]
enum CubismPhysicsTargetType {
    /// パラメータに対して適用
    CubismPhysicsTargetTypeParameter,
}

/**
 * @brief 物理演算の入力の種類
 *
 * 物理演算の入力の種類。
 */
#[derive(Debug, Clone, PartialEq)]
enum CubismPhysicsSource {
    /// X軸の位置から
    CubismPhysicsSourceX,
    /// Y軸の位置から
    CubismPhysicsSourceY,
    /// 角度から
    CubismPhysicsSourceAngle,
}

/**
 * @brief 物理演算で使用する外部の力
 *
 * 物理演算で使用する外部の力。
 */
#[derive(Debug, Default, Clone)]
struct PhysicsJsonEffectiveForces {
    /// 重力
    gravity: CubismVector2,
    /// 風
    wind: CubismVector2,
}

/**
 * @brief 物理演算のパラメータ情報
 *
 * 物理演算のパラメータ情報。
 */
#[derive(Debug, Clone, PartialEq)]
struct CubismPhysicsParameter {
    /// パラメータID
    id: String,
    /// 適用先の種類
    target_type: CubismPhysicsTargetType,
}

/**
 * @brief 物理演算の正規化情報
 *
 * 物理演算の正規化情報。
 */
#[derive(Debug, Default, Clone, PartialEq)]
struct CubismPhysicsNormalization {
    /// 最大値
    minimum: f32,
    /// 最小値
    maximum: f32,
    /// デフォルト値
    default: f32,
}

/**
 * @brief 物理演算の演算に使用する物理点の情報
 *
 * 物理演算の演算に使用する物理点の情報。
 */
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct CubismPhysicsParticle {
    /// 初期位置
    pub initial_position: CubismVector2,
    /// 動きやすさ
    pub mobility: f32,
    /// 遅れ
    pub delay: f32,
    /// 加速度
    pub acceleration: f32,
    /// 距離
    pub radius: f32,
    /// 現在の位置
    pub position: CubismVector2,
    /// 最後の位置
    pub last_position: CubismVector2,
    /// 最後の重力
    pub last_gravity: CubismVector2,
    /// 現在かかっている力
    pub force: CubismVector2,
    /// 現在の速度
    pub velocity: CubismVector2,
}

/**
 * @brief 物理演算の物理点の管理
 *
 * 物理演算の物理点の管理。
 */
#[derive(Debug, Default, Clone, PartialEq)]
struct CubismPhysicsSubRig {
    /// 入力の個数
    input_count: usize,
    /// 出力の個数
    output_count: usize,
    /// 物理点の個数
    particle_count: usize,
    /// 入力の最初のインデックス
    base_input_index: usize,
    /// 出力の最初のインデックス
    base_output_index: usize,
    /// 物理点の最初のインデックス
    base_particle_index: usize,
    /// 正規化された位置
    normalization_position: CubismPhysicsNormalization,
    /// 正規化された角度
    normalization_angle: CubismPhysicsNormalization,
}

/**
 * @brief 物理演算の入力情報
 *
 * 物理演算の入力情報。
 */
#[derive(Clone, Debug, PartialEq)]
struct CubismPhysicsInput {
    /// 入力元のパラメータ
    source: CubismPhysicsParameter,
    /// 入力元のパラメータのインデックス
    source_parameter_index: Option<usize>,
    /// 重み
    weight: f32,
    /// 入力の種類
    input_type: CubismPhysicsSource,
    /// 値が反転されているかどうか
    reflect: bool,
    // 正規化されたパラメータ値の取得関数
    cubism_physics_input_type: CubismPhysicsInputType,
}

/**
 * @brief 物理演算の出力情報
 *
 * 物理演算の出力情報。
 */
#[derive(Debug, Clone, PartialEq)]
struct CubismPhysicsOutput {
    /// 出力先のパラメータ
    destination: CubismPhysicsParameter,
    /// 出力先のパラメータのインデックス
    destination_parameter_index: Option<usize>,
    /// 振り子のインデックス
    vertex_index: usize,
    /// 移動値のスケール
    translation_scale: CubismVector2,
    /// 角度のスケール
    angle_scale: f32,
    /// 重み
    weight: f32,
    /// 出力の種類
    output_type: CubismPhysicsSource,
    /// 値が反転されているかどうか
    reflect: bool,
    /// 最小値を下回った時の値
    value_below_minimum: f32,
    /// 最大値をこえた時の値
    value_exceeded_maximum: f32,
    // 物理演算の値の取得関数
    get_value: GetOutputTranslation,
    // 物理演算のスケール値の取得関数
    get_scale: GetOutputScaleTranslationType,
}

/**
 * @brief 物理演算のデータ
 *
 * 物理演算のデータ。
 */
#[derive(Clone, Debug, PartialEq)]
struct CubismPhysicsRig {
    /// 物理演算の物理点の個数
    /// Vec\<PhysicsSetting\>::len()
    sub_rig_count: usize,
    /// 物理演算の物理点の管理のリスト
    settings: Vec<CubismPhysicsSubRig>,
    /// 物理演算の入力のリスト
    inputs: Vec<CubismPhysicsInput>,
    /// 物理演算の出力のリスト
    outputs: Vec<CubismPhysicsOutput>,
    /// 物理演算の物理点のリスト
    particles: Vec<CubismPhysicsParticle>,
    /// 重力
    gravity: CubismVector2,
    /// 風
    wind: CubismVector2,
    /// 物理演算動作FPS
    fps: Option<f32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Physics {
    /// 最新の振り子計算の結果
    current_rig_outputs: Vec<Vec<f32>>,
    /// 一つ前の振り子計算の結果
    previous_rig_outputs: Vec<Vec<f32>>,
    /// 物理演算が処理していない時間
    current_remain_time: f32,
    /// Evaluateで利用するパラメータのキャッシュ
    parameter_cache: Vec<f32>,
    parameter_cache_tmp: Option<Vec<f32>>,
    /// UpdateParticlesが動くときの入力をキャッシュ
    parameter_input_cache: Vec<f32>,
    /// 物理演算のデータ
    physics_rig: CubismPhysicsRig,
}

#[derive(Debug, Clone, PartialEq)]
enum GetOutputScaleTranslationType {
    X,
    Y,
    Angle,
}

impl GetOutputScaleTranslationType {
    fn get_output_scale_translation(
        &self,
        translation_scale: CubismVector2,
        angle_scale: f32,
    ) -> f32 {
        match self {
            GetOutputScaleTranslationType::X => translation_scale.x,
            GetOutputScaleTranslationType::Y => translation_scale.y,
            GetOutputScaleTranslationType::Angle => angle_scale,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum CubismPhysicsInputType {
    X,
    Y,
    Angle,
}

impl CubismPhysicsInputType {
    fn get_normalized_parameter_value(
        &self,
        target_translation: &mut CubismVector2,
        target_angle: &mut f32,
        value: f32,
        parameter_minimum: f32,
        parameter_maximum: f32,
        parameter_default: f32,
        normalization_position: &CubismPhysicsNormalization,
        normalization_angle: &CubismPhysicsNormalization,
        is_inverted: bool,
        weight: f32,
    ) -> () {
        match self {
            CubismPhysicsInputType::X => {
                target_translation.x += normalize_parameter_value(
                    value,
                    parameter_minimum,
                    parameter_maximum,
                    parameter_default,
                    normalization_position.minimum,
                    normalization_position.maximum,
                    normalization_position.default,
                    is_inverted,
                ) * weight;
            }
            CubismPhysicsInputType::Y => {
                target_translation.y += normalize_parameter_value(
                    value,
                    parameter_minimum,
                    parameter_maximum,
                    parameter_default,
                    normalization_position.minimum,
                    normalization_position.maximum,
                    normalization_position.default,
                    is_inverted,
                ) * weight;
            }
            CubismPhysicsInputType::Angle => {
                *target_angle += normalize_parameter_value(
                    value,
                    parameter_minimum,
                    parameter_maximum,
                    parameter_default,
                    normalization_angle.minimum,
                    normalization_angle.maximum,
                    normalization_angle.default,
                    is_inverted,
                ) * weight;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GetOutputTranslation {
    X,
    Y,
    Angle,
}

impl GetOutputTranslation {
    pub fn get_value(
        &self,
        translation: CubismVector2,
        particles: Vec<&CubismPhysicsParticle>,
        particle_index: usize,
        is_inverted: bool,
        parent_gravity: CubismVector2,
    ) -> f32 {
        match self {
            GetOutputTranslation::X => {
                if is_inverted {
                    translation.x * -1.0
                } else {
                    translation.x
                }
            }
            GetOutputTranslation::Y => {
                if is_inverted {
                    translation.y * -1.0
                } else {
                    translation.y
                }
            }
            GetOutputTranslation::Angle => {
                let gravity = if particle_index >= 2 {
                    particles[particle_index - 1].position - particles[particle_index - 2].position
                } else {
                    parent_gravity * -1.0
                };

                let output_value = gravity.to_radian(&translation);

                if is_inverted {
                    output_value * -1.0
                } else {
                    output_value
                }
            }
        }
    }
}

/// Constant of air resistance.
const AIR_RESISTANCE: f32 = 5.0;

/// Constant of maximum weight of input and output ratio.
const MAXIMUM_WEIGHT: f32 = 100.0;

/// Constant of threshold of movement.
const MOVEMENT_THRESHOLD: f32 = 0.001;

/// Constant of maximum allowed delta time
const MAX_DELTA_TIME: f32 = 5.0;

// L73
fn normalize_parameter_value(
    value: f32,
    parameter_minimum: f32,
    parameter_maximum: f32,
    _parameter_default: f32,
    normalized_minimum: f32,
    normalized_maximum: f32,
    normalized_default: f32,
    is_inverted: bool,
) -> f32 {
    let mut result = 0.0;

    let max_value = parameter_maximum.max(parameter_minimum);
    // 補正する
    let value = if max_value < value { max_value } else { value };

    let min_value = parameter_maximum.min(parameter_minimum);
    // 補正する
    let value = if min_value > value { min_value } else { value };

    let min_norm_value = normalized_minimum.min(normalized_maximum);
    let max_norm_value = normalized_minimum.max(normalized_maximum);
    let middle_norm_value = normalized_default;

    let middle_value = get_default_value(min_value, max_value);
    let param_value = value - middle_value;

    match param_value {
        n if n > 0.0 => {
            let n_length = max_norm_value - middle_norm_value;
            let p_length = max_value - middle_value;
            if p_length != 0.0 {
                result = param_value * (n_length / p_length) + middle_norm_value;
            }
        }
        n if n < 0.0 => {
            let n_length = min_norm_value - middle_norm_value;
            let p_length = min_value - middle_value;
            if p_length != 0.0 {
                result = param_value * (n_length / p_length) + middle_norm_value;
            }
        }
        n if n == 0.0 => result = middle_norm_value,
        _ => unreachable!(),
    }

    if is_inverted {
        result
    } else {
        -result
    }
}

// L67
fn get_default_value(min: f32, max: f32) -> f32 {
    let min_value = min.min(max);

    min_value + (get_range_value(min, max) / 2.0)
}

// L38
fn get_range_value(min: f32, max: f32) -> f32 {
    let max_value = min.max(max);
    let min_value = min.min(max);

    (max_value - min_value).abs()
}

trait CubismPhysicsUtil {
    fn to_direction(&self) -> CubismVector2;
}

impl CubismPhysicsUtil for f32 {
    fn to_direction(&self) -> CubismVector2 {
        CubismVector2 {
            x: self.sin(),
            y: self.cos(),
        }
    }
}

// https://github.com/Live2D/CubismNativeFramework/blob/cbd4dfaa5ee95218ea3f9af30f8525c60b4a9b36/src/Physics/CubismPhysics.cpp
impl<'a> Physics {
    pub fn new(json: physic_json::PhysicJson) -> Self {
        let mut settings: Vec<CubismPhysicsSubRig> = vec![];
        let mut inputs: Vec<CubismPhysicsInput> = vec![];
        let mut outputs: Vec<CubismPhysicsOutput> = vec![];
        let mut particles: Vec<CubismPhysicsParticle> = vec![];

        let mut current_rig_outputs = vec![];
        let mut previous_rig_outputs = vec![];

        let (mut base_input_index, mut base_output_index, mut base_particle_index) = (0, 0, 0);

        json.PhysicsSettings.iter().for_each(|setting| {
            // settings
            settings.push(CubismPhysicsSubRig {
                input_count: setting.Input.len(),
                output_count: setting.Output.len(),
                particle_count: setting.Vertices.len(),
                base_input_index,
                base_output_index,
                base_particle_index,
                normalization_position: CubismPhysicsNormalization {
                    minimum: setting.Normalization.Position.Minimum,
                    maximum: setting.Normalization.Position.Maximum,
                    default: setting.Normalization.Position.Default,
                },
                normalization_angle: CubismPhysicsNormalization {
                    minimum: setting.Normalization.Angle.Minimum,
                    maximum: setting.Normalization.Angle.Maximum,
                    default: setting.Normalization.Angle.Default,
                },
            });

            // input
            for input in setting.Input.iter() {
                let weight = input.Weight;
                let reflect = input.Reflect;
                let id = input.Source.Id.clone();

                inputs.push(match &*input.Type {
                    "X" => CubismPhysicsInput {
                        source: CubismPhysicsParameter {
                            target_type: CubismPhysicsTargetType::CubismPhysicsTargetTypeParameter,
                            id,
                        },
                        source_parameter_index: None,
                        weight,
                        input_type: CubismPhysicsSource::CubismPhysicsSourceX,
                        reflect,
                        cubism_physics_input_type: CubismPhysicsInputType::X,
                    },
                    "Y" => CubismPhysicsInput {
                        source: CubismPhysicsParameter {
                            target_type: CubismPhysicsTargetType::CubismPhysicsTargetTypeParameter,
                            id,
                        },
                        source_parameter_index: None,
                        weight,
                        input_type: CubismPhysicsSource::CubismPhysicsSourceY,
                        reflect,
                        cubism_physics_input_type: CubismPhysicsInputType::Y,
                    },
                    "Angle" => CubismPhysicsInput {
                        source: CubismPhysicsParameter {
                            target_type: CubismPhysicsTargetType::CubismPhysicsTargetTypeParameter,
                            id,
                        },
                        source_parameter_index: None,
                        weight,
                        input_type: CubismPhysicsSource::CubismPhysicsSourceAngle,
                        reflect,
                        cubism_physics_input_type: CubismPhysicsInputType::Angle,
                    },

                    _ => unreachable!(),
                });
            }

            // output
            for output in setting.Output.iter() {
                let translation_scale = CubismVector2::default();
                let weight = output.Weight;
                let reflect = output.Reflect;
                let angle_scale = output.Scale;
                let vertex_index = output.VertexIndex;
                let destination = CubismPhysicsParameter {
                    id: output.Destination.Id.clone(),
                    target_type: CubismPhysicsTargetType::CubismPhysicsTargetTypeParameter,
                };

                outputs.push(match &*output.Type {
                    "X" => CubismPhysicsOutput {
                        destination,
                        destination_parameter_index: None,
                        vertex_index,
                        translation_scale,
                        angle_scale,
                        weight,
                        output_type: CubismPhysicsSource::CubismPhysicsSourceX,
                        reflect,
                        value_below_minimum: 0.0,
                        value_exceeded_maximum: 0.0,
                        get_value: GetOutputTranslation::X,
                        get_scale: GetOutputScaleTranslationType::X,
                    },
                    "Y" => CubismPhysicsOutput {
                        destination,
                        destination_parameter_index: None,
                        vertex_index,
                        translation_scale,
                        angle_scale,
                        weight,
                        output_type: CubismPhysicsSource::CubismPhysicsSourceY,
                        reflect,
                        value_below_minimum: 0.0,
                        value_exceeded_maximum: 0.0,
                        get_value: GetOutputTranslation::Y,
                        get_scale: GetOutputScaleTranslationType::Y,
                    },
                    "Angle" => CubismPhysicsOutput {
                        destination,
                        destination_parameter_index: None,
                        vertex_index,
                        translation_scale,
                        angle_scale,
                        weight,
                        output_type: CubismPhysicsSource::CubismPhysicsSourceAngle,
                        reflect,
                        value_below_minimum: 0.0,
                        value_exceeded_maximum: 0.0,
                        get_value: GetOutputTranslation::Angle,
                        get_scale: GetOutputScaleTranslationType::Angle,
                    },

                    _ => unreachable!(),
                });
            }

            // particles
            for particle in setting.Vertices.iter() {
                particles.push(CubismPhysicsParticle {
                    initial_position: CubismVector2::default(), //後で初期化する
                    mobility: particle.Mobility,
                    delay: particle.Delay,
                    acceleration: particle.Acceleration,
                    radius: particle.Radius,
                    position: CubismVector2 {
                        x: particle.Position.X,
                        y: particle.Position.Y,
                    },
                    last_position: CubismVector2::default(), //後で初期化する
                    last_gravity: CubismVector2::default(),  //後で初期化する
                    force: CubismVector2::default(),         //後で初期化する
                    velocity: CubismVector2::default(),      //後で初期化する
                });
            }

            base_input_index += setting.Input.len();
            base_output_index += setting.Output.len();
            base_particle_index += setting.Vertices.len();

            current_rig_outputs.push(Vec::from_iter(outputs.iter().map(|_| 0.0)));
            previous_rig_outputs.push(Vec::from_iter(outputs.iter().map(|_| 0.0)));
        });

        Physics {
            current_rig_outputs,
            previous_rig_outputs,
            current_remain_time: 0.0,
            parameter_cache: vec![],
            parameter_cache_tmp: None,
            parameter_input_cache: vec![],
            physics_rig: CubismPhysicsRig {
                sub_rig_count: json.PhysicsSettings.len(),
                settings,
                inputs,
                outputs,
                particles,
                gravity: CubismVector2 {
                    x: json.Meta.EffectiveForces.Gravity.X,
                    y: json.Meta.EffectiveForces.Gravity.Y,
                },
                wind: CubismVector2 {
                    x: json.Meta.EffectiveForces.Wind.X,
                    y: json.Meta.EffectiveForces.Wind.Y,
                },
                fps: json.Meta.Fps,
            },
        }
    }

    // L416
    pub fn initialize(&mut self) {
        for current_setting in self.physics_rig.settings.iter() {
            let mut prev_strand = self.physics_rig.particles[current_setting.base_particle_index];
            // Initialize the top of particle.
            prev_strand.initial_position = CubismVector2::default();
            prev_strand.last_position = CubismVector2::default();
            prev_strand.last_gravity = CubismVector2 { x: 0.0, y: 1.0 };
            prev_strand.velocity = CubismVector2::default();
            prev_strand.force = CubismVector2::default();

            // 同じVecで現在の位置のmutと一つ前の参照を取ることになるので
            // cloneするしかない
            let mut prev_strand = prev_strand.clone();
            for particle in self
                .physics_rig
                .particles
                .iter_mut()
                .skip(current_setting.base_particle_index + 1)
                .take(current_setting.particle_count - 1)
            {
                let mut radius = CubismVector2::default();
                radius.y = particle.radius;
                particle.initial_position = prev_strand.initial_position + radius;
                particle.last_position = particle.initial_position;
                particle.last_gravity = CubismVector2 { x: 0.0, y: 1.0 };
                particle.velocity = CubismVector2::default();
                particle.force = CubismVector2::default();

                prev_strand = particle.clone();
            }
        }
    }

    pub fn initialize2(&mut self) {
        for current_setting in self.physics_rig.settings.iter() {
            // let prev_strand = self
            //     .physics_rig
            //     .particles
            //     .get_mut(current_setting.base_particle_index)
            //     .unwrap();
            // // Initialize the top of particle.
            // prev_strand.initial_position = CubismVector2::default();
            // prev_strand.last_position = CubismVector2::default();
            // prev_strand.last_gravity = CubismVector2 { x: 0.0, y: 1.0 };
            // prev_strand.velocity = CubismVector2::default();
            // prev_strand.force = CubismVector2::default();

            // let mut prev_strand = prev_strand.clone();
            // for particle in self
            //     .physics_rig
            //     .particles
            //     .iter_mut()
            //     .skip(current_setting.base_particle_index + 1)
            //     .take(current_setting.particle_count - 1)
            // {
            //     let mut radius = CubismVector2::default();
            //     radius.y = particle.radius;
            //     particle.initial_position = prev_strand.initial_position + radius;
            //     particle.last_position = particle.initial_position;
            //     particle.last_gravity = CubismVector2 { x: 0.0, y: 1.0 };
            //     particle.velocity = CubismVector2::default();
            //     particle.force = CubismVector2::default();

            //     prev_strand = particle.clone();
            // }
            // 上記コードをリファクタする

            for w in self.physics_rig.particles.windows(2) {
                let (prev_stand, next_stand) = (w[0], w[1]);
            }
        }
    }

    // L837
    fn interpolate(&mut self, model: &Live2DModelResource, weight: f32) {
        for setting_index in 0..self.physics_rig.sub_rig_count {
            let current_setting = &self.physics_rig.settings[setting_index];
            // let current_output = &self.physics_rig.outputs[current_setting.base_output_index];

            // Load input parameters.
            for (i, output) in self
                .physics_rig
                .outputs
                .iter_mut()
                .skip(current_setting.base_output_index)
                .take(current_setting.output_count)
                .enumerate()
            {
                if let Some(index) = output.destination_parameter_index {
                    update_output_parameter_value(
                        &mut model.csm_get_mut_parameter_values()[index],
                        model.csm_get_parameter_minimum_values()[index],
                        model.csm_get_parameter_maximum_values()[index],
                        self.previous_rig_outputs[setting_index][i] * (1.0 - weight)
                            + self.current_rig_outputs[setting_index][i] * weight,
                        output,
                    )
                } else {
                    continue;
                }
            }
        }
    }

    /// see: https://github.com/Live2D/CubismNativeFramework/blob/cbd4dfaa5ee95218ea3f9af30f8525c60b4a9b36/src/Physics/CubismPhysics.cpp#L661
    pub fn evaluate(&mut self, model: &mut Live2DModelResource, delta_time: f32) {
        // L856
        if 0.0 >= delta_time {
            return;
        }
        // dbg!(model.csm_get_parameter_values());
        // dbg!(model.csm_get_parameter_default_values());
        // L685
        self.current_remain_time += delta_time;
        if self.current_remain_time > MAX_DELTA_TIME {
            self.current_remain_time = 0.0;
        }

        // L696
        if self.parameter_cache.len() < model.csm_get_parameter_count() {
            self.parameter_cache = Vec::with_capacity(model.csm_get_parameter_count());
            self.parameter_cache
                .resize(model.csm_get_parameter_count(), 0.0);
        }
        if self.parameter_input_cache.len() < model.csm_get_parameter_count() {
            self.parameter_input_cache = Vec::with_capacity(model.csm_get_parameter_count());
            self.parameter_input_cache
                .resize(model.csm_get_parameter_count(), 0.0);
            for (i, param) in model.iter_parameters().enumerate() {
                self.parameter_input_cache[i] = *param.value;
            }
        }

        // L890
        let physics_delta_time = if let Some(fps) = self.physics_rig.fps {
            if fps > 0.0 {
                1.0 / fps
            } else {
                delta_time
            }
        } else {
            delta_time
        };

        // L899
        while self.current_remain_time >= physics_delta_time {
            // L902
            // copyRigOutputs _currentRigOutputs to _previousRigOutputs
            for setting_index in 0..self.physics_rig.sub_rig_count {
                let current_setting = &self.physics_rig.settings[setting_index];
                // let current_output = &this.physics_rig.outputs[setting_index as usize];
                for i in 0..current_setting.output_count {
                    self.previous_rig_outputs[setting_index][i] =
                        self.current_rig_outputs[setting_index][i];
                }
            }

            // L916
            // 入力キャッシュとパラメータで線形補間してUpdateParticlesするタイミングでの入力を計算する。
            // _parameterCacheはグループ間での値の伝搬の役割があるので_parameterInputCacheとの分離が必要。
            let input_weight = physics_delta_time / self.current_remain_time;

            // L917
            for (j, param) in model.iter_parameters().enumerate() {
                self.parameter_cache[j] = self.parameter_input_cache[j] * (1.0 - input_weight)
                    + *param.value * input_weight;
                self.parameter_input_cache[j] = self.parameter_cache[j];
            }

            // L923
            for setting_index in 0..self.physics_rig.sub_rig_count {
                let mut total_angle = 0.0;
                let mut total_translation = CubismVector2::default();
                let current_setting = &self.physics_rig.settings[setting_index];
                // let current_input = &this.physics_rig.inputs[current_setting.base_input_index];
                // let current_output = &this.physics_rig.outputs[current_setting.base_output_index];
                // let currentParticles = &this.physics_rig.particles[current_setting.base_particle_index];

                // Load input parameters.
                // L934
                for input in self
                    .physics_rig
                    .inputs
                    .iter_mut()
                    .skip(current_setting.base_input_index)
                    .take(current_setting.input_count)
                {
                    let weight = input.weight / MAXIMUM_WEIGHT;
                    // L938
                    if input.source_parameter_index.is_none() {
                        input.source_parameter_index =
                            Some(model.get_parameter_index(&input.source.id));
                    }

                    // L943
                    input
                        .cubism_physics_input_type
                        .get_normalized_parameter_value(
                            &mut total_translation,
                            &mut total_angle,
                            self.parameter_cache[input.source_parameter_index.unwrap()],
                            model.csm_get_parameter_minimum_values()
                                [input.source_parameter_index.unwrap()],
                            model.csm_get_parameter_maximum_values()
                                [input.source_parameter_index.unwrap()],
                            model.csm_get_parameter_default_values()
                                [input.source_parameter_index.unwrap()],
                            &current_setting.normalization_position,
                            &current_setting.normalization_angle,
                            input.reflect,
                            weight,
                        );
                }

                // L957
                let rad_angle = (-total_angle).to_radians();

                total_translation.x =
                    total_translation.x * rad_angle.cos() - total_translation.y * rad_angle.sin();
                total_translation.y =
                    total_translation.x * rad_angle.sin() + total_translation.y * rad_angle.cos();

                // L962
                // Calculate particles position.
                update_particles(
                    &mut self
                        .physics_rig
                        .particles
                        .iter_mut()
                        .skip(current_setting.base_particle_index)
                        .collect::<_>(),
                    current_setting.particle_count,
                    total_translation,
                    total_angle,
                    CubismVector2 { x: 0.0, y: 0.0 },
                    MOVEMENT_THRESHOLD * current_setting.normalization_position.maximum,
                    physics_delta_time,
                    AIR_RESISTANCE,
                );

                // L974
                // Update output parameters.
                for (i, output) in self
                    .physics_rig
                    .outputs
                    .iter_mut()
                    .skip(current_setting.base_output_index)
                    .take(current_setting.output_count)
                    .enumerate()
                {
                    let particle_index = output.vertex_index;
                    // L797
                    if output.destination_parameter_index.is_none() {
                        output.destination_parameter_index =
                            Some(model.get_parameter_index(&output.destination.id));
                    }

                    if particle_index < 1 || particle_index >= current_setting.particle_count {
                        continue;
                    }

                    let translation = {
                        let index = current_setting.base_particle_index + particle_index;

                        self.physics_rig.particles[index].position
                            - self.physics_rig.particles[index - 1].position
                        // CubismVector2 {
                        //     x: self.physics_rig.particles[index].position.x
                        //         - self.physics_rig.particles[index - 1].position.x,
                        //     y: self.physics_rig.particles[index].position.y
                        //         - self.physics_rig.particles[index - 1].position.y,
                        // }
                    };

                    let output_value = output.get_value.get_value(
                        translation,
                        self.physics_rig
                            .particles
                            .iter()
                            .skip(current_setting.base_particle_index)
                            .collect::<_>(),
                        particle_index,
                        output.reflect,
                        CubismVector2 { x: 0.0, y: -1.0 },
                    );

                    self.current_rig_outputs[setting_index][i] = output_value;

                    update_output_parameter_value(
                        &mut self.parameter_cache[output.destination_parameter_index.unwrap()],
                        model.csm_get_parameter_minimum_values()
                            [output.destination_parameter_index.unwrap()],
                        model.csm_get_parameter_maximum_values()
                            [output.destination_parameter_index.unwrap()],
                        output_value,
                        output,
                    );
                }
            }

            self.current_remain_time -= physics_delta_time;
        }
        let alpha = self.current_remain_time / physics_delta_time;

        self.interpolate(&model, alpha);
    }
}

fn update_particles2(
    strand: &Vec<CubismPhysicsParticle>,
    strand_count: usize,
    total_translation: CubismVector2,
    total_angle: f32,
    wind_direction: CubismVector2,
    threshold_value: f32,
    delta_time_seconds: f32,
    air_resistance: f32,
) -> Vec<CubismPhysicsParticle> {
    let mut ret = vec![];
    let total_redian = total_angle.to_radians();
    let current_gravity = total_redian.to_direction().normalize();

    ret.push(CubismPhysicsParticle {
        position: total_translation,
        ..strand[0]
    });

    for w in strand.windows(2) {
        let (prev, next) = (&w[0], &w[1]);
        let last_position = next.position;
        let radian = next.last_gravity.to_radian(&current_gravity) / air_resistance;

        let mut direction = next.position - prev.position;
        direction.x = (radian.cos() * direction.x) - (direction.y * radian.sin());
        direction.y = (radian.sin() * direction.x) + (direction.y * radian.cos());
        let delay = next.delay * delta_time_seconds * 30.0;
        let velocity = CubismVector2 {
            x: next.velocity.x * delay,
            y: next.velocity.y * delay,
        };

        let force = ((current_gravity * next.acceleration) + wind_direction) * delay * delay;
        let new_direction = (direction + velocity + force).normalize();
        let mut next_potison = prev.position + (new_direction * next.radius);

        if next_potison.x.abs() < threshold_value {
            next_potison.x = 0.0;
        }

        let next_velocity = if delay != 0.0 {
            (next.position - next.last_position) / delay * next.mobility
        } else {
            next.velocity
        };

        let next_particle = CubismPhysicsParticle {
            last_position,
            force: CubismVector2::default(),
            velocity: next_velocity,
            ..next.clone()
        };

        ret.push(next_particle);
    }

    ret
}

// L277
fn update_particles(
    strand: &mut Vec<&mut CubismPhysicsParticle>,
    strand_count: usize,
    total_translation: CubismVector2,
    total_angle: f32,
    wind_direction: CubismVector2,
    threshold_value: f32,
    delta_time_seconds: f32,
    air_resistance: f32,
) {
    strand[0].position = total_translation;
    let total_redian = total_angle.to_radians();
    let current_gravity = total_redian.to_direction().normalize();

    for i in 1..strand_count {
        // strand[i].force = (current_gravity * strand[i].acceleration) + wind_direction;
        strand[i].last_position = strand[i].position;
        let radian = strand[i].last_gravity.to_radian(&current_gravity) / air_resistance;
        // let mut direction = CubismVector2 {
        //     x: strand[i].position.x - strand[i - 1].position.x,
        //     y: strand[i].position.y - strand[i - 1].position.y,
        // };

        let mut direction = strand[i].position - strand[i - 1].position;
        direction.x = (radian.cos() * direction.x) - (direction.y * radian.sin());
        direction.y = (radian.sin() * direction.x) + (direction.y * radian.cos());

        // strand[i].position = strand[i - 1].position + direction;

        let delay = strand[i].delay * delta_time_seconds * 30.0;
        let velocity = CubismVector2 {
            x: strand[i].velocity.x * delay,
            y: strand[i].velocity.y * delay,
        };

        // let force = strand[i].force * delay * delay;
        let force = ((current_gravity * strand[i].acceleration) + wind_direction) * delay * delay;
        // strand[i].position = strand[i].position + velocity + force;
        // strand[i].position = strand[i - 1].position + direction + velocity + force;

        // let new_direction = (strand[i].position - strand[i - 1].position).normalize();
        let new_direction = (direction + velocity + force).normalize();
        strand[i].position = strand[i - 1].position + (new_direction * strand[i].radius);

        if strand[i].position.x.abs() < threshold_value {
            strand[i].position.x = 0.0;
        }

        if delay != 0.0 {
            // strand[i].velocity.x = strand[i].position.x - strand[i].last_position.x;
            // strand[i].velocity.y = strand[i].position.y - strand[i].last_position.y;

            // strand[i].velocity = strand[i].position - strand[i].last_position;
            // strand[i].velocity /= delay;
            // strand[i].velocity = strand[i].velocity * strand[i].mobility;
            strand[i].velocity =
                (strand[i].position - strand[i].last_position) / delay * strand[i].mobility;
        }

        strand[i].force = CubismVector2::default();
        strand[i].last_gravity = current_gravity;
    }
}

// L350
fn update_output_parameter_value(
    parameter_value: &mut f32,
    parameter_value_minimum: f32,
    parameter_value_maximum: f32,
    translation: f32,
    output: &mut CubismPhysicsOutput,
) {
    let output_scale = output
        .get_scale
        .get_output_scale_translation(output.translation_scale, output.angle_scale);
    let mut value = translation * output_scale;

    if value < parameter_value_minimum {
        if value < output.value_below_minimum {
            output.value_below_minimum = value;
        }
        value = parameter_value_minimum;
    } else if value > parameter_value_maximum {
        if value > output.value_exceeded_maximum {
            output.value_exceeded_maximum = value;
        }
        value = parameter_value_maximum;
    }

    let weight = output.weight / MAXIMUM_WEIGHT;

    if weight >= 1.0 {
        *parameter_value = value;
    } else {
        value = (*parameter_value * (1.0 - weight)) + (value + weight);
        *parameter_value = value;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iter_skip_take() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let mut vec_iter = vec.iter().skip(3).take(3);

        assert_eq!(vec_iter.next(), Some(&4));
        assert_eq!(vec_iter.next(), Some(&5));
        assert_eq!(vec_iter.next(), Some(&6));
        assert_eq!(vec_iter.next(), None);
    }

    // L1053
    #[test]
    fn test_update_output_parameter_value() {
        use super::*;

        let mut parameter_value = -1.0;

        let mut physic_output = CubismPhysicsOutput {
            destination: CubismPhysicsParameter {
                id: "ParamHairFront".to_string(),
                target_type: CubismPhysicsTargetType::CubismPhysicsTargetTypeParameter,
            },
            destination_parameter_index: Some(36),
            vertex_index: 1,
            translation_scale: CubismVector2 { x: 0.0, y: 0.0 },
            angle_scale: 1.52199996,
            weight: 100.0,
            output_type: CubismPhysicsSource::CubismPhysicsSourceAngle,
            reflect: false,
            value_below_minimum: 0.0,
            value_exceeded_maximum: 0.0,
            get_value: GetOutputTranslation::Angle,
            get_scale: GetOutputScaleTranslationType::Angle,
        };

        update_output_parameter_value(&mut parameter_value, -1.0, 1.0, 0.0, &mut physic_output);

        assert_eq!(parameter_value, 0.0);
        assert_eq!(physic_output.value_below_minimum, 0.0);
        assert_eq!(physic_output.value_exceeded_maximum, 0.0);

        let mut physic_output = CubismPhysicsOutput {
            destination: CubismPhysicsParameter {
                id: "ParamHairFront".to_string(),
                target_type: CubismPhysicsTargetType::CubismPhysicsTargetTypeParameter,
            },
            destination_parameter_index: Some(50),
            vertex_index: 2,
            translation_scale: CubismVector2 { x: 0.0, y: 0.0 },
            angle_scale: 20.0,
            weight: 100.0,
            output_type: CubismPhysicsSource::CubismPhysicsSourceAngle,
            reflect: false,
            value_below_minimum: 0.0,
            value_exceeded_maximum: 0.0,
            get_value: GetOutputTranslation::Angle,
            get_scale: GetOutputScaleTranslationType::Angle,
        };

        update_output_parameter_value(
            &mut parameter_value,
            -45.0,
            45.0,
            0.213295937,
            &mut physic_output,
        );

        assert_eq!(parameter_value, 4.26591873);
        assert_eq!(physic_output.value_below_minimum, 0.0);
        assert_eq!(physic_output.value_exceeded_maximum, 0.0);
    }

    #[test]
    fn test_normalize_parameter_value() {
        use super::*;

        assert_eq!(
            normalize_parameter_value(1.31227696, -30.0, 30.0, 0.0, -10.0, 10.0, 0.0, false)
                * 0.600000024,
            -0.262455404
        );

        assert_eq!(
            normalize_parameter_value(-3.26054907, -30.0, 30.0, 0.0, -10.0, 10.0, 0.0, false)
                * 0.600000024,
            0.652109861
        );

        assert_eq!(
            normalize_parameter_value(-1.99831104, -10.0, 10.0, 0.0, -10.0, 10.0, 0.0, false)
                * 0.400000006,
            0.79932445
        );

        assert_eq!(
            normalize_parameter_value(10000.0, -30.0, 30.0, 0.0, -10.0, 10.0, 0.0, false)
                * 0.600000024,
            -6.0
        );
    }

    #[test]
    fn test_update_particles() {
        use super::*;

        let mut strand_1 = CubismPhysicsParticle {
            initial_position: CubismVector2 { x: 0.0, y: 0.0 },
            mobility: 1.0,
            delay: 1.0,
            acceleration: 1.0,
            radius: 0.0,
            position: CubismVector2 {
                x: -9.84807777,
                y: -1.71010089,
            },
            last_position: CubismVector2 { x: 0.0, y: 0.0 },
            last_gravity: CubismVector2 { x: 0.0, y: 1.0 },
            force: CubismVector2 { x: 0.0, y: 0.0 },
            velocity: CubismVector2 { x: 0.0, y: 0.0 },
        };

        let mut strand_2 = CubismPhysicsParticle {
            initial_position: CubismVector2 { x: 0.0, y: 3.0 },
            mobility: 0.949999988,
            delay: 0.899999976,
            acceleration: 1.5,
            radius: 3.0,
            position: CubismVector2 {
                x: -7.20117282,
                y: -0.298047066,
            },
            last_position: CubismVector2 { x: 0.0, y: 3.0 },
            last_gravity: CubismVector2 {
                x: -0.173648208,
                y: 0.984807789,
            },
            force: CubismVector2 { x: 0.0, y: 0.0 },
            velocity: CubismVector2 {
                x: -25.3374615,
                y: -11.6042404,
            },
        };

        let mut strand = vec![&mut strand_1, &mut strand_2];

        update_particles(
            &mut strand,
            2,
            CubismVector2 {
                x: -9.84807777,
                y: -1.71010089,
            },
            -10.0,
            CubismVector2 { x: 0.0, y: 0.0 },
            0.0100000007,
            0.00999999977,
            5.0,
        );

        assert_eq!(
            strand_1,
            CubismPhysicsParticle {
                initial_position: CubismVector2 { x: 0.0, y: 0.0 },
                mobility: 1.0,
                delay: 1.0,
                acceleration: 1.0,
                radius: 0.0,
                position: CubismVector2 {
                    x: -9.84807777,
                    y: -1.71010089,
                },
                last_position: CubismVector2 { x: 0.0, y: 0.0 },
                last_gravity: CubismVector2 { x: 0.0, y: 1.0 },
                force: CubismVector2 { x: 0.0, y: 0.0 },
                velocity: CubismVector2 { x: 0.0, y: 0.0 },
            }
        );

        assert_eq!(
            strand_2,
            CubismPhysicsParticle {
                initial_position: CubismVector2 { x: 0.0, y: 3.0 },
                mobility: 0.949999988,
                delay: 0.899999976,
                acceleration: 1.5,
                radius: 3.0,
                position: CubismVector2 {
                    x: -12.649684,
                    y: -2.78294802,
                },
                last_position: CubismVector2 {
                    x: -7.20117282,
                    y: -0.298047066
                },
                last_gravity: CubismVector2 {
                    x: -0.173648208,
                    y: 0.984807789,
                },
                force: CubismVector2 { x: 0.0, y: 0.0 },
                velocity: CubismVector2 {
                    x: -19.1706886,
                    y: -8.74317073,
                },
            }
        );
    }

    // #[test]
    // fn radian_to_direction_test() {
    //     use super::*;

    //     assert_eq!(
    //         radian_to_direction(-0.174532935),
    //         CubismVector2 {
    //             x: -0.1736482,
    //             y: 0.9848077
    //         }
    //     );
    // }

    #[test]
    fn test_update_particles2() {
        use super::*;

        let mut strand_1 = CubismPhysicsParticle {
            initial_position: CubismVector2 { x: 0.0, y: 0.0 },
            mobility: 1.0,
            delay: 1.0,
            acceleration: 1.0,
            radius: 0.0,
            position: CubismVector2 {
                x: -9.84807777,
                y: -1.71010089,
            },
            last_position: CubismVector2 { x: 0.0, y: 0.0 },
            last_gravity: CubismVector2 { x: 0.0, y: 1.0 },
            force: CubismVector2 { x: 0.0, y: 0.0 },
            velocity: CubismVector2 { x: 0.0, y: 0.0 },
        };

        let mut strand_2 = CubismPhysicsParticle {
            initial_position: CubismVector2 { x: 0.0, y: 3.0 },
            mobility: 0.949999988,
            delay: 0.899999976,
            acceleration: 1.5,
            radius: 3.0,
            position: CubismVector2 {
                x: -7.20117282,
                y: -0.298047066,
            },
            last_position: CubismVector2 { x: 0.0, y: 3.0 },
            last_gravity: CubismVector2 {
                x: -0.173648208,
                y: 0.984807789,
            },
            force: CubismVector2 { x: 0.0, y: 0.0 },
            velocity: CubismVector2 {
                x: -25.3374615,
                y: -11.6042404,
            },
        };

        let mut strand = vec![strand_1, strand_2];

        update_particles2(
            &strand,
            2,
            CubismVector2 {
                x: -9.84807777,
                y: -1.71010089,
            },
            -10.0,
            CubismVector2 { x: 0.0, y: 0.0 },
            0.0100000007,
            0.00999999977,
            5.0,
        );

        assert_eq!(
            strand[0],
            CubismPhysicsParticle {
                initial_position: CubismVector2 { x: 0.0, y: 0.0 },
                mobility: 1.0,
                delay: 1.0,
                acceleration: 1.0,
                radius: 0.0,
                position: CubismVector2 {
                    x: -9.84807777,
                    y: -1.71010089,
                },
                last_position: CubismVector2 { x: 0.0, y: 0.0 },
                last_gravity: CubismVector2 { x: 0.0, y: 1.0 },
                force: CubismVector2 { x: 0.0, y: 0.0 },
                velocity: CubismVector2 { x: 0.0, y: 0.0 },
            }
        );

        assert_eq!(
            strand[1],
            CubismPhysicsParticle {
                initial_position: CubismVector2 { x: 0.0, y: 3.0 },
                mobility: 0.949999988,
                delay: 0.899999976,
                acceleration: 1.5,
                radius: 3.0,
                position: CubismVector2 {
                    x: -12.649684,
                    y: -2.78294802,
                },
                last_position: CubismVector2 {
                    x: -7.20117282,
                    y: -0.298047066
                },
                last_gravity: CubismVector2 {
                    x: -0.173648208,
                    y: 0.984807789,
                },
                force: CubismVector2 { x: 0.0, y: 0.0 },
                velocity: CubismVector2 {
                    x: -19.1706886,
                    y: -8.74317073,
                },
            }
        );
    }
}
