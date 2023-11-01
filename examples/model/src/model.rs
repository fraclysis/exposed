use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind},
    mem::size_of,
    path::PathBuf,
};

use exposed::log::log_error;
use gl::types::GLuint;
use image::ImageFormat;
use nalgebra_glm::Mat4;

use crate::{
    gl_log,
    renderer::{GlBuffer, GlTexture, GlVao, PmxRenderer},
};

pub struct Pmx {
    pub model: Mat4,
    pub gl_materials: Vec<Material>,
    pub gl_textures: Vec<GlTexture>,
    pub index_data: GlBuffer,
    pub vertex_data: GlBuffer,
    pub vao: GlVao,
}

#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub add_uv: [[f32; 4]; 4],
    pub bone_indices: [f32; 4],
    pub bone_weights: [f32; 4],
    pub weight_formula: f32,
}

impl Pmx {
    pub fn new(path: &str, _: &PmxRenderer) -> Result<Self, Error> {
        let base_path = get_parent_folder(path);
        let m = match PMXUtil::reader::ModelInfoStage::open(path) {
            Some(m) => m,
            None => return Err(ErrorKind::NotFound.into()),
        };

        let _header = m.get_header();
        let (_model_info, rn) = m.read();
        let (vertices, rn) = rn.read();
        let (faces, rn) = rn.read();
        let (textures, rn) = rn.read();
        let (materials, rn) = rn.read();
        let (_bones, rn) = rn.read();
        let (_morphs, rn) = rn.read();
        let (_frames, rn) = rn.read();
        let (_rigid_bodies, rn) = rn.read();
        let (_joints, _) = rn.read();

        let mut gl_textures: Vec<GlTexture> = Vec::with_capacity(textures.len());
        let mut gl_materials: Vec<Material> = Vec::with_capacity(materials.len());

        for texture_name in textures {
            let mut texture_path = base_path.clone();
            texture_path.push(&texture_name);

            let texture_path = match texture_path.to_str() {
                Some(s) => s,
                None => {
                    log_error!("Model", "Utf error {texture_path:?}.");
                    continue;
                }
            };

            let texture_path = texture_path.replace('\\', "/");

            unsafe {
                let mut texture = 0;
                gl::GenTextures(1, &mut texture);

                gl_textures.push(GlTexture(texture));

                let image_file = match File::open(&texture_path) {
                    Ok(f) => f,
                    Err(e) => {
                        log_error!("Model", "Failed to open {texture_path}. {e}");
                        continue;
                    }
                };

                let image = match image::load(BufReader::new(image_file), ImageFormat::Png) {
                    Ok(image) => image,
                    Err(e) => {
                        log_error!("Image loader error", "Failed to load image {texture_name}. {e}");
                        continue;
                    }
                };

                let image_data = image.to_rgba8();

                gl::BindTexture(gl::TEXTURE_2D, texture);

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    image.width() as i32,
                    image.height() as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    image_data.as_ptr() as *const _,
                );

                gl::GenerateMipmap(gl::TEXTURE_2D);

                gl_log!();
            }
        }

        let mut vertex_list = Vec::with_capacity(vertices.len());

        for v in vertices {
            let weight_formula;
            let bone_weights;
            let bone_indices;

            match v.weight_type {
                PMXUtil::types::VertexWeight::BDEF1(w) => {
                    weight_formula = 0.0;
                    bone_indices = [w as f32, 0.0, 0.0, 0.0];
                    bone_weights = [0.0, 0.0, 0.0, 0.0];
                }
                PMXUtil::types::VertexWeight::BDEF2 { bone_index_1, bone_index_2, bone_weight_1 } => {
                    weight_formula = 1.0;
                    bone_indices = [bone_index_1 as f32, bone_index_2 as f32, 0.0, 0.0];
                    bone_weights = [bone_weight_1, 0.0, 0.0, 0.0];
                }
                PMXUtil::types::VertexWeight::BDEF4 {
                    bone_index_1,
                    bone_index_2,
                    bone_index_3,
                    bone_index_4,
                    bone_weight_1,
                    bone_weight_2,
                    bone_weight_3,
                    bone_weight_4,
                } => {
                    weight_formula = 2.0;
                    bone_indices = [bone_index_1 as f32, bone_index_2 as f32, bone_index_3 as f32, bone_index_4 as f32];
                    bone_weights = [bone_weight_1, bone_weight_2, bone_weight_3, bone_weight_4];
                }
                PMXUtil::types::VertexWeight::SDEF { .. } => todo!("Formula: 3"),
                PMXUtil::types::VertexWeight::QDEF { .. } => todo!("Formula: 4"),
            }

            vertex_list.push(Vertex {
                position: v.position,
                normal: v.norm,
                add_uv: v.add_uv,
                uv: v.uv,
                bone_indices,
                bone_weights,
                weight_formula,
            });
        }

        let mut vertex_data = 0;
        unsafe { gl::GenBuffers(1, &mut vertex_data) };
        let vertex_data = GlBuffer(vertex_data);

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_data.0);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_list.len() * size_of::<Vertex>()) as _,
                vertex_list.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            gl_log!();
        }

        let vao = unsafe {
            let mut vao = 0;
            gl_log!();

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl_log!();

            macro_rules! offset_of {
                ($base:path, $field:ident) => {{
                    #[allow(unused_unsafe)]
                    unsafe {
                        let b: $base = std::mem::zeroed();
                        std::ptr::addr_of!(b.$field) as isize - std::ptr::addr_of!(b) as isize
                    }
                }};
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_data.0);

            macro_rules! vao {
                ($index:expr, $count:expr, $gl_type:expr, $type_:ty, $field:ident) => {
                    gl::VertexAttribPointer(
                        $index,
                        $count,
                        $gl_type,
                        0,
                        std::mem::size_of::<$type_>() as _,
                        offset_of!($type_, $field) as _,
                    );
                    gl::EnableVertexAttribArray($index);
                };
            }

            vao!(0, 3, gl::FLOAT, Vertex, position);
            vao!(1, 3, gl::FLOAT, Vertex, normal);
            vao!(2, 2, gl::FLOAT, Vertex, uv);
            vao!(3, 4, gl::FLOAT, Vertex, add_uv);
            vao!(4, 4, gl::FLOAT, Vertex, bone_indices);
            vao!(5, 4, gl::FLOAT, Vertex, bone_weights);
            vao!(6, 1, gl::FLOAT, Vertex, weight_formula);

            GlVao(vao)
        };

        {
            let mut start = 0;
            for m in materials {
                let num_vertices = m.num_face_vertices as u32;

                // TODO: check for if a texture exists
                gl_materials.push(Material {
                    gl_texture: gl_textures[m.texture_index as usize].0,
                    index_start: start,
                    index_size: num_vertices,
                });

                dbg!(start..num_vertices + start);

                start += num_vertices;
            }
        }

        let index_list = {
            let mut list = Vec::new();
            for face in faces {
                list.push(face.vertices[0] as u32);
                list.push(face.vertices[1] as u32);
                list.push(face.vertices[2] as u32);
            }
            list
        };

        dbg!(index_list.len());
        dbg!(vertex_list.len());

        let mut index_data = 0;
        unsafe { gl::GenBuffers(1, &mut index_data) };
        let index_data = GlBuffer(index_data);

        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_data.0);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (index_list.len() * size_of::<u32>()) as _,
                index_list.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            gl_log!();
        }

        Ok(Self { gl_textures, vertex_data, gl_materials, index_data, vao, model: Mat4::identity() })
    }
}

impl Drop for Pmx {
    fn drop(&mut self) {}
}

fn get_parent_folder(path: &str) -> PathBuf {
    let mut path = PathBuf::from(path);
    path.pop();
    path
}

pub struct Material {
    pub gl_texture: GLuint,
    pub index_start: u32,
    pub index_size: u32,
}
