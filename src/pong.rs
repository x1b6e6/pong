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
    pub fn new(width: u32, height: u32, mut random: RND) -> Self {
        Self {
            width,
            height,
            status: Status::GameInProgress,
            progress: Progress {
                ball: Ball::with_rand_x_spd(width, height, &mut random),
                player1: Player::player1(width, height),
                player2: Player::player2(width, height),
            },
            random,
        }
    }

    pub fn reinit(&mut self) {
        let ball = match self.status {
            Status::GameOver(last_goal_from) => match last_goal_from {
                LastGoalFrom::Player1 => Ball::with_x_spd(self.width, self.height, -BALL_MAX_SPEED),
                LastGoalFrom::Player2 => Ball::with_x_spd(self.width, self.height, BALL_MAX_SPEED),
            },
            _ => Ball::with_rand_x_spd(self.width, self.height, &mut self.random),
        };

        self.progress = Progress {
            ball,
            player1: Player::player1(self.width, self.height),
            player2: Player::player2(self.width, self.height),
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
            return self.game_over(LastGoalFrom::Player2);
        }

        if ball.x > self.width as f32 {
            return self.game_over(LastGoalFrom::Player1);
        }

        Result::GameInProgress(self.progress)
    }

    fn game_over(&mut self, last_goal_from: LastGoalFrom) -> Result {
        self.status = Status::GameOver(last_goal_from);
        Result::GameOver(last_goal_from)
    }

    fn move_player1(&mut self, delta: i32) {
        if delta > 0 {
            self.progress.player1.move_up(delta, 0);
        } else if delta < 0 {
            self.progress.player1.move_down(delta, self.height)
        }
    }

    fn move_player2(&mut self, delta: i32) {
        if delta > 0 {
            self.progress.player2.move_up(delta, 0);
        } else if delta < 0 {
            self.progress.player2.move_down(delta, self.height);
        }
    }

    pub fn next(&mut self, delta1: i32, delta2: i32) -> Result {
        if let Status::GameInProgress = self.status {
            self.move_player1(delta1);
            self.move_player2(delta2);
            self.move_ball()
        } else {
            Result::Err
        }
    }

    pub fn status(&self) -> Status {
        self.status
    }
}
