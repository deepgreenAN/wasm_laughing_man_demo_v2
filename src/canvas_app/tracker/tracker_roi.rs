use rustface::Rectangle;

#[derive(Clone, Copy)]
pub enum RoiState {
    DETECTED,
    NOTDETECTED,
}

#[derive(Clone)]
pub struct TrackerRoi {
    pub id: u32,
    pub tl_x: f64,
    pub tl_y: f64,
    pub width: f64,
    pub height: f64,
    pub state: RoiState,
    pub not_detected_count: u32,
}

impl TrackerRoi {
    pub fn new(id: u32, tl_x: f64, tl_y: f64, width: f64, height: f64) -> Self {
        Self {
            id,
            tl_x,
            tl_y,
            width,
            height,
            state: RoiState::DETECTED,
            not_detected_count: 0,
        }
    }

    pub fn center_x(&self) -> f64 {
        self.tl_x + self.width / 2.0
    }
    pub fn center_y(&self) -> f64 {
        self.tl_y + self.height / 2.0
    }
    pub fn distance_with_rect(&self, rect: &Rectangle) -> f64 {
        let rect_center_x = (rect.x() + rect.width() as i32 / 2) as f64;
        let rect_center_y = (rect.y() + rect.height() as i32 / 2) as f64;
        (self.center_x() - rect_center_x).powi(2) + (self.center_y() - rect_center_y).powi(2)
    }
    pub fn detected(&mut self) {
        match self.state {
            RoiState::DETECTED => {}
            RoiState::NOTDETECTED => {
                self.state = RoiState::DETECTED;
                self.not_detected_count = 0;
            }
        }
    }
    pub fn not_detected(&mut self) {
        match self.state {
            RoiState::DETECTED => {
                self.state = RoiState::NOTDETECTED;
            }
            RoiState::NOTDETECTED => {
                self.not_detected_count += 1;
            }
        }
    }
}
