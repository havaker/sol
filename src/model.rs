use nalgebra_glm as glm;

use std::error::Error;

use glium::{texture::RawImage2d, Display, IndexBuffer, Texture2d, VertexBuffer};
use glm::Mat4;
use image::{ImageBuffer, ImageError, Rgba};
use obj::{load_obj, Obj, ObjError, TexturedVertex};

pub struct Assets {
    obj: Obj<TexturedVertex, u16>,
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Assets {
    pub fn load(obj_path: &str, texture_path: &str) -> Result<Self, Box<dyn Error>> {
        let obj = Self::load_obj(obj_path)?;
        let img = Self::load_image(texture_path)?;

        Ok(Self { obj, img })
    }

    fn load_obj(obj_path: &str) -> Result<Obj<TexturedVertex, u16>, ObjError> {
        use std::fs::File;
        use std::io::BufReader;

        let input = BufReader::new(File::open(obj_path)?);

        Ok(load_obj(input)?)
    }

    fn load_image(path: &str) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, ImageError> {
        use image::io::Reader as ImageReader;

        Ok(ImageReader::open(path)?.decode()?.into_rgba8())
    }
}

pub struct Model {
    pub vertex_buffer: VertexBuffer<TexturedVertex>,
    pub index_buffer: IndexBuffer<u16>,
    pub texture: Texture2d,

    pub transform: Mat4,
}

impl Model {
    pub fn new(assets: &Assets, display: &Display) -> Result<Self, Box<dyn Error>> {
        let dimensions = assets.img.dimensions();
        let img = RawImage2d::from_raw_rgba_reversed(&assets.img.clone().into_raw(), dimensions);

        Ok(Self {
            vertex_buffer: assets.obj.vertex_buffer(display)?,
            index_buffer: assets.obj.index_buffer(display)?,
            texture: glium::texture::Texture2d::new(display, img)?,
            transform: glm::identity(),
        })
    }
}
