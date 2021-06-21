use super::Player;

#[derive(Clone, Copy)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub r: u32,
    x_spd: f32,
    y_spd: f32,
}

impl Ball {
    pub(super) fn new(_width: u32, height: u32) -> Self {
        Self {
            x: 5.0,
            y: height as f32 / 2.0,
            r: 3, // TODO: calculate from width and height
            x_spd: 1.5,
            y_spd: 0.0,
        }
    }

    fn rand_add_y_spd<RND>(&mut self, rand: &mut RND)
    where
        RND: FnMut() -> i32,
    {
        let mut rnd = rand();
        rnd %= 13;
        rnd -= 6;
        let mut rnd = rnd as f32;
        rnd /= 5.0;

        self.y_spd += rnd;
        self.limit_speed();
    }

    fn limit_speed(&mut self) {
        let limit = 1.5;

        if self.y_spd > limit {
            self.y_spd -= self.y_spd - limit;
        } else if self.y_spd < -limit {
            self.y_spd -= self.y_spd + limit;
        }
    }

    pub(super) fn player_collision(&self, player: &Player) -> bool {
        {
            let ball_top = self.y - self.r as f32;
            let ball_bottom = self.y + self.r as f32;
            let player_top = player.y as f32;
            let player_bottom = (player.y + player.height as i32) as f32;

            if ball_top > player_bottom {
                return false;
            }

            if ball_bottom < player_top {
                return false;
            }
        }
        {
            let ball_left = self.x - self.r as f32;
            let ball_right = self.x + self.r as f32;
            let player_left = player.x as f32;
            let player_right = (player.x + player.width as i32) as f32;

            if ball_left > player_right {
                return false;
            }

            if ball_right < player_left {
                return false;
            }
        }
        true
    }

    pub(super) fn border_collision(&self, top_border: u32, bottom_border: u32) -> bool {
        let ball_top = self.y - self.r as f32;
        let ball_bottom = self.y + self.r as f32;
        ball_top < top_border as f32 || ball_bottom > bottom_border as f32
    }

    pub(super) fn bounce_off_player<RND>(&mut self, random: &mut RND)
    where
        RND: FnMut() -> i32,
    {
        self.x_spd = -self.x_spd;
        self.rand_add_y_spd(random);
    }

    pub(super) fn bounce_off_border(&mut self) {
        self.y_spd = -self.y_spd;
    }

    pub(super) fn move_next(&mut self) {
        self.x += self.x_spd;
        self.y += self.y_spd;
    }
}
