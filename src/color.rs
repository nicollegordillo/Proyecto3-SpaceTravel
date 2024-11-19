// color.rs
use std::ops::{Add, Mul};
use std::fmt;
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }

    pub fn black() -> Self {
        Color { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t).round() as f32,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t).round() as f32,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t).round() as f32,
        }
    }

    pub fn blend_overlay(&self, blend: &Color) -> Color {
        // Overlay: Combines Multiply and Screen blend modes
        fn overlay_channel(base: f32, blend: f32) -> f32 {
            if base < 0.5 {
                (2.0 * base * blend).min(1.0)
            } else {
                (1.0 - 2.0 * (1.0 - base) * (1.0 - blend)).max(0.0)
            }
        }

        Color::new(
            overlay_channel(self.r, blend.r),
            overlay_channel(self.g, blend.g),
            overlay_channel(self.b, blend.b)
        )
    }

    pub fn blend_darken(&self, blend: &Color) -> Color {
        Color::new(
            self.r.min(blend.r),
            self.g.min(blend.g),
            self.b.min(blend.b)
        )
    }

    pub fn blend_lighten(&self, blend: &Color) -> Color {
        Color::new(
            self.r.max(blend.r),
            self.g.max(blend.g),
            self.b.max(blend.b)
        )
    }

    pub fn mix(a: Color, b: Color, t: f32) -> Color {
        a * (1.0 - t) + b * t // Linear interpolation
    }


    pub fn blend_color_dodge(&self, blend: &Color) -> Color {
        fn dodge_channel(base: f32, blend: f32) -> f32 {
            if blend >= 255.0 {
                255.0
            } else {
                (base / (255.0 - blend)).min(255.0)
            }
        }

        Color::new(
            dodge_channel(self.r, blend.r),
            dodge_channel(self.g, blend.g),
            dodge_channel(self.b, blend.b)
        )
    }

    pub fn blend_color_burn(&self, blend: &Color) -> Color {
        fn burn_channel(base: f32, blend: f32) -> f32 {
            if blend == 0.0 {
                0.0
            } else {
                255.0 - ((255.0 - base) * 255.0 / blend).min(255.0)
            }
        }

        Color::new(
            burn_channel(self.r, blend.r),
            burn_channel(self.g, blend.g),
            burn_channel(self.b, blend.b),
        )
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.min(255.0).max(0.0),
            g: self.g.min(255.0).max(0.0),
            b: self.b.min(255.0).max(0.0),
        }
    }

    pub fn blend_hard_light(&self, blend: &Color) -> Color {
        self.blend_overlay(blend)
    }

    pub fn blend_soft_light(&self, blend: &Color) -> Color {
        fn soft_light_channel(base: f32, blend: f32) -> f32 {
            let b = base / 255.0;
            let s = blend / 255.0;
            if s < 0.5 {
                (b - (1.0 - 2.0 * s) * b * (1.0 - b)) * 255.0
            } else {
                (b + (2.0 * s - 1.0) * (((b - 0.5).abs() * 16.0 + 12.0) * b - 3.0)) * 255.0
            }.round() as f32
        }
        Color::new(
            soft_light_channel(self.r, blend.r),
            soft_light_channel(self.g, blend.g),
            soft_light_channel(self.b, blend.b)
        )
    }

    pub fn blend_difference(&self, blend: &Color) -> Color {
        Color::new(
            (self.r - blend.r).abs().min(1.0),
            (self.g - blend.g).abs().min(1.0),
            (self.b - blend.b).abs().min(1.0)
        )
    }

    pub fn blend_exclusion(&self, blend: &Color) -> Color {
        Color::new(
            (self.r + blend.r - 2.0 * self.r * blend.r).clamp(0.0, 1.0),
            (self.g + blend.g - 2.0 * self.g * blend.g).clamp(0.0, 1.0),
            (self.b + blend.b - 2.0 * self.b * blend.b).clamp(0.0, 1.0)
        )
    }

    pub fn blend_additive(&self, blend: &Color) -> Color {
        Color::new(
            (self.r + blend.r).min(255.0),
            (self.g + blend.g).min(255.0),
            (self.b + blend.b).min(255.0),
        )
    }

}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        Color {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
  }
}