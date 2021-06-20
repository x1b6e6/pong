#[derive(Clone, Copy)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub r: u32,
    x_spd: f32,
    y_spd: f32,
}

#[derive(Clone, Copy)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub type Score = (u32, u32);

#[derive(Clone, Copy)]
pub struct PongProgress {
    pub ball: Ball,
    pub player1: Player,
    pub player2: Player,
    pub score: Score,
}

pub trait PongDrawer {
    fn draw_ball(&mut self, ball: Ball);
    fn draw_player(&mut self, player: Player);
    fn draw_score(&mut self, score: Score);
}

#[derive(Clone, Copy)]
pub enum ResultPongStatus {
    GameOver(Score),
    GameInProgress(PongProgress),
    Err,
}

#[derive(Clone, Copy)]
pub enum PongStatus {
    GameOver,
    GameInProgress,
}

pub struct Pong<RND>
where
    RND: FnMut() -> i32,
{
    width: u32,
    height: u32,
    progress: PongProgress,
    status: PongStatus,
    random: RND,
}

impl<RND> Pong<RND>
where
    RND: FnMut() -> i32,
{
    pub fn new(width: u32, height: u32, random: RND) -> Self {
        Self {
            width,
            height,
            status: PongStatus::GameInProgress,
            progress: PongProgress {
                ball: Ball::new(width, height),
                player1: Player::player1(width, height),
                player2: Player::player2(width, height),
                score: (0, 0),
            },
            random,
        }
    }

    pub fn reinit(&mut self) {
        self.progress = PongProgress {
            ball: Ball::new(self.width, self.height),
            player1: Player::player1(self.width, self.height),
            player2: Player::player2(self.width, self.height),
            score: self.progress.score,
        };
        self.status = PongStatus::GameInProgress;
    }

    fn move_ball(&mut self) -> ResultPongStatus {
        let ball = &mut self.progress.ball;

        ball.x += ball.x_spd;
        ball.y += ball.y_spd;

        let player1 = &self.progress.player1;
        ball.check_player1_collision(player1, &mut self.random);
        let player2 = &self.progress.player2;
        ball.check_player2_collision(player2, &mut self.random);

        if ball.x < 0f32 {
            self.progress.score.1 += 1;
            self.status = PongStatus::GameOver;
            return ResultPongStatus::GameOver(self.progress.score);
        }

        if ball.x > self.width as f32 {
            self.progress.score.0 += 1;
            self.status = PongStatus::GameOver;
            return ResultPongStatus::GameOver(self.progress.score);
        }

        if ball.y - ball.r as f32 <= 0.0 {
            ball.y_spd = -ball.y_spd;
        } else if ball.y + ball.r as f32 > self.height as f32 {
            ball.y_spd = -ball.y_spd;
        }
        ResultPongStatus::GameInProgress(self.progress)
    }

    fn player1_move(&mut self, delta: i32) {
        if delta > 0 {
            self.progress.player1.move_up(delta, 0);
        } else if delta < 0 {
            self.progress.player1.move_down(delta, self.height)
        }
    }

    fn player2_move(&mut self, delta: i32) {
        if delta > 0 {
            self.progress.player2.move_up(delta, 0);
        } else if delta < 0 {
            self.progress.player2.move_down(delta, self.height);
        }
    }

    pub fn next(&mut self, delta1: i32, delta2: i32) -> ResultPongStatus {
        if let PongStatus::GameInProgress = self.status {
            self.player1_move(delta1);
            self.player2_move(delta2);
            self.move_ball()
        } else {
            ResultPongStatus::Err
        }
    }

    pub fn status(&self) -> PongStatus {
        self.status
    }
}

impl Ball {
    fn new(_width: u32, height: u32) -> Self {
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

    fn check_player1_collision<RND>(&mut self, player: &Player, random: &mut RND) -> bool
    where
        RND: FnMut() -> i32,
    {
        if self.x_spd > 0.0 {
            return false;
        }

        if (self.x - self.r as f32) > (player.x + player.width as i32) as f32 {
            return false;
        }

        if (self.y - self.r as f32) > (player.y + player.height as i32) as f32 {
            return false;
        }

        if (self.y + self.r as f32) < player.y as f32 {
            return false;
        }

        self.x_spd = -self.x_spd;
        self.rand_add_y_spd(random);
        true
    }

    fn check_player2_collision<RND>(&mut self, player: &Player, random: &mut RND) -> bool
    where
        RND: FnMut() -> i32,
    {
        if self.x_spd < 0.0 {
            return false;
        }

        if (self.x + self.r as f32) < player.x as f32 {
            return false;
        }

        if self.y > (player.y + player.height as i32) as f32 {
            return false;
        }

        if self.y < player.y as f32 {
            return false;
        }

        self.x_spd = -self.x_spd;
        self.rand_add_y_spd(random);
        true
    }
}
impl Player {
    fn player1(_width: u32, height: u32) -> Self {
        Self {
            x: 0,
            y: (3 * height / 8) as i32,
            width: 2,
            height: (height / 4) as u32,
        }
    }
    fn player2(width: u32, height: u32) -> Self {
        Self {
            x: (width - 2) as i32,
            y: (3 * height / 8) as i32,
            width: 2,
            height: (height / 4) as u32,
        }
    }
    fn move_up(&mut self, up: i32, up_limit: u32) {
        self.y -= up;
        if self.y < up_limit as i32 {
            self.y = up_limit as i32;
        }
    }
    fn move_down(&mut self, down: i32, down_limit: u32) {
        self.y -= down;
        let player_down_y = self.y + self.height as i32;
        if player_down_y > down_limit as i32 {
            self.y = (down_limit - self.height) as i32;
        }
    }
}
