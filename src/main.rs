//! 3D Cube Rotation with Mouse in Rust using minifb
//! Author: M3str3

mod transformations;
mod rendering;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use native_dialog::FileDialog;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    thread,
    time::Duration,
    env,
};

////////////////////////////////////////////////////////////////////////////////
// Window parameters and perspective settings
////////////////////////////////////////////////////////////////////////////////
const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xFFFFFFFF;
const RED: u32 = 0xFF0000;
const GREEN: u32 = 0x00FF00;
const BLUE: u32 = 0x0000FF;

const COLORS: [u32; 5] = [BLACK, WHITE, RED, GREEN, BLUE]; 
const WIDTH: usize = 1000;
const HEIGHT: usize = 800;
const SCALE: f32 = 600.0; // Scaling factor for the 3D model in screen space
const FRAME_DELAY_MS: u64 = 16; // ~60 fps (16 ms per frame)

/// 3D model structure: stores vertices and edges.
struct Model {
    vertices: Vec<(f32, f32, f32)>,
    edges: Vec<(usize, usize)>,
}

/// Loads a 3D model from a Wavefront `.obj` file.
/// ---------------------------------------------------------------------
/// Each line starting with `v` defines a vertex (`v x y z`).
/// Each line starting with `f` defines a face (`f v1 v2 v3 [v4 ...]`).
/// Indices in `.obj` are 1-based, so we shift them to 0-based for Rust.
fn load_obj(file_path: &str) -> Result<Model, String> {
    let file = File::open(file_path)
        .map_err(|e| format!("Could not open file: {}", e))?;
    let reader = BufReader::new(file);

    let mut vertices = Vec::new();
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        // Strip comments
        let line = line.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => {
                // Vertex line: v x y z
                let x: f32 = parts[1].parse().unwrap();
                let y: f32 = parts[2].parse().unwrap();
                let z: f32 = parts[3].parse().unwrap();
                vertices.push((x, y, z));
            }
            "f" => {
                // Face line: f v1 v2 v3 [v4 ...]
                let face_indices: Result<Vec<usize>, _> = parts[1..]
                    .iter()
                    .map(|v_str| v_str.parse::<usize>().map(|idx| idx - 1))
                    .collect();

                let face_indices = match face_indices {
                    Ok(face_indices) => face_indices,
                    Err(_) => {
                        eprintln!("Error parsing face indices in line: {}", line);
                        continue;
                    }
                };

                // Check index range
                if face_indices.iter().any(|&i| i >= vertices.len()) {
                    eprintln!("Index out of range in line: {}", line);
                    continue;
                }

                // Build edges from consecutive vertex indices
                for i in 0..face_indices.len() {
                    let start = face_indices[i];
                    let end = face_indices[(i + 1) % face_indices.len()];
                    if !edges.contains(&(start, end)) && !edges.contains(&(end, start)) {
                        edges.push((start, end));
                    }
                }
            }
            _ => {}
        }
    }

    Ok(Model { vertices, edges })
}

fn main() {
    let mut obj_color: usize = 0;
    let mut bg_color: usize = 1;

    let mut window = match Window::new(
        "M3str3 - Model viewer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ) {
        Ok(win) => win,
        Err(e) => {
            eprintln!("Failed to create window: {}", e);
            return;
        }
    };

    // A buffer of size WIDTH * HEIGHT for drawing
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    // Rotation angles around X, Y, Z
    let mut angle_x = 0.0_f32;
    let mut angle_y = 0.0_f32;
    let angle_z = 0.0_f32;

    let mut distance: f32 = 8.0; // Distance from the camera to the origin
    let mut auto_rotate = true;

    // Default 3D model, a cube with 8 vertices and 12 edges
    let mut model = Model {
        vertices: vec![
            (-1.0, -1.0, -1.0),
            ( 1.0, -1.0, -1.0),
            ( 1.0,  1.0, -1.0),
            (-1.0,  1.0, -1.0),
            (-1.0, -1.0,  1.0),
            ( 1.0, -1.0,  1.0),
            ( 1.0,  1.0,  1.0),
            (-1.0,  1.0,  1.0),
        ],
        edges: vec![
            // Bottom face
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            // Top face
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            // Vertical edges
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ],
    };

    // Load argument at start
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let obj_file_path = &args[1];
        match load_obj(obj_file_path) {
            Ok(loaded_model) => {
                model = loaded_model;
                println!("Model loaded successfully: {:?}", obj_file_path);
            }
            Err(err) => {
                eprintln!("Error loading model: {}", err);
            }
        }
    }

    
    let mut last_mouse_pos: Option<(f32, f32)> = None;

    //////////////////////////////////////////////////////////////////////////////////////////
    // Main loop                                                                            //
    //////////////////////////////////////////////////////////////////////////////////////////
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer to black
        buffer.fill(COLORS[bg_color]);

        //////////////////////////////////////////////////////////////////////////////////////
        // Keyboard controls 
        //////////////////////////////////////////////////////////////////////////////////////
        // Zoom in
        if window.is_key_down(Key::Up) || window.is_key_down(Key::Equal) {
            distance -= 0.1;
            if distance < 0.1 {
                distance = 0.1;
            }
        }
        // Zoom out
        if window.is_key_down(Key::Down) || window.is_key_down(Key::Minus) {
            distance += 0.1;
        }

        // Change background color
        if window.is_key_pressed(Key::B, minifb::KeyRepeat::No) {
            bg_color += 1;
            if bg_color >= COLORS.len() {
                bg_color = 0;
            }
            println!("Background color: {}", COLORS[bg_color]);
        }

        // Change object color
        if window.is_key_pressed(Key::M, minifb::KeyRepeat::No) {
            obj_color += 1;
            if obj_color >= COLORS.len() {
                obj_color = 0;
            }
            println!("Object color: {}", COLORS[obj_color]);
        }

        // Toggle auto-rotation
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            auto_rotate = !auto_rotate;
            println!("Auto-rotation: {}", if auto_rotate { "ENABLED" } else { "DISABLED" });
        }

        // If auto-rotation is enabled, increment angles each frame
        if auto_rotate {
            angle_y += 0.01;
            angle_x += 0.01;
        }

        //////////////////////////////////////////////////////////////////////////////////////
        // Mouse control for manual rotation (left-click)
        //////////////////////////////////////////////////////////////////////////////////////
        if let Some(pos) = window.get_mouse_pos(MouseMode::Discard) {
            if window.get_mouse_down(MouseButton::Left) {
                if let Some((last_x, last_y)) = last_mouse_pos {
                    let dx = pos.0 - last_x;
                    let dy = pos.1 - last_y;
                    // Adjust sensitivity here (e.g. 0.01)
                    angle_y -= dx * 0.01;
                    angle_x -= dy * 0.01;
                }
            }
            last_mouse_pos = Some(pos);
        } else {
            last_mouse_pos = None;
        }

        //////////////////////////////////////////////////////////////////////////////////////
        // Press 'L' to load a new model from file
        //////////////////////////////////////////////////////////////////////////////////////
        if window.is_key_pressed(Key::L, minifb::KeyRepeat::No) {
            println!("Loading model from file...");
            if let Some(path) = FileDialog::new()
                .add_filter("Wavefront OBJ", &["obj"])
                .show_open_single_file()
                .unwrap()
            {
                match load_obj(path.to_str().unwrap()) {
                    Ok(loaded_model) => {
                        model = loaded_model;
                        println!("Model loaded successfully: {:?}", path);
                    }
                    Err(err) => {
                        eprintln!("Error loading model: {}", err);
                    }
                }
            } else {
                println!("No file was selected");
            }
        }

        //////////////////////////////////////////////////////////////////////////////////////
        // Drawing the 3D model
        //////////////////////////////////////////////////////////////////////////////////////
        // We rotate each vertex around X, Y, Z, then project it using a simple perspective:
        //
        // $$ x' = x \cos(\theta_x) + \dots $$
        // $$ u = x' \frac{\text{SCALE}}{z' + \text{distance}} + \frac{\text{WIDTH}}{2} $$
        // $$ v = -y' \frac{\text{SCALE}}{z' + \text{distance}} + \frac{\text{HEIGHT}}{2} $$
        
        for &(i1, i2) in &model.edges {
            let (x1, y1, z1) = model.vertices[i1];
            let (x2, y2, z2) = model.vertices[i2];

            // Rotate each endpoint around X, Y, and Z
            let (rx1, ry1, rz1) = {
                let (tx, ty, tz) = transformations::rotate_x(x1, y1, z1, angle_x);
                let (tx, ty, tz) = transformations::rotate_y(tx, ty, tz, angle_y);
                transformations::rotate_z(tx, ty, tz, angle_z)
            };

            let (rx2, ry2, rz2) = {
                let (tx, ty, tz) = transformations::rotate_x(x2, y2, z2, angle_x);
                let (tx, ty, tz) = transformations::rotate_y(tx, ty, tz, angle_y);
                transformations::rotate_z(tx, ty, tz, angle_z)
            };

            if let (Some(start), Some(end)) = (
                transformations::project_perspective(rx1, ry1, rz1, distance, SCALE, WIDTH, HEIGHT),
                transformations::project_perspective(rx2, ry2, rz2, distance, SCALE, WIDTH, HEIGHT),
            ) {
                rendering::draw_line(&mut buffer, WIDTH, HEIGHT, start, end, COLORS[obj_color]);
            }
        }
        
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        thread::sleep(Duration::from_millis(FRAME_DELAY_MS));
    }
}
