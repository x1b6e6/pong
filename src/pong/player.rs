#[derive(Clone, Copy)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Player {
    pub(super) fn player1(_width: u32, height: u32) -> Self {
        Self {
            x: 0,
            y: (3 * height / 8) as i32,
            width: 2,
            height: (height / 4) as u32,
        }
    }

    pub(super) fn player2(width: u32, height: u32) -> Self {
        Self {
            x: (width - 2) as i32,
            y: (3 * height / 8) as i32,
            width: 2,
            height: (height / 4) as u32,
        }
    }

    pub(super) fn move_up(&mut self, up: i32, up_limit: u32) {
        self.y -= up;
        if self.y < up_limit as i32 {
            self.y = up_limit as i32;
        }
    }

    pub(super) fn move_down(&mut self, down: i32, down_limit: u32) {
        self.y -= down;
        let player_down_y = self.y + self.height as i32;
        if player_down_y > down_limit as i32 {
            self.y = (down_limit - self.height) as i32;
        }
    }
}
