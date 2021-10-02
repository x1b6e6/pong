#![no_std]
#![no_main]

#[cfg(any(
    not(any(feature = "stm32f103", feature = "stm32f401")),
    all(feature = "st32f103", feature = "stm32f401"),
))]
compile_error!("select only one of platforms: stm32f103, stm32f401");

use {
    control::{PlayerControl, PlayerEncoder},
    cortex_m_rt::{entry, exception, ExceptionFrame},
    embedded_hal::{digital::v2::OutputPin, timer::CountDown},
    nb::block,
    panic_halt as _,
    ssd1306::{prelude::SPIInterface, size::DisplaySize128x64},
    stm32::{blink_loop, prelude::*},
};

mod blink;
mod control;
mod drawer;
mod rnd;
mod stm32;

#[entry]
fn main() -> ! {
    let stm32::Device {
        spi,
        mut led,
        dc,
        cs,
        rand_seed,
        encoder1,
        encoder2,
        syst,
    } = stm32::Device::new();

    OutputPin::set_high(&mut led).unwrap();

    let interface = SPIInterface::new(spi, dc, cs);

    let mut drawer = drawer::Ssd1306PongDrawer::new(interface, DisplaySize128x64);

    let mut timer = syst.start_count_down(60.hz());
    let mut rand_generator = rnd::PseudoRandomGenerator::new(rand_seed);
    let mut pong = pong::Pong::new(128, 64, || rand_generator.get() as i32);

    let mut player1 = PlayerEncoder::new(encoder1);
    let mut player2 = PlayerEncoder::new(encoder2);

    let mut score = (0, 0);

    loop {
        block!(timer.wait()).unwrap();

        let delta1 = player1.delta();
        let delta2 = player2.delta();

        let res = pong.next(delta1, delta2);

        match res {
            pong::Result::GameInProgress(progress) => {
                use pong::Drawer;

                drawer.clear();
                drawer.draw_score(&score);
                drawer.draw_ball(&progress.ball);
                drawer.draw_player(&progress.player1);
                drawer.draw_player(&progress.player2);
                drawer.flush();
            }
            pong::Result::GameOver(last_goal_from) => {
                match last_goal_from {
                    pong::LastGoalFrom::Player1 => score.0 += 1,
                    pong::LastGoalFrom::Player2 => score.1 += 1,
                };
                wait_press(&mut player1, &mut player2, &mut timer, &mut led);
                pong.reinit();
            }
            _ => {
                wait_press(&mut player1, &mut player2, &mut timer, &mut led);
                pong.reinit();
            }
        };
    }
}

fn wait_press<Player1, Player2, Timer, Led>(
    player1: &mut Player1,
    player2: &mut Player2,
    timer: &mut Timer,
    led: &mut Led,
) where
    Player1: PlayerControl,
    Player2: PlayerControl,
    Timer: CountDown,
    Led: OutputPin,
    Led::Error: core::fmt::Debug,
{
    led.set_low().unwrap();
    control::wait_press(player1, player2, timer);
    led.set_high().unwrap();
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    blink_loop(20);
}
