/// Draws a line onto the `buffer` using the Bresenham algorithm in 2D.
/// The color is specified as a 32-bit ARGB value (`u32`).
///
/// Bresenham's algorithm avoids floating-point operations by using an "error term" to
/// determine when to step in `x` or `y`. The updates are:
/// - If \( e_2 \geq \Delta y \): move horizontally.
/// - If \( e_2 \leq \Delta x \): move vertically.
///
/// Parameters:
/// - `buffer`: Target pixel buffer (one-dimensional array of `u32` colors).
/// - `width`, `height`: Dimensions of the 2D drawing surface.
/// - `(x0, y0)`, `(x1, y1)`: Start and end coordinates.
/// - `color`: The 32-bit ARGB color to use when drawing the line.
pub fn draw_line(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    (x0, y0): (usize, usize),
    (x1, y1): (usize, usize),
    color: u32,
) {
    let (x1, y1) = (x1 as i32, y1 as i32);
    let (mut x0, mut y0) = (x0 as i32, y0 as i32);

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx + dy;

    loop {
        if x0 >= 0 && x0 < width as i32 && y0 >= 0 && y0 < height as i32 {
            let index = (y0 as usize) * width + (x0 as usize);
            buffer[index] = color;
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
