use nalgebra_glm::{Mat4, Vec3};
use crate::framebuffer::Framebuffer;
use crate::{Uniforms, vertex::Vertex};
use crate::shaders::{vertex_shader, fragment_shader_neptune, fragment_shader_jupiter, fragment_shader_venus, fragment_shader_mars, fragment_shader_earth, fragment_shader_mercury, fragment_shader_sun};

pub struct CelestialBody {
    pub name: String,
    pub vertex_array: Vec<Vertex>,
    pub shader_type: u8,
    pub orbit_radius: f32,
    pub rotation_speed: f32,
    pub orbit_speed: f32,
    pub scale: f32,
}

impl CelestialBody {
    pub fn get_model_matrix(&self, time: f32) -> Mat4 {
        let orbit_angle = self.orbit_speed * time;
        let rotation_angle = self.rotation_speed * time;

        let translation = Vec3::new(
            self.orbit_radius * orbit_angle.cos(),
            0.0,
            self.orbit_radius * orbit_angle.sin(),
        );

        let rotation = Vec3::new(0.0, rotation_angle, 0.0);

        super::create_model_matrix(translation, self.scale, rotation)
    }

    pub fn get_world_position(&self) -> Vec3 {
        Vec3::new(
            self.orbit_radius * (self.orbit_speed * self.rotation_speed).cos(),
            0.0,
            self.orbit_radius * (self.orbit_speed * self.rotation_speed).sin(),
        )
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, uniforms: &Uniforms) {
        let mut transformed_vertices = Vec::with_capacity(self.vertex_array.len());
        for vertex in &self.vertex_array {
            let transformed = crate::vertex_shader(vertex, uniforms);
            transformed_vertices.push(transformed);
        }

        let mut triangles = Vec::new();
        for i in (0..transformed_vertices.len()).step_by(3) {
            if i + 2 < transformed_vertices.len() {
                triangles.push([
                    transformed_vertices[i].clone(),
                    transformed_vertices[i + 1].clone(),
                    transformed_vertices[i + 2].clone(),
                ]);
            }
        }

        let mut fragments = Vec::new();
        for tri in &triangles {
            fragments.extend(crate::triangle(&tri[0], &tri[1], &tri[2]));
        }

        for fragment in fragments {
            let x = fragment.position.x as usize;
            let y = fragment.position.y as usize;
            if x < framebuffer.width && y < framebuffer.height {
                let shaded_color = match self.shader_type {
                    1 => fragment_shader_jupiter(&fragment, uniforms),
                    4 => fragment_shader_venus(&fragment, uniforms),
                    5 => fragment_shader_mars(&fragment, uniforms),
                    6 => fragment_shader_earth(&fragment, uniforms),
                    7 => fragment_shader_mercury(&fragment, uniforms),
                    8 => fragment_shader_sun(&fragment, uniforms),
                    _ => fragment_shader_neptune(&fragment, uniforms),
                };

                let color = shaded_color.to_hex();
                framebuffer.set_current_color(color);
                framebuffer.point(x, y, fragment.depth);
            }
        }
    }
}
