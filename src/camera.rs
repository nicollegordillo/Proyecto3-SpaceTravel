use nalgebra_glm::{Vec3, rotate_vec3};
use std::f32::consts::PI;

pub enum CameraMode {
    Normal,
    BirdsEye,
}

pub struct Camera {
  pub eye: Vec3,
  pub center: Vec3,
  pub up: Vec3,
  pub has_changed: bool,
  pub mode: CameraMode,
}

impl Camera {
  pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
    Camera {
      eye,
      center,
      up,
      has_changed: true,
      mode: CameraMode::Normal,
    }
  }

  pub fn switch_to_birds_eye(&mut self) {
        self.eye = Vec3::new(0.0, 20.0, 0.0); // Set camera above the scene
        self.center = Vec3::new(0.0, 0.0, 0.0); // Look at the origin
        self.up = Vec3::new(0.0, 0.0, -1.0); // Adjust 'up' vector for downward view
        self.mode = CameraMode::BirdsEye;
    }

    pub fn switch_to_normal(&mut self) {
        self.eye = Vec3::new(0.0, 0.0, 20.0); // Reset to original position
        self.center = Vec3::new(0.0, 0.0, 0.0);
        self.up = Vec3::new(0.0, 1.0, 0.0);
        self.mode = CameraMode::Normal;
    }

  pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
    let forward = (self.center - self.eye).normalize();
    let right = forward.cross(&self.up).normalize();
    let up = right.cross(&forward).normalize();

    let rotated = 
    vector.x * right +
    vector.y * up +
    - vector.z * forward;

    rotated.normalize()
  }

  pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
    let radius_vector = self.eye - self.center;
    let radius = radius_vector.magnitude();

    let current_yaw = radius_vector.z.atan2(radius_vector.x);

    let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
    let current_pitch = (-radius_vector.y).atan2(radius_xz);

    let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
    let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

    let new_eye = self.center + Vec3::new(
      radius * new_yaw.cos() * new_pitch.cos(),
      -radius * new_pitch.sin(),
      radius * new_yaw.sin() * new_pitch.cos()
    );

    self.eye = new_eye;
    self.has_changed = true;
  }

  pub fn zoom(&mut self, delta: f32) {
    let direction = (self.center - self.eye).normalize();
    self.eye += direction * delta;
    self.has_changed = true;
  }

  pub fn move_center(&mut self, direction: Vec3) {
    let radius_vector = self.center - self.eye;
    let radius = radius_vector.magnitude();

    let angle_x = direction.x * 0.05; // Adjust this factor to control rotation speed
    let angle_y = direction.y * 0.05;

    let rotated = rotate_vec3(&radius_vector, angle_x, &Vec3::new(0.0, 1.0, 0.0));

    let right = rotated.cross(&self.up).normalize();
    let final_rotated = rotate_vec3(&rotated, angle_y, &right);

    self.center = self.eye + final_rotated.normalize() * radius;
    self.has_changed = true;
  }
}