use hecs::World;
use macroquad::material::{gl_use_default_material, gl_use_material, load_material, MaterialParams};
use macroquad::miniquad::UniformDesc;
use macroquad::prelude::*;

use crate::components::{BlackHole, GameObject, Geometry, Label, PhysicsClock, Position, Weight};

const MAX_BH: usize = 8;

const VERTEX_SHADER: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
attribute vec4 normal;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}"#;

const FRAGMENT_SHADER: &str = r#"#version 100

precision highp float;
uniform vec2 iResolution;
uniform float iTime;
uniform float density;
uniform vec3 bh_0;
uniform vec3 bh_1;
uniform vec3 bh_2;
uniform vec3 bh_3;
uniform vec3 bh_4;
uniform vec3 bh_5;
uniform vec3 bh_6;
uniform vec3 bh_7;

vec2 rand2(vec2 p)
{
    p = vec2(dot(p, vec2(12.9898,78.233)), dot(p, vec2(26.65125, 83.054543)));
    return fract(sin(p) * 43758.5453);
}

float rand(vec2 p)
{
    return fract(sin(dot(p.xy ,vec2(54.90898,18.233))) * 4337.5453);
}

// Thanks to David Hoskins https://www.shadertoy.com/view/4djGRh
float stars(in vec2 x, float numCells, float size, float br)
{
    vec2 n = x * numCells;
    vec2 f = floor(n);

	float d = 1.0e10;
    for (int i = -1; i <= 1; ++i)
    {
        for (int j = -1; j <= 1; ++j)
        {
            vec2 g = f + vec2(float(i), float(j));
			g = n - g - rand2(mod(g, numCells)) + rand(g);
            // Control size
            g *= 1. / (numCells * size);
			d = min(d, dot(g, g));
        }
    }

    return br * (smoothstep(.98, 1., (1. - sqrt(d))));
}


vec2 rotateUV(vec2 uv, float rotation, vec2 mid)
{
    return vec2(
      cos(rotation) * (uv.x - mid.x) + sin(rotation) * (uv.y - mid.y) + mid.x,
      cos(rotation) * (uv.y - mid.y) - sin(rotation) * (uv.x - mid.x) + mid.y
    );
}

vec2 apply_blackhole(vec2 coord, vec3 bh) {
    float from_bh = distance(coord, bh.xy);
    float angle = 1. / log2(from_bh/bh.z + 1.0) - 1.05;

    if (angle > 0.0)
        coord = rotateUV(coord, 3.*angle, bh.xy);

    vec2 dir = coord - bh.xy;
    float factor = (exp((from_bh-bh.z)/0.005)) + bh.z;
    if (factor < 1.)
        coord = bh.xy + dir * factor;

    return coord;
}

void main()
{
    float resolution = iResolution.x;
    vec2 coord = gl_FragCoord.xy / resolution;
    bool in_bh = false;
    float black_factor = 0.08;

    if (bh_0.z > 0.0) {
        coord = apply_blackhole(coord, bh_0);
        if (distance(coord, bh_0.xy) < black_factor * bh_0.z)
            in_bh = true;
    }

    if (bh_1.z > 0.0) {
        coord = apply_blackhole(coord, bh_1);
        if (distance(coord, bh_1.xy) < black_factor * bh_1.z)
            in_bh = true;
    }

    if (bh_2.z > 0.0) {
        coord = apply_blackhole(coord, bh_2);
        if (distance(coord, bh_2.xy) < black_factor * bh_2.z)
            in_bh = true;
    }

    if (bh_3.z > 0.0) {
        coord = apply_blackhole(coord, bh_3);
        if (distance(coord, bh_3.xy) < black_factor * bh_3.z)
            in_bh = true;
    }

    if (bh_4.z > 0.0) {
        coord = apply_blackhole(coord, bh_4);
        if (distance(coord, bh_4.xy) < black_factor * bh_4.z)
            in_bh = true;
    }

    if (bh_5.z > 0.0) {
        coord = apply_blackhole(coord, bh_5);
        if (distance(coord, bh_5.xy) < black_factor * bh_5.z)
            in_bh = true;
    }

    if (bh_6.z > 0.0) {
        coord = apply_blackhole(coord, bh_6);
        if (distance(coord, bh_6.xy) < black_factor * bh_6.z)
            in_bh = true;
    }

    if (bh_7.z > 0.0) {
        coord = apply_blackhole(coord, bh_7);
        if (distance(coord, bh_7.xy) < black_factor * bh_7.z)
            in_bh = true;
    }

    coord += iTime / 800.;


    vec3 result = vec3(0.);
    if (!in_bh) {
        result += stars(coord, density, 100.0 / resolution, 2.) * vec3(.74, .74, .74);
        result += stars(coord, density * 3.0, 50.0 / resolution, 1.) * vec3(.97, .74, .74);
        result += stars(coord, density * 6.0, 25.0 / resolution, 0.5) * vec3(.9, .9, .95);
    }

    gl_FragColor = vec4(result, 1.);
}
"#;

pub struct BlackHoleRenderer {
    material: macroquad::material::Material,
}

impl BlackHoleRenderer {
    fn new() -> Option<Self> {
        let material = load_material(
            ShaderSource::Glsl { vertex: VERTEX_SHADER, fragment: FRAGMENT_SHADER },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("iResolution", UniformType::Float2),
                    UniformDesc::new("iTime", UniformType::Float1),
                    UniformDesc::new("bh_count",   UniformType::Float1),
                    UniformDesc::new("density",   UniformType::Float1),
                    UniformDesc::new("bh_0",    UniformType::Float3),
                    UniformDesc::new("bh_1",    UniformType::Float3),
                    UniformDesc::new("bh_2",    UniformType::Float3),
                    UniformDesc::new("bh_3",    UniformType::Float3),
                    UniformDesc::new("bh_4",    UniformType::Float3),
                    UniformDesc::new("bh_5",    UniformType::Float3),
                    UniformDesc::new("bh_6",    UniformType::Float3),
                    UniformDesc::new("bh_7",    UniformType::Float3),
                ],
                ..Default::default()
            },
        )
        .map_err(|e| error!("black hole shader failed: {:?}", e))
        .ok()?;

        Some(Self { material })
    }
}

pub fn system_render(world: &World, bh_renderer: &mut Option<BlackHoleRenderer>, density: f32) {
    let sw = screen_width();
    let sh = screen_height();

    // Collect black holes.
    let mut bh_data: Vec<(f32, f32, f32)> = Vec::new();
    for (_, (pos, weight, _hole)) in world.query::<(&Position, &Weight, &BlackHole)>().iter() {
        if bh_data.len() < MAX_BH {
            bh_data.push((pos.x, pos.y, weight.weight.max(5.0)));
        }
    }

    if bh_renderer.is_none() {
        *bh_renderer = BlackHoleRenderer::new();
    }

    let renderer = match bh_renderer.as_ref() {
        Some(r) => r,
        None => return,
    };

    // Write remaining accumulator back.
    if let Some((_, clock)) = world.query::<&mut PhysicsClock>().into_iter().next() {
        renderer.material.set_uniform("iTime", clock.global);
    }

    let uv_names  = ["bh_0",  "bh_1",  "bh_2", "bh_3", "bh_4", "bh_5", "bh_6", "bh_7"];

    renderer.material.set_uniform("iResolution", vec2(sw, sh));
    renderer.material.set_uniform("bh_count", bh_data.len() as f32);
    renderer.material.set_uniform("density", density);

    for i in 0..MAX_BH {
        if i < bh_data.len() {
            let (x, y, r) = bh_data[i];
            renderer.material.set_uniform(uv_names[i],  vec3(x / sw, (sh - y) / sw, r * 2.0 / sw));
        } else {
            renderer.material.set_uniform(uv_names[i],  vec3(-1.0f32, -1.0f32, 0.0f32));
        }
    }

    // Overlay the black hole visuals on top of the scene via a full-screen quad.
    gl_use_material(&renderer.material);
    draw_rectangle(0.0, 0.0, sw, sh, WHITE);
    gl_use_default_material();

    // Draw scene directly onto the screen.
    /*for (_, (pos, geo, obj)) in world.query::<(&Position, &Geometry, &GameObject)>().iter() {
        match (geo, obj) {
            (Geometry::Circle(r), GameObject::Asteroid) => {
                draw_circle(pos.x, pos.y, *r, WHITE);
                draw_circle_lines(pos.x, pos.y, *r, 2.0, WHITE);
            }
            _ => () //debug!("Unable to render object"),
        }
    }*/
    for (_, (pos, label)) in world.query::<(&Position, &Label)>().iter() {
        draw_text(&label.0, pos.x, pos.y, 24.0, WHITE);
    }
}
