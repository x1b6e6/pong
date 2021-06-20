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

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::prelude::*;
use stm32f1xx_hal::delay::Delay;

pub struct Blink<OP: OutputPin> {
    timer: BlinkTimer,
    led: OP,
    delay: Delay,
}

impl<OP: OutputPin> Blink<OP> {
    pub fn new(start: u32, led: OP, delay: Delay) -> Self {
        Self {
            timer: BlinkTimer::new(start),
            led,
            delay,
        }
    }

    pub fn next(&mut self) -> Result<(), OP::Error> {
        self.led.set_high()?;
        self.delay.delay_ms(self.timer.up());
        self.led.set_low()?;
        self.delay.delay_ms(self.timer.down());

        self.timer.next();

        Ok(())
    }
}
