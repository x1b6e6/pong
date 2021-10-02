use {
    crate::blink::Blink,
    cortex_m::peripheral::SYST,
    embedded_hal::spi::MODE_0,
    stm32f4xx_hal::{
        adc::Adc,
        gpio::{Alternate, Floating, Input, NoPin, Output, Pin, PushPull},
        pac::{SPI1, TIM1, TIM4},
        prelude::*,
        qei,
        spi::{Spi, TransferModeNormal},
        timer::Timer,
    },
};

pub use stm32f4xx_hal::{delay::Delay, pac, prelude};

pub struct Device {
    pub spi: Spi<
        SPI1,
        (
            Pin<Input<Floating>, 'A', 5>,
            NoPin,
            Pin<Input<Floating>, 'A', 7>,
        ),
        TransferModeNormal,
    >,
    pub encoder1: qei::Qei<TIM1, (Pin<Alternate<1>, 'A', 8>, Pin<Alternate<1>, 'A', 9>)>,
    pub encoder2: qei::Qei<TIM4, (Pin<Alternate<2>, 'B', 6>, Pin<Alternate<2>, 'B', 7>)>,
    pub rand_seed: u16,
    pub led: Pin<Output<PushPull>, 'C', 13>,
    pub dc: Pin<Output<PushPull>, 'A', 3>,
    pub cs: Pin<Output<PushPull>, 'A', 2>,
    pub syst: Timer<SYST>,
}

impl Device {
    pub fn new() -> Self {
        let dp = pac::Peripherals::take().unwrap();
        let cp = cortex_m::Peripherals::take().unwrap();

        let rcc = dp.RCC.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(25.mhz())
            .sysclk(84.mhz())
            .hclk(84.mhz())
            .pclk2(84.mhz())
            .pclk1(42.mhz())
            .freeze();

        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();
        let gpioc = dp.GPIOC.split();

        let dc = gpioa.pa3.into_push_pull_output();
        let cs = gpioa.pa2.into_push_pull_output();

        let sck = gpioa.pa5;
        let mosi = gpioa.pa7;

        let spi = Spi::new(dp.SPI1, (sck, NoPin, mosi), MODE_0, 28.mhz(), clocks);

        let encoder1 = qei::Qei::new(
            dp.TIM1,
            (gpioa.pa8.into_alternate(), gpioa.pa9.into_alternate()),
        );

        let encoder2 = qei::Qei::new(
            dp.TIM4,
            (gpiob.pb6.into_alternate(), gpiob.pb7.into_alternate()),
        );

        let led = gpioc.pc13.into_push_pull_output();

        let syst = Timer::syst(cp.SYST, &clocks);

        let mut rand_pin = gpioa.pa4.into_analog();
        let mut adc = Adc::adc1(dp.ADC1, false, Default::default());
        let rand_seed: u16 = adc.read(&mut rand_pin).unwrap();

        Self {
            spi,
            encoder1,
            encoder2,
            rand_seed,
            led,
            dc,
            cs,
            syst,
        }
    }
}

pub unsafe fn blink_loop(start: u32) -> ! {
    let dp = pac::Peripherals::steal();
    let cp = cortex_m::Peripherals::steal();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let gpioc = dp.GPIOC.split();

    let led = gpioc.pc13.into_push_pull_output();
    let delay = Delay::new(cp.SYST, &clocks);

    let mut blink = Blink::new(start, led, delay);

    loop {
        blink.next().unwrap();
    }
}
