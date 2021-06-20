use crate::pong;
use embedded_graphics::{
    mono_font::{ascii::FONT_8X13, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle},
    text::Text,
};
use embedded_hal::{blocking::spi, digital::v2::OutputPin};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};

pub struct Ssd1306PongDrawer<SPI, DC, CS, SIZE>
where
    SPI: spi::Write<u8>,
    DC: OutputPin,
    CS: OutputPin,
    SIZE: DisplaySize,
{
    display: Ssd1306<SPIInterface<SPI, DC, CS>, SIZE, BufferedGraphicsMode<SIZE>>,
}

impl<SPI, DC, CS, SIZE> Ssd1306PongDrawer<SPI, DC, CS, SIZE>
where
    SPI: spi::Write<u8>,
    DC: OutputPin,
    CS: OutputPin,
    SIZE: DisplaySize,
{
    pub fn new(spi: SPI, dc: DC, cs: CS, size: SIZE) -> Self {
        let interface = SPIInterface::new(spi, dc, cs);
        let mut display =
            Ssd1306::new(interface, size, DisplayRotation::Rotate0).into_buffered_graphics_mode();
        display.init().unwrap();

        Self { display }
    }

    pub fn clear(&mut self) {
        Rectangle::new(Point::new(0, 0), Size::new(128, 64))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(BinaryColor::Off)
                    .stroke_color(BinaryColor::Off)
                    .build(),
            )
            .draw(&mut self.display)
            .unwrap();
    }

    fn draw_rect(&mut self, top_left: Point, size: Size) {
        Rectangle::new(top_left, size)
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(BinaryColor::On)
                    .stroke_width(0)
                    .build(),
            )
            .draw(&mut self.display)
            .unwrap();
    }
    pub fn flush(&mut self) {
        self.display.flush().unwrap();
    }
}

impl<SPI, DC, CS, SIZE> pong::Drawer for Ssd1306PongDrawer<SPI, DC, CS, SIZE>
where
    SPI: spi::Write<u8>,
    DC: OutputPin,
    CS: OutputPin,
    SIZE: DisplaySize,
{
    fn draw_ball(&mut self, ball: pong::Ball) {
        Circle::new(
            Point::new(ball.x as i32 - ball.r as i32, ball.y as i32 - ball.r as i32),
            ball.r * 2,
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(BinaryColor::On)
                .stroke_color(BinaryColor::On)
                .build(),
        )
        .draw(&mut self.display)
        .unwrap();
    }

    fn draw_player(&mut self, player: pong::Player) {
        self.draw_rect(
            Point::new(player.x, player.y),
            Size::new(player.width, player.height),
        )
    }
    fn draw_score(&mut self, score: (u32, u32)) {
        use numtoa::NumToA;
        let mut data1 = [0u8; 10];
        let mut data2 = [0u8; 10];
        let text1 = score.0.numtoa_str(10, &mut data1);
        let text2 = score.1.numtoa_str(10, &mut data2);

        let style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::Off)
            .stroke_color(BinaryColor::On)
            .stroke_width(1)
            .build();

        Rectangle::new(Point::new(38, 0), Size::new(26, 15))
            .into_styled(style)
            .draw(&mut self.display)
            .unwrap();
        Rectangle::new(Point::new(64, 0), Size::new(26, 15))
            .into_styled(style)
            .draw(&mut self.display)
            .unwrap();

        let style = MonoTextStyleBuilder::new()
            .font(&FONT_8X13)
            .text_color(BinaryColor::On)
            .build();

        Text::new(text1, Point::new(40, 11), style)
            .draw(&mut self.display)
            .unwrap();

        Text::new(text2, Point::new(66, 11), style)
            .draw(&mut self.display)
            .unwrap();
    }
}
