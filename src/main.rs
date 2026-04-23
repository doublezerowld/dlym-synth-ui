use embedded_graphics::{
    image::Image,
    mono_font::{
        MonoTextStyle, MonoTextStyleBuilder,
        ascii::{FONT_5X7, FONT_8X13_BOLD},
    },
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
    primitives::Rectangle,
    text::{Alignment, Text},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use tinybmp::Bmp;

const DISABLED_OPACITY: f32 = 0.33;

// Preset dimensions

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

const H_CENTER: i32 = WIDTH as i32 / 2;
const V_CENTER: i32 = HEIGHT as i32 / 2;

// Preset points
const NOISE_REG: Point = Point::new(51, 169);
const ENV_REG: Point = Point::new(51, 182);

const CONSOLE: Point = Point::new(125, 43);

// Preset rects

const TONE_MASK_SIZE: Size = Size::new(6, 9);
const NOISE_MASK_SIZE: Size = Size::new(8, 9);

const METER_SIZE: Size = Size::new(10, 64);
const A_RECTS: [Rectangle; 3] = [
    Rectangle::new(Point::new(45, 67), TONE_MASK_SIZE),
    Rectangle::new(Point::new(45, 67), TONE_MASK_SIZE),
    Rectangle::new(Point::new(53, 67), NOISE_MASK_SIZE),
];

//const CONSOLE_TEXT_STYLE: MonoTextStyle<'a, C> = (

//)

// Masks
struct ChannelUIMasks {
    channel: Rectangle,
    tone: Rectangle,
    noise: Rectangle,
}

enum ChannelUIElement {
    EntireChannel,
    ToneGenerator,
    NoiseGenerator,
}

struct ChannelUI {
    pub masks: ChannelUIMasks,
    pub meter_rect: Rectangle,
}

// Channel UI Controller
impl ChannelUI {
    fn new(&mut self, meter_top_left: Point, masks: ChannelUIMasks) -> Self {
        Self {
            masks,
            meter_rect: Rectangle::with_corners(meter_top_left, meter_top_left + METER_SIZE),
        }
    }

    fn meter(&mut self, value: u8) -> Rectangle {
        let fraction: f32 = (value / 255).into();
        let new_height = (fraction * METER_SIZE.height as f32).round() as u32;
        self.meter_rect
            .resized_height(new_height, embedded_graphics::geometry::AnchorY::Bottom)
    }

    fn set_state(&mut self, element: ChannelUIElement, enabled: bool) -> Rectangle {
        Rectangle::zero()
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> =
        SimulatorDisplay::new(Size::new(WIDTH, HEIGHT));

    let bmp: Bmp<BinaryColor> = Bmp::from_slice(include_bytes!("../static.bmp")).unwrap();

    let image = Image::new(&bmp, Point::new(0, -1));
    image.draw(&mut display)?;

    let status = Text::with_alignment(
        "USB CONNECTED",
        Point::new(H_CENTER, 13),
        MonoTextStyle::new(&FONT_8X13_BOLD, BinaryColor::Off),
        Alignment::Center,
    );

    status.draw(&mut display)?;

    let firmware_version = Text::with_alignment(
        "FIRMWARE VERSION 0.1a",
        Point::new(H_CENTER, 240 - 10),
        MonoTextStyle::new(&FONT_5X7, BinaryColor::On),
        Alignment::Center,
    );

    firmware_version.draw(&mut display)?;

    let console_text_style = MonoTextStyleBuilder::<BinaryColor>::new()
        .font(&FONT_5X7)
        .text_color(BinaryColor::On)
        .build();

    let console_text = Text::with_baseline(
        "$ ON(A, A4, 127) => [\n. 0x0 0x0;\n. 0x1 0xC1;\n. 0x8 0xF\n. ]\n$ OFF(B) => 0x9 0x0;\n$ CC(A, 7, 63) => 0x8 0x7;\n$",
        CONSOLE,
        console_text_style,
        embedded_graphics::text::Baseline::Top,
    );

    console_text.draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("YM2149 MIDI Interpreter", &output_settings).show_static(&display);

    Ok(())
}
