use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;
mod celestial_body;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shaders::{vertex_shader, fragment_shader_neptune, fragment_shader_jupiter, fragment_shader_venus, fragment_shader_mars, fragment_shader_earth, fragment_shader_mercury, fragment_shader_sun};
use camera::Camera;
use celestial_body::CelestialBody;

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    sun_position: Vec3,
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, cos_x, -sin_x, 0.0,
        0.0, sin_x, cos_x, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y, 0.0, sin_y, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z, cos_z, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0, 0.0, translation.x,
        0.0, scale, 0.0, translation.y,
        0.0, 0.0, scale, translation.z,
        0.0, 0.0, 0.0, 1.0,
    );
    
    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}


fn main() {
    let window_width = 600;
    let window_height = 600;
    let framebuffer_width = 600;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust 3D model",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(0,0);
    window.update();

    framebuffer.set_background_color(0x333355);


    let planets = vec![
        CelestialBody {
            name: String::from("Earth"),
            vertex_array: Obj::load("./assets/models/sphere.obj").unwrap().get_vertex_array(),
            shader_type: 6,
            orbit_radius: 4.0,
            rotation_speed: 0.05,
            orbit_speed: 0.015,
            scale: 1.0,
        },
        CelestialBody {
            name: String::from("Mars"),
            vertex_array: Obj::load("./assets/models/sphere.obj").unwrap().get_vertex_array(),
            shader_type: 5,
            orbit_radius: 5.0,
            rotation_speed: 0.045,
            orbit_speed: 0.0095,
            scale: 0.8,
        },
        CelestialBody {
            name: String::from("Jupiter"),
            vertex_array: Obj::load("./assets/models/sphere.obj").unwrap().get_vertex_array(),
            shader_type: 1,
            orbit_radius: 7.0,
            rotation_speed: 0.07,
            orbit_speed: 0.0085,
            scale: 1.5,
        },
        CelestialBody {
            name: String::from("Venus"),
            vertex_array: Obj::load("./assets/models/sphere.obj").unwrap().get_vertex_array(),
            shader_type: 4,
            orbit_radius: 3.0,
            rotation_speed: 0.01,
            orbit_speed: 0.03,
            scale: 0.9,
        },
        CelestialBody {
            name: String::from("Mercury"),
            vertex_array: Obj::load("./assets/models/sphere.obj").unwrap().get_vertex_array(),
            shader_type: 7,
            orbit_radius: 2.0,
            rotation_speed: 0.02,
            orbit_speed: 0.05,
            scale: 0.5,
        },
    ];

    let sun = CelestialBody {
        name: String::from("Sun"),
        vertex_array: Obj::load("./assets/models/sphere.obj").unwrap().get_vertex_array(),
        shader_type: 8,
        orbit_radius: 0.0,  // Doesn't orbit
        rotation_speed: 0.01,
        orbit_speed: 0.0,
        scale: 2.5,
    };


    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 20.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );


    let mut time = 0;
    let mut shader_type = 0;

    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    let sun_world_position = Vec3::new(0.0, 0.0, 0.0);
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        
        
        time += 1;

        framebuffer.clear();

        handle_input(&window, &mut camera, &planets);
        let delta_time = frame_delay.as_secs_f32();
        camera.update(delta_time);

        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        for planet in &planets {
            let model_matrix = planet.get_model_matrix(time as f32);
            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                sun_position: sun_world_position,
            };
            planet.render(&mut framebuffer, &uniforms);
        }

        // Render the Sun separately (it doesn't orbit).
        let sun_model_matrix = sun.get_model_matrix(time as f32);
        let sun_uniforms = Uniforms {
            model_matrix: sun_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            sun_position: sun_world_position,
        };
        sun.render(&mut framebuffer, &sun_uniforms);
       
        framebuffer.set_current_color(0xFFDDDD);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn handle_input(window: &Window, camera: &mut Camera, planets: &[CelestialBody]){
    let movement_speed= 1.0;
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;

    if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
        // Earth
        let planet = &planets[0];
        camera.start_warp(
            planet.get_world_position() + Vec3::new(0.0, 2.0, 10.0), // Offset for viewing
            planet.get_world_position(),
            2.0, // 2-second animation
        );
    }

    if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) {
        // Marte
        let planet = &planets[1];
        camera.start_warp(
            planet.get_world_position() + Vec3::new(0.0, 2.0, 10.0), // Offset for viewing
            planet.get_world_position(),
            2.0, // 2-second animation
        );
    }

    if window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) {
        // Jupiter
        let planet = &planets[2];
        camera.start_warp(
            planet.get_world_position() + Vec3::new(0.0, 2.0, 10.0), // Offset for viewing
            planet.get_world_position(),
            2.0, // 2-second animation
        );
    }

    if window.is_key_pressed(Key::Key4, minifb::KeyRepeat::No) {
        // Venus
        let planet = &planets[3];
        camera.start_warp(
            planet.get_world_position() + Vec3::new(0.0, 2.0, 10.0), // Offset for viewing
            planet.get_world_position(),
            2.0, // 2-second animation
        );
    }

    if window.is_key_pressed(Key::Key5, minifb::KeyRepeat::No) {
        // Mercurio
        let planet = &planets[4];
        camera.start_warp(
            planet.get_world_position() + Vec3::new(0.0, 2.0, 10.0), // Offset for viewing
            planet.get_world_position(),
            2.0, // 2-second animation
        );
    }

    if window.is_key_down(Key::Left){
        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right){
        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W){
        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S){
        camera.orbit(0.0, rotation_speed);
    }

    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A){
        movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D){
        movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q){
        movement.y += movement_speed;
    }
    if window.is_key_down(Key::E){
        movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
        camera.move_center(movement);
    }

    if window.is_key_down(Key::Up){
        camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down){
        camera.zoom(-zoom_speed);
    }

    if window.is_key_pressed(Key::B, minifb::KeyRepeat::No) {
        camera.switch_to_birds_eye();
    }
    if window.is_key_pressed(Key::N, minifb::KeyRepeat::No) {
        camera.switch_to_normal();
    }
}