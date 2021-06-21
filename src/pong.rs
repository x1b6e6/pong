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
pub struct Progress {
    pub ball: Ball,
    pub player1: Player,
    pub player2: Player,
    pub score: Score,
}

pub trait Drawer {
    fn draw_ball(&mut self, ball: Ball);
    fn draw_player(&mut self, player: Player);
    fn draw_score(&mut self, score: Score);
}

#[derive(Clone, Copy)]
pub enum Result {
    GameOver(Score),
    GameInProgress(Progress),
    Err,
}

#[derive(Clone, Copy)]
pub enum Status {
    GameOver,
    GameInProgress,
}

pub struct Pong<RND>
where
    RND: FnMut() -> i32,
{
    width: u32,
    height: u32,
    progress: Progress,
    status: Status,
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
            status: Status::GameInProgress,
            progress: Progress {
                ball: Ball::new(width, height),
                player1: Player::player1(width, height),
                player2: Player::player2(width, height),
                score: (0, 0),
            },
            random,
        }
    }

    pub fn reinit(&mut self) {
        self.progress = Progress {
            ball: Ball::new(self.width, self.height),
            player1: Player::player1(self.width, self.height),
            player2: Player::player2(self.width, self.height),
            score: self.progress.score,
        };
        self.status = Status::GameInProgress;
    }

    fn move_ball(&mut self) -> Result {
        let ball = &mut self.progress.ball;

        ball.x += ball.x_spd;
        ball.y += ball.y_spd;

        if ball.player_collision(&self.progress.player1)
            || ball.player_collision(&self.progress.player2)
        {
            ball.bounce_off_player(&mut self.random);
        }

        if ball.x < 0f32 {
            self.progress.score.1 += 1;
            self.status = Status::GameOver;
            return Result::GameOver(self.progress.score);
        }

        if ball.x > self.width as f32 {
            self.progress.score.0 += 1;
            self.status = Status::GameOver;
            return Result::GameOver(self.progress.score);
        }

        if ball.y - ball.r as f32 <= 0.0 {
            ball.y_spd = -ball.y_spd;
        } else if ball.y + ball.r as f32 > self.height as f32 {
            ball.y_spd = -ball.y_spd;
        }
        Result::GameInProgress(self.progress)
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

    pub fn next(&mut self, delta1: i32, delta2: i32) -> Result {
        if let Status::GameInProgress = self.status {
            self.player1_move(delta1);
            self.player2_move(delta2);
            self.move_ball()
        } else {
            Result::Err
        }
    }

    pub fn status(&self) -> Status {
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

    fn player_collision(&self, player: &Player) -> bool {
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

    fn bounce_off_player<RND>(&mut self, random: &mut RND)
    where
        RND: FnMut() -> i32,
    {
        self.x_spd = -self.x_spd;
        self.rand_add_y_spd(random);
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
