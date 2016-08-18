use render::DisplayData;
use glium::vertex::VertexBuffer;
use glium::texture::{SrgbTexture2d, RawImage2d, Texture2d};
use std::io::Cursor;
use image;
use glium::index::{NoIndices, PrimitiveType};
use glium::Surface;

pub struct Model {
    shape: VertexBuffer<Vertex>,
    position: [[f32; 4]; 4],
    diffuse_texture: SrgbTexture2d,
    normal_texture: Texture2d,
}

impl Model {
    pub fn new(display: &DisplayData) -> Model {
        let shape = VertexBuffer::new(&display.display, &[
            Vertex { position: [-1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
            Vertex { position: [1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
        ]).unwrap();

        let diffuse_texture = Model::load_image(include_bytes!("../assets/tuto-14-diffuse.jpg"), image::JPEG);
        let diffuse_texture = SrgbTexture2d::new(&display.display, diffuse_texture).unwrap();

        let normal_texture = Model::load_image(include_bytes!("../assets/tuto-14-normal.png"), image::PNG);
        let normal_texture = Texture2d::new(&display.display, normal_texture).unwrap();

        let position = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        Model {
            shape: shape,
            position: position,
            diffuse_texture: diffuse_texture,
            normal_texture: normal_texture,
        }
    }

    fn load_image<'a>(bytes: &[u8], encoding: image::ImageFormat) -> RawImage2d<'a, u8> {
        let image = image::load(Cursor::new(bytes), encoding).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions)
    }

    pub fn render<F>(&self, display_data: &DisplayData, target: &mut F) where F: Surface {
        target.draw(&self.shape, NoIndices(PrimitiveType::TriangleStrip), &display_data.program,
                    &uniform! { model: self.position, view: display_data.view, perspective: display_data.perspective,
                                u_light: display_data.light, diffuse_tex: &self.diffuse_texture, normal_tex: &self.normal_texture },
                    &display_data.draw_parameters).unwrap();
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coords);