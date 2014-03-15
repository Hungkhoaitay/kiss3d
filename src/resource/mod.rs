//! GPU resource managers

pub use resource::framebuffer_manager::{FramebufferManager, RenderTarget};
pub use resource::texture_manager::{Texture, TextureManager};
pub use resource::material::Material;
pub use resource::shader::{Shader, ShaderAttribute, ShaderUniform};
pub use resource::gpu_vector::{GPUVector, StaticDraw, DynamicDraw, StreamDraw, ArrayBuffer, ElementArrayBuffer, BufferType, AllocationType};
pub use resource::gl_primitive::GLPrimitive;
pub use resource::mesh::Mesh;

mod framebuffer_manager;
mod texture_manager;
pub mod material;
mod gpu_vector;
mod gl_primitive;
mod mesh;
mod shader;