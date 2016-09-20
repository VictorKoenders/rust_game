use std::collections::HashMap;
use glium::texture::{SrgbTexture2d, RawImage2d, Texture2d};
use render::DisplayData;
use image;
use std::io::Cursor;
use std::rc::Rc;
use error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Texture {
	Wall,
	WallNormal,
	PanelBackground,
}

pub enum TextureData {
	SrgbTexture(SrgbTexture2d),
	Texture(Texture2d),
}

static mut TEXTURE_DATA: Option<HashMap<Texture, Rc<TextureData>>> = None;

impl Texture {
	pub fn get(texture: Texture) -> Rc<TextureData> {
		unsafe {
			if let Some(ref data) = TEXTURE_DATA {
				if let Some(rc) = data.get(&texture) {
					return rc.clone();
				}
			}
			panic!("")
		}
	}
}

impl TextureData {
	pub fn get_srgb_texture2d(&self) -> Option<&SrgbTexture2d> {
		if let TextureData::SrgbTexture(ref data) = *self {
			Some(data)
		} else {
			None
		}
	}
	pub fn get_texture2d(&self) -> Option<&Texture2d> {
		if let TextureData::Texture(ref data) = *self {
			Some(data)
		} else {
			None
		}
	}
}

pub fn init(display: &DisplayData) -> Result<(), error::GameError>{
	let mut hashmap = HashMap::new();
	hashmap.insert(Texture::Wall, Rc::new(try!(load_srgb_texture(include_bytes!("../../assets/tuto-14-diffuse.jpg"), image::JPEG, display))));
	hashmap.insert(Texture::WallNormal, Rc::new(try!(load_texture(include_bytes!("../../assets/tuto-14-normal.png"), image::PNG, display))));
	hashmap.insert(Texture::PanelBackground, Rc::new(try!(load_texture(include_bytes!("../../assets/panel_background.png"), image::PNG, display))));
	unsafe {
		TEXTURE_DATA = Some(hashmap);
	}
	Ok(())
}

// TODO: Merge similar code in these load functions
// TODO: Find a way to load images with transparent backgrounds
fn load_srgb_texture(bytes: &[u8], encoding: image::ImageFormat, display: &DisplayData) -> Result<TextureData, error::GameError> {
	let image = try!(image::load(Cursor::new(bytes), encoding)).to_rgba();
	let image_dimensions = image.dimensions();
	let texture = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
	Ok(TextureData::SrgbTexture(try!(SrgbTexture2d::new(&display.display, texture))))
}

fn load_texture(bytes: &[u8], encoding: image::ImageFormat, display: &DisplayData) -> Result<TextureData, error::GameError> {
	let image = try!(image::load(Cursor::new(bytes), encoding)).to_rgba();
	let image_dimensions = image.dimensions();
	let texture = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
	Ok(TextureData::Texture(try!(Texture2d::new(&display.display, texture))))
}