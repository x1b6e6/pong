use {
    embedded_hal::{timer::CountDown, Qei},
    nb::block,
};

pub trait PlayerControl {
    fn any(&mut self) -> bool;
    fn delta(&mut self) -> i32;
}

pub struct PlayerEncoder<Counter>
where
    Counter: Qei<Count = u16>,
{
    prev: u16,
    encoder: Counter,
}

impl<Counter> PlayerEncoder<Counter>
where
    Counter: Qei<Count = u16>,
{
    pub fn new(encoder: Counter) -> Self {
        let prev = encoder.count();
        Self { encoder, prev }
    }
}

impl<Counter> PlayerControl for PlayerEncoder<Counter>
where
    Counter: Qei<Count = u16>,
{
    fn delta(&mut self) -> i32 {
        let cnt = self.encoder.count();
        let out = cnt.wrapping_sub(self.prev) as i16;
        self.prev = cnt;

        out as i32
    }

    fn any(&mut self) -> bool {
        self.delta().abs() > 2
    }
}

pub fn wait_press<Player1, Player2, Timer>(
    player1: &mut Player1,
    player2: &mut Player2,
    timer: &mut Timer,
) where
    Player1: PlayerControl,
    Player2: PlayerControl,
    Timer: CountDown,
{
    while !player1.any() && !player2.any() {
        block!(timer.wait()).unwrap();
    }
    while player1.any() || player2.any() {
        block!(timer.wait()).unwrap();
    }
}
