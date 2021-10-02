pub struct BlinkTimer {
    up: u32,
    down: u32,
    is_up: bool,
}

impl BlinkTimer {
    pub fn new(start: u32) -> Self {
        Self {
            up: 0,
            down: start,
            is_up: false,
        }
    }

    pub fn up(&self) -> u32 {
        self.up
    }

    pub fn down(&self) -> u32 {
        self.down
    }

    pub fn next(&mut self) {
        if self.up == 0 {
            self.is_up = true
        } else if self.down == 0 {
            self.is_up = false
        }
        if self.is_up {
            self.up += 1;
            self.down -= 1;
        } else {
            self.up -= 1;
            self.down += 1;
        }
    }
}

use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

pub struct Blink<LED, DELAY>
where
    LED: OutputPin,
    DELAY: DelayMs<u32>,
{
    timer: BlinkTimer,
    led: LED,
    delay: DELAY,
}

impl<LED, DELAY> Blink<LED, DELAY>
where
    LED: OutputPin,
    DELAY: DelayMs<u32>,
{
    pub fn new(start: u32, led: LED, delay: DELAY) -> Self {
        Self {
            timer: BlinkTimer::new(start),
            led,
            delay,
        }
    }

    pub fn next(&mut self) -> Result<(), LED::Error> {
        self.led.set_high()?;
        self.delay.delay_ms(self.timer.up());
        self.led.set_low()?;
        self.delay.delay_ms(self.timer.down());

        self.timer.next();

        Ok(())
    }
}
