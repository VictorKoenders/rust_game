use glium::{DisplayBuild, Program, DrawParameters, Depth};
use glium::draw_parameters::DepthTest;
use glium::backend::glutin_backend::{GlutinFacade, WinRef};
use glium::glutin::WindowBuilder;
use std::f32::consts;

pub struct DisplayData<'a> {
    pub display: GlutinFacade,
    pub program: Program,
    pub perspective: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
    pub light: [f32; 3],
    pub draw_parameters: DrawParameters<'a>,
}

impl<'a> DisplayData<'a> {
    pub fn new() -> DisplayData<'a> {
        let display = WindowBuilder::new()
            .with_depth_buffer(24)
            .build_glium().unwrap();

        let vertex_shader_src = include_str!("../assets/default.vert");
        let fragment_shader_src = include_str!("../assets/default.frag");

        let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let view = view_matrix(&[0.5, 0.2, -3.0], &[-0.5, -0.2, 3.0], &[0.0, 1.0, 0.0]);

        let perspective = {
            let window: WinRef = display.get_window().unwrap();
            let (width, height) = window.get_inner_size_pixels().unwrap();

            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = consts::PI / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
            ]
        };

        let light = [1.4, 0.4, 0.7f32];

        let params = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        DisplayData {
            display: display,
            program: program,
            perspective: perspective,
            light: light,
            draw_parameters: params,
            view: view,
        }
    }
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}