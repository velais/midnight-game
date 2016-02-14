
pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub zoom: f64,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            x: 0.0,
            y: 0.0,
            dx: 0.0,
            dy: 0.0,
            zoom: 1.0,
        }
    }

    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
} 
