# Rust3D

A basic 3D viewer written in Rust for rotating, zooming, and loading `.obj` files.

## Features

- Rotate the 3D model.
- Zoom in and out.
- Load `.obj` files dynamically.

## Controls

| **Key/Mouse**         | **Action**               |
|------------------------|-------------------------|
| **Left Click + Drag**  | Rotate the model        |
| **Space**              | Toggle auto-rotation    |
| **Up / +**             | Zoom in                 |
| **Down / -**           | Zoom out                |
| **L**                  | Load a new `.obj` file  |
| **Escape**             | Exit the program        |

## How It Works

### Rotation Matrices
The 3D point \((x, y, z)\) is rotated using standard rotation matrices:

- **X-axis Rotation**:
```math
\begin{cases}
x' = x \
y' = y \cos\theta - z \sin\theta \
z' = y \sin\theta + z \cos\theta 
\end{cases}
```

- **Perspective Projection**:
```math
\begin{cases}
u = x \cdot \frac{\text{scale}}{z + \text{distance}} + \frac{\text{screen\_width}}{2} \
v = -y \cdot \frac{\text{scale}}{z + \text{distance}} + \frac{\text{screen\_height}}{2}
\end{cases}
```
