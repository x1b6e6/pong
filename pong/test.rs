macro_rules! add_test {
    ($($(#[$($mod_macro_args:tt)*])*$name:ident{$($(#[$($fn_macro_args:tt)*])*$subname:ident$(::<$($ty:ty),+$(,)?>)?($($args:expr),*$(,)?)),*$(,)?}),*$(,)?) => {
        $(
            $(
                #[$($mod_macro_args)*]
            )*
            mod $name {
                use super::$name;
                $(
                    $(
                        #[$($fn_macro_args)*]
                    )*
                    #[test]
                    pub fn $subname () {
                        $name$(::<$($ty,)+>)?($($args,)*);
                    }
                )*
            }
        )*
    };
}

use crate::{Ball, Player};

fn new_ball(x: i32, y: i32) -> Ball {
    let mut ball = Ball::with_x_spd(64, 64, 0.0);

    ball.x = x as f32;
    ball.y = y as f32;

    dbg!(ball)
}

fn new_player(x: i32, y: i32) -> Player {
    let mut player = Player::player1(64, 64);

    player.x = x;
    player.y = y;

    dbg!(player)
}

fn player_collision(bx: i32, by: i32, px: i32, py: i32, expect: bool) {
    let ball = new_ball(bx, by);
    let player = new_player(px, py);

    assert_eq!(ball.player_collision(&player), expect);
}

fn border_collision(bx: i32, by: i32, top_border: u32, bottom_border: u32, expect: bool) {
    let mut ball1 = new_ball(bx, by);
    ball1.add_rand_y_spd(&mut || 5);
    let ball1 = ball1;

    let mut ball2 = ball1;
    ball2.bounce_off_border();
    let ball2 = ball2;

    assert_eq!(
        ball1.border_collision(top_border, bottom_border)
            || ball2.border_collision(top_border, bottom_border),
        expect
    );
}

add_test! {
    player_collision {
        same(0, 0, 0, 0, true),

        far_top(0, 0, 0, 50, false),
        far_bottom(0, 50, 0, 0, false),
        far_left(0, 0, 50, 0, false),
        far_right(50, 0, 0 ,0, false),

        closed_top(0, 0, 0, 4, false),
        closed_bottom(0, 20, 0, 0, false),
        closed_left(0, 0, 4, 0, false),
        closed_right(6, 0, 0, 0, false),

        collision_top(0, 0, 0, 3, true),
        collision_bottom(0, 19, 0, 0, true),
        collision_left(0, 0, 3, 0, true),
        collision_right(5, 0, 0, 0, true),
    },
    border_collision {
        far(32, 32, 0, 64, false),

        closed_top(0, 4, 0, 64, false),
        closed_bottom(0, 60, 0, 64, false),

        collision_top(0, 3, 0, 64, true),
        collision_bottom(0, 61, 0, 64, true),
    },
}
