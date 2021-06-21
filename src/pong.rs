pub mod ball;
pub mod player;
pub mod prelude;

pub use prelude::*;

const BALL_MAX_SPEED: f32 = 1.5;

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

        ball.move_next();

        if ball.player_collision(&self.progress.player1)
            || ball.player_collision(&self.progress.player2)
        {
            ball.bounce_off_player(&mut self.random);
        }

        if ball.border_collision(0, self.height) {
            ball.bounce_off_border();
        }

        if ball.x < 0f32 {
            self.progress.score.1 += 1;
            self.status = Status::GameOver;
            return Result::GameOver;
        }

        if ball.x > self.width as f32 {
            self.progress.score.0 += 1;
            self.status = Status::GameOver;
            return Result::GameOver;
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
