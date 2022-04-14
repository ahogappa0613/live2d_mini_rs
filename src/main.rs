mod animation;
mod model_json;
mod motion_json;
mod pose_json;
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use miniquad::*;

#[derive(Debug)]
#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[derive(Debug)]
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

struct Stage {
    pipeline: Pipeline,
    bindings: Vec<Bindings>,
    // vertex_vec: Vec<Live2DVector2>,
    max: Vec<usize>,
    anime: animation::Animation,
    start_time: f64,
    last_frame: f64,
    model: live2d_mini::model::Live2DModel,
}
impl Stage {
    pub fn new(ctx: &mut Context) -> Self {
        let path =
            Path::new("/Users/ahogappa/project/live2d_mini_rs/resources/Hiyori/Hiyori.model3.json");

        let current_dir = path.parent().unwrap();

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let model_json: model_json::ModelJson = serde_json::from_reader(reader).unwrap();
        let textures: Vec<Texture> = model_json
            .FileReferences
            .Textures
            .iter()
            .map(|path| {
                let img = image::io::Reader::open(current_dir.join(path))
                    .unwrap()
                    .decode()
                    .unwrap()
                    .flipv()
                    .to_rgba8();

                Texture::from_rgba8(ctx, img.width() as _, img.height() as _, &img.into_raw())
            })
            .collect();
        // dbg!(&model_json);

        let mut indices4 = vec![];
        let model =
            live2d_mini::model::Live2DModel::new(current_dir.join(model_json.FileReferences.Moc));
        let file = File::open(current_dir.join(model_json.FileReferences.Pose.unwrap())).unwrap();
        let reader = BufReader::new(file);
        let u: pose_json::PoseJson = serde_json::from_reader(reader).unwrap();
        // dbg!(&u);
        let motions = model_json
            .FileReferences
            .Motions
            .unwrap()
            .Idle
            .iter()
            .map(|idle| {
                let file = File::open(current_dir.join(&idle.File)).unwrap();
                let reader = BufReader::new(file);
                serde_json::from_reader(reader).unwrap()
            })
            .collect::<Vec<motion_json::MotionJson>>();
        // dbg!(&motions);
        let anime1 = animation::Animation::new(&motions[2]);
        let mut bindings_vec = vec![];
        let mut group_info: HashMap<&str, isize> = HashMap::new();
        for (index, group) in u.Groups.iter().enumerate() {
            for g in group.iter() {
                group_info.insert(&g.Id, index as _);
            }
        }
        // dbg!(&group_info);
        let info = model.csm_read_canvas_info();
        let scale_x = info.out_size_in_pixels.x() / 1024.0 / 4.0;
        let scale_y = info.out_size_in_pixels.y() / 1024.0 / 4.0;
        dbg!(info.out_size_in_pixels);
        dbg!(info.out_pixels_per_unit);
        dbg!(info.out_origin_in_pixels);
        let scale = info.out_pixels_per_unit;

        // for part in model.iter_mut_parts() {
        //     if part.id() == "PartArmB" {
        //         *part.opacitiy = 0.0;
        //     }
        // }

        // dbg!(&anime1.duration);
        animation::evaluate_animation(&model, &anime1, 0.0);

        model.update();

        for (index, drawable) in model.iter_sorted_drawables().enumerate() {
            if drawable.dynamic_flag().is_csm_is_visible() && drawable.indices().is_some() {
                // dbg!(index);
                let vertex_positions = drawable.vertex_positions();
                let vertex_uvs = drawable.vertex_uvs();
                let mut vertices4 = vec![];
                for i in 0..*drawable.vertex_count() as usize {
                    vertices4.push(Vertex {
                        pos: Vec2 {
                            x: vertex_positions[i].x(),
                            y: vertex_positions[i].y(),
                        },
                        uv: Vec2 {
                            x: vertex_uvs[i].x(),
                            y: vertex_uvs[i].y(),
                        },
                    });
                }

                let buf = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices4);

                bindings_vec.push(Bindings {
                    vertex_buffers: vec![buf],
                    index_buffer: Buffer::immutable(
                        ctx,
                        BufferType::IndexBuffer,
                        drawable.indices().unwrap(),
                    ),
                    images: vec![textures[*drawable.texture_index() as usize]],
                });

                indices4.push(drawable.indices().unwrap().len());
            }
        }

        let shader1 = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        let mut param = PipelineParams::default();
        let state = BlendState::new(
            Equation::Add,
            BlendFactor::Value(BlendValue::SourceAlpha),
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
        );

        param.color_blend = Some(state);
        param.alpha_blend = Some(state);
        param.cull_face = CullFace::Back;
        let pipeline1 = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::with_buffer("pos", VertexFormat::Float2, 0),
                VertexAttribute::with_buffer("uv", VertexFormat::Float2, 0),
            ],
            shader1,
            param,
        );

        let time = miniquad::date::now();

        Stage {
            pipeline: pipeline1,
            bindings: bindings_vec,
            // vertex_vec:
            max: indices4,
            anime: anime1,
            start_time: time,
            last_frame: 0.0,
            model,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, ctx: &mut Context) {
        // let time = miniquad::date::now();
        // let delta = (time - self.start_time) / 1000.0;
        self.last_frame += 0.02;

        // dbg!(delta);
        animation::evaluate_animation(&self.model, &self.anime, self.last_frame as f32);

        self.model.update();

        let mut vertices4s = vec![];
        for (index, drawable) in self.model.iter_sorted_drawables().enumerate() {
            if drawable.dynamic_flag().is_csm_is_visible() && drawable.indices().is_some() {
                // dbg!(index);
                let vertex_positions = drawable.vertex_positions();
                let vertex_uvs = drawable.vertex_uvs();
                let mut vertices4 = vec![];
                for i in 0..*drawable.vertex_count() as usize {
                    vertices4.push(Vertex {
                        pos: Vec2 {
                            x: vertex_positions[i].x(),
                            y: vertex_positions[i].y(),
                        },
                        uv: Vec2 {
                            x: vertex_uvs[i].x(),
                            y: vertex_uvs[i].y(),
                        },
                    });
                }
                vertices4s.push(vertices4);
                // self.bindings[index].vertex_buffers[0].update(ctx, &vertices4);
            }
        }

        for (index, bind) in self.bindings.iter().enumerate() {
            bind.vertex_buffers[0].update(ctx, &vertices4s[index])
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(PassAction::default());
        ctx.apply_pipeline(&self.pipeline);

        for (index, bind) in self.bindings.iter().enumerate() {
            ctx.apply_bindings(bind);

            ctx.draw(0, self.max[index] as _, 1);
        }
        ctx.end_render_pass();

        ctx.commit_frame();
    }
}
fn main() {
    miniquad::start(
        conf::Conf {
            window_title: "Miniquad".to_string(),
            window_width: 1024,
            window_height: 1024,
            fullscreen: false,
            high_dpi: true,
            ..Default::default()
        },
        |mut ctx| UserData::owning(Stage::new(&mut ctx), ctx),
    );
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;
    varying lowp vec2 texcoord;
    void main() {
        gl_Position = vec4(pos, 0, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;
    uniform sampler2D tex1;
    void main() {
        gl_FragColor = texture2D(tex1, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex1".to_string()],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}
