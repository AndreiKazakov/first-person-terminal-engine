pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

impl Player {
    pub fn turn_left(&mut self) {
        self.angle -= 0.1
    }
    
    pub fn turn_right(&mut self) {
        self.angle += 0.1
    }
    
    pub fn move_forward(&mut self) {
        self.x += self.angle.sin();
        self.y += self.angle.cos();
    }
    
    pub fn move_backwards(&mut self) {
        self.x -= self.angle.sin();
        self.y -= self.angle.cos();
    }
}
