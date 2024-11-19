use nalgebra_glm::{Vec2, Vec3};
use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
    pub color: Color,
    pub transformed_position: Vec3,
    pub transformed_normal: Vec3,
}

impl Vertex {
    /// Creates a new vertex with the given position, normal, and texture coordinates.
    /// You can also specify a color, with a default of white if not provided.
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
            color: Color::black(),
            transformed_position: position,
            transformed_normal: normal,
          }
    }

    /// Creates a new vertex with a specified position and color, using defaults for other fields.
    pub fn new_with_color(position: Vec3, color: Color) -> Self {
        Vertex {
            position,
            normal: Vec3::new(0.0, 0.0, 0.0), // Default normal
            tex_coords: Vec2::new(0.0, 0.0), // Default texture coordinates
            color,
            transformed_position: Vec3::new(0.0, 0.0, 0.0), // Default transformed position
            transformed_normal: Vec3::new(0.0, 0.0, 0.0), // Default transformed normal
        }
    }

    /// Sets the transformed position and normal for the vertex.
    pub fn set_transformed(&mut self, position: Vec3, normal: Vec3) {
        self.transformed_position = position;
        self.transformed_normal = normal;
    }
}
impl Default for Vertex {
  fn default() -> Self {
    Vertex {
      position: Vec3::new(0.0, 0.0, 0.0),
      normal: Vec3::new(0.0, 1.0, 0.0),
      tex_coords: Vec2::new(0.0, 0.0),
      color: Color::black(),
      transformed_position: Vec3::new(0.0, 0.0, 0.0),
      transformed_normal: Vec3::new(0.0, 1.0, 0.0),
    }
  }
}