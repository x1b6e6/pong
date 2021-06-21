#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use nb::block;
use panic_halt as _;
use ssd1306::size::DisplaySize128x64;
use stm32f1xx_hal::{
    adc::Adc,
    delay::Delay,
    prelude::*,
    spi::{Mode as SpiMode, Phase, Polarity, Spi},
    stm32,
    timer::Timer,
};

mod blink;
mod drawer;
mod pong;
mod rnd;

struct PlayerControl<BtnUp, BtnDown>
where
    BtnUp: InputPin,
    BtnDown: InputPin,
{
    btn_up: BtnUp,
    btn_down: BtnDown,
    up_spd: f32,
    down_spd: f32,
}

impl<BtnUp, BtnDown> PlayerControl<BtnUp, BtnDown>
where
    BtnUp: InputPin,
    BtnDown: InputPin,
{
    fn new(btn_up: BtnUp, btn_down: BtnDown) -> Self {
        Self {
            btn_up,
            btn_down,
            up_spd: 0f32,
            down_spd: 0f32,
        }
    }

    fn any(&self) -> bool {
        self.btn_up.is_low().ok().unwrap() || self.btn_down.is_low().ok().unwrap()
    }

    fn delta(&mut self) -> i32 {
        let up = self.btn_up.is_low().ok().unwrap();
        let down = self.btn_down.is_low().ok().unwrap();

        if up {
            self.up_spd += 0.3;
        } else if self.up_spd > 0.0 {
            self.up_spd = 0.0;
        }

        if down {
            self.down_spd += 0.3;
        } else if self.down_spd > 0.0 {
            self.down_spd = 0.0;
        }

        if up && down {
            self.up_spd = 0.0;
            self.down_spd = 0.0;
        }

        (self.up_spd - self.down_spd) as i32
    }
}

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .pclk1(36.mhz())
        .freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let dc = gpioa.pa3.into_push_pull_output(&mut gpioa.crl);
    let cs = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);

    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6.into_floating_input(&mut gpioa.crl);
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let btn_up1 = gpioa.pa0.into_pull_down_input(&mut gpioa.crl);
    let btn_down1 = gpioa.pa1.into_pull_down_input(&mut gpioa.crl);
    let btn_up2 = gpiob.pb0.into_pull_down_input(&mut gpiob.crl);
    let btn_down2 = gpiob.pb1.into_pull_down_input(&mut gpiob.crl);

    let mut rand_pin = gpioa.pa4.into_analog(&mut gpioa.crl);
    let mut adc = Adc::adc1(dp.ADC1, &mut rcc.apb2, clocks);
    let rand_seed: u16 = adc.read(&mut rand_pin).unwrap();

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    led.set_high().unwrap();

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        SpiMode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        36.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut drawer = drawer::Ssd1306PongDrawer::new(spi, dc, cs, DisplaySize128x64);

    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(60.hz());
    let mut rand_generator = rnd::PseudoRandomGenerator::new(rand_seed);
    let mut pong = pong::Pong::new(128, 64, || rand_generator.get() as i32);

    let mut player1 = PlayerControl::new(btn_up1, btn_down1);
    let mut player2 = PlayerControl::new(btn_up2, btn_down2);

    loop {
        block!(timer.wait()).unwrap();

        if let pong::Status::GameOver = pong.status() {
            if player1.any() || player2.any() {
                continue;
            }
            led.set_high().unwrap();
            pong.reinit();
        }

        let delta1 = player1.delta();
        let delta2 = player2.delta();

        let res = pong.next(delta1, delta2);

        match res {
            pong::Result::GameInProgress(progress) => {
                use pong::Drawer;

                drawer.clear();
                drawer.draw_score(progress.score);
                drawer.draw_ball(progress.ball);
                drawer.draw_player(progress.player1);
                drawer.draw_player(progress.player2);
                drawer.flush();
            }
            _ => {
                wait_press(&player1, &player2, &mut timer, &mut led);
            }
        };
    }
}

fn wait_press<BtnUp1, BtnDown1, BtnUp2, BtnDown2, TIMER, LED>(
    player1: &PlayerControl<BtnUp1, BtnDown1>,
    player2: &PlayerControl<BtnUp2, BtnDown2>,
    timer: &mut TIMER,
    led: &mut LED,
) where
    BtnUp1: InputPin,
    BtnDown1: InputPin,
    BtnUp2: InputPin,
    BtnDown2: InputPin,
    TIMER: embedded_hal::timer::CountDown,
    LED: OutputPin,
{
    led.set_low().ok().unwrap();
    while !player1.any() && !player2.any() {
        block!(timer.wait()).unwrap();
    }
}

unsafe fn blink_loop(start: u32) -> ! {
    let dp = stm32::Peripherals::steal();
    let cp = cortex_m::Peripherals::steal();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let delay = Delay::new(cp.SYST, clocks);

    let mut blink = blink::Blink::new(start, led, delay);

    loop {
        blink.next().unwrap();
    }
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    blink_loop(20);
}
