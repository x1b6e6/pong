use {
    crate::blink::Blink,
    cortex_m::peripheral::SYST,
    embedded_hal::spi::MODE_0,
    stm32f1xx_hal::{
        adc::Adc,
        gpio::{
            gpioa::{PA0, PA1, PA2, PA3, PA5, PA7, PA8, PA9},
            gpioc::PC13,
            Alternate, Floating, Input, Output, PushPull,
        },
        pac::{SPI1, TIM1, TIM2},
        prelude::*,
        qei,
        spi::{NoMiso, Spi, Spi1NoRemap},
        timer::{Tim1NoRemap, Tim2NoRemap, Timer},
    },
};

pub use stm32f1xx_hal::{delay::Delay, pac, prelude};

pub struct Device {
    pub spi:
        Spi<SPI1, Spi1NoRemap, (PA5<Alternate<PushPull>>, NoMiso, PA7<Alternate<PushPull>>), u8>,
    pub encoder1: qei::Qei<TIM1, Tim1NoRemap, (PA8<Input<Floating>>, PA9<Input<Floating>>)>,
    pub encoder2: qei::Qei<TIM2, Tim2NoRemap, (PA0<Input<Floating>>, PA1<Input<Floating>>)>,
    pub rand_seed: u16,
    pub led: PC13<Output<PushPull>>,
    pub dc: PA3<Output<PushPull>>,
    pub cs: PA2<Output<PushPull>>,
    pub syst: Timer<SYST>,
}

impl Device {
    pub fn new() -> Self {
        let dp = pac::Peripherals::take().unwrap();
        let cp = cortex_m::Peripherals::take().unwrap();

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
        let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .pclk2(72.mhz())
            .freeze(&mut flash.acr);

        let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
        let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

        let dc = gpioa.pa3.into_push_pull_output(&mut gpioa.crl);
        let cs = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);

        let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
        let miso = NoMiso;
        let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

        let spi = Spi::spi1(
            dp.SPI1,
            (sck, miso, mosi),
            &mut afio.mapr,
            MODE_0,
            36.mhz(),
            clocks,
            &mut rcc.apb2,
        );

        let mut rand_pin = gpioa.pa4.into_analog(&mut gpioa.crl);
        let mut adc = Adc::adc1(dp.ADC1, &mut rcc.apb2, clocks);
        let rand_seed: u16 = adc.read(&mut rand_pin).unwrap();

        let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        let encoder1 = Timer::tim1(dp.TIM1, &clocks, &mut rcc.apb2).qei::<Tim1NoRemap, _>(
            (gpioa.pa8, gpioa.pa9),
            &mut afio.mapr,
            Default::default(),
        );

        let encoder2 = Timer::tim2(dp.TIM2, &clocks, &mut rcc.apb1).qei::<Tim2NoRemap, _>(
            (gpioa.pa0, gpioa.pa1),
            &mut afio.mapr,
            Default::default(),
        );
        let syst = Timer::syst(cp.SYST, &clocks);

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

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let delay = Delay::new(cp.SYST, clocks);

    let mut blink = Blink::new(start, led, delay);

    loop {
        blink.next().unwrap();
    }
}
