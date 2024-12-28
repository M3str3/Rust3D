pub fn rotate_x(x: f32, y: f32, z: f32, angle: f32) -> (f32, f32, f32) {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let y_new = y * cos_a - z * sin_a;
    let z_new = y * sin_a + z * cos_a;
    (x, y_new, z_new)
}

pub fn rotate_y(x: f32, y: f32, z: f32, angle: f32) -> (f32, f32, f32) {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let x_new = x * cos_a + z * sin_a;
    let z_new = -x * sin_a + z * cos_a;
    (x_new, y, z_new)
}

pub fn rotate_z(x: f32, y: f32, z: f32, angle: f32) -> (f32, f32, f32) {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let x_new = x * cos_a - y * sin_a;
    let y_new = x * sin_a + y * cos_a;
    (x_new, y_new, z)
}

/// Projects a 3D point \((x, y, z)\) onto a 2D plane using perspective projection.
///
/// Formula for projection:
/// $$ u = x \cdot \frac{\text{scale}}{z + \text{distance}} + \frac{\text{screen\_width}}{2} $$
/// $$ v = -y \cdot \frac{\text{scale}}{z + \text{distance}} + \frac{\text{screen\_height}}{2} $$
pub fn project_perspective(
    x: f32,
    y: f32,
    z: f32,
    distance: f32,
    scale: f32,
    screen_width: usize,
    screen_height: usize,
) -> Option<(usize, usize)> {
    let z_cam = z + distance;

    if z_cam <= 0.0 {
        return None;
    }

    let factor = scale / z_cam;
    let u = x * factor + (screen_width as f32) / 2.0;
    let v = -y * factor + (screen_height as f32) / 2.0;

    if u >= 0.0 && u < screen_width as f32 && v >= 0.0 && v < screen_height as f32 {
        Some((u as usize, v as usize))
    } else {
        None
    }
}
