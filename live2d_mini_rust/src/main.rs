use std::{collections::HashMap, fs::File, io::BufReader, path::Path, rc::Rc};

use image::EncodableLayout;
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
    model: live2d_mini::model::Live2DModel,
    // vertex_vec: Vec<Live2DVector2>,
    max: Vec<usize>,
    // anime: live2d_mini::animation::Animation,
    start_time: f64,
    last_frame: f64,
    textures: Vec<Texture>, // model: live2d_mini::model_resource::Live2DModelResource,
}
impl Stage {
    pub fn new(ctx: &mut Context) -> Self {
        let mut model =
            live2d_mini::model::Live2DModel::new("./live2d_mini_rust/resources/Hiyori/Hiyori.model3.json");
        let textures = model
            .textures
            .iter()
            .map(|tex| {
                Texture::from_rgba8(ctx, tex.width() as _, tex.height() as _, tex.as_bytes())
            })
            .collect::<Vec<Texture>>();

        let mut indices4 = vec![];
        let mut bindings_vec = vec![];

        model.reset_animation(1);
        model.evaluate_physic(0.01);
        model.resource.update();

        // dbg!(&model.physics);

        for (index, drawable) in model.resource.iter_sorted_drawables().enumerate() {
            if drawable.dynamic_flag().is_csm_is_visible() && drawable.indices().is_some() {
                let mut vertices4 = vec![];
                for (pos, uv) in drawable
                    .vertex_positions()
                    .iter()
                    .zip(drawable.vertex_uvs())
                {
                    vertices4.push(Vertex {
                        pos: Vec2 {
                            x: pos.x(),
                            y: pos.y(),
                        },
                        uv: Vec2 {
                            x: uv.x(),
                            y: uv.y(),
                        },
                    });
                }

                let buf = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices4);

                bindings_vec.push(Bindings {
                    vertex_buffers: vec![buf],
                    index_buffer: Buffer::immutable(
                        ctx,
                        BufferType::IndexBuffer,
                        drawable.indices().unwrap_or(&[]),
                    ),
                    images: vec![textures[*drawable.texture_index() as usize]],
                });

                indices4.push(drawable.indices().unwrap_or(&[]).len());
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
        // dbg!("----------------------------------------------------------------------------");
        Stage {
            pipeline: pipeline1,
            bindings: bindings_vec,
            model,
            // vertex_vec:
            max: indices4,
            // anime: anime1,
            start_time: time,
            last_frame: 0.0,
            textures, // model,
        }
    }
}

impl<'a> EventHandler for Stage {
    fn update(&mut self, ctx: &mut Context) {
        self.last_frame += 0.02;

        if self.last_frame > self.model.get_animation().unwrap().duration.into() {
            self.last_frame = 0.0;
            self.model
                .get_mut_animation()
                .unwrap()
                .reset_evaluate_indeies();
        }
        // self.model.evaluate_physic(0.01);
        self.model.animation(self.last_frame as f32);
        self.model.evaluate_physic(0.01);

        // self.model.resource.csm_update_model();

        self.model.resource.update();

        let mut indices4 = vec![];
        let mut bindings_vec = vec![];

        for drawable in self.model.resource.iter_sorted_drawables() {
            if drawable.dynamic_flag().is_csm_is_visible() && drawable.indices().is_some() {
                // dbg!(&drawable.id());
                let mut vertices4 = vec![];
                for (pos, uv) in drawable
                    .vertex_positions()
                    .iter()
                    .zip(drawable.vertex_uvs())
                {
                    vertices4.push(Vertex {
                        pos: Vec2 {
                            x: pos.x(),
                            y: pos.y(),
                        },
                        uv: Vec2 {
                            x: uv.x(),
                            y: uv.y(),
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
                    images: vec![self.textures[*drawable.texture_index() as usize]],
                });

                indices4.push(drawable.indices().unwrap().len());
            }
        }

        // 全部取っ替える
        for binding in self.bindings.iter_mut() {
            binding.index_buffer.delete();
            binding.vertex_buffers[0].delete();
        }

        self.bindings = bindings_vec;
        self.max = indices4;
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
            window_title: "live2d_mini_rs_demo".to_string(),
            window_width: 1024,
            window_height: 1024,
            fullscreen: false,
            high_dpi: true,
            ..Default::default()
        },
        |mut ctx| Box::new(Stage::new(&mut ctx)),
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
