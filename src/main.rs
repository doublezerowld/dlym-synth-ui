use embedded_graphics::{
    image::Image,
    mono_font::{
        MonoTextStyle, MonoTextStyleBuilder,
        ascii::{FONT_5X7, FONT_8X13_BOLD},
    },
    pixelcolor::{BinaryColor, Gray2, Gray4, Rgb565},
    prelude::*,
    primitives::Rectangle,
    text::{Alignment, Text},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use tinybmp::Bmp;

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

const CHANNEL_MASK_SIZE: Size = Size::new(12, 12);
const TONE_MASK_SIZE: Size = Size::new(6, 9);
const NOISE_MASK_SIZE: Size = Size::new(8, 9);

const METER_MASK_SIZE: Size = Size::new(12, 76);
const METER_SIZE: Size = Size::new(10, 64);

const A_MASKS: ChannelUIMasks = ChannelUIMasks {
    channel: Rectangle::new(Point::new(32, 66), CHANNEL_MASK_SIZE),
    tone: Rectangle::new(Point::new(45, 67), TONE_MASK_SIZE),
    noise: Rectangle::new(Point::new(53, 67), NOISE_MASK_SIZE),
    meter: Rectangle::new(Point::new(38, 87), METER_MASK_SIZE),
};

// Masks
struct ChannelUIMasks {
    channel: Rectangle,
    tone: Rectangle,
    noise: Rectangle,
    meter: Rectangle,
}

enum ChannelUIElement {
    EntireChannel,
    ToneGenerator,
    NoiseGenerator,
    Meter,
}

struct ChannelUI {
    pub masks: ChannelUIMasks,
    pub volume_rect: Rectangle,
}

// Channel UI Controller
impl ChannelUI {
    fn new(meter_top_left: Point, masks: ChannelUIMasks) -> Self {
        Self {
            masks,
            volume_rect: Rectangle::with_corners(meter_top_left, meter_top_left + METER_SIZE),
        }
    }

    fn meter(&mut self, value: u8) -> Rectangle {
        let fraction: f32 = (value / 255).into();
        let new_height = (fraction * METER_SIZE.height as f32).round() as u32;
        self.volume_rect
            .resized_height(new_height, embedded_graphics::geometry::AnchorY::Bottom)
    }

    fn set_state(&mut self, element: ChannelUIElement, enabled: bool) -> Rectangle {
        Rectangle::zero()
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Gray4> = SimulatorDisplay::new(Size::new(WIDTH, HEIGHT));

    let bmp: Bmp<Gray4> = Bmp::from_slice(include_bytes!("../artboard.bmp")).unwrap();

    let image = Image::new(&bmp, Point::new(0, -1));
    image.draw(&mut display)?;

    let status = Text::with_alignment(
        "USB CONNECTED",
        Point::new(H_CENTER, 13),
        MonoTextStyle::new(&FONT_8X13_BOLD, Gray4::BLACK),
        Alignment::Center,
    );

    status.draw(&mut display)?;

    let firmware_version = Text::with_alignment(
        "FIRMWARE VERSION 0.1a",
        Point::new(H_CENTER, 240 - 10),
        MonoTextStyle::new(&FONT_5X7, Gray4::WHITE),
        Alignment::Center,
    );

    firmware_version.draw(&mut display)?;

    let console_text_style = MonoTextStyleBuilder::<Gray4>::new()
        .font(&FONT_5X7)
        .text_color(Gray4::WHITE)
        .build();

    let console_text = Text::with_baseline(
        "$ ON(A, A4, 127) => [\n. 0x0 0x0;\n. 0x1 0xC1;\n. 0x8 0xF\n. ]\n$ OFF(B) => 0x9 0x0;\n$ CC(A, 7, 63) => 0x8 0x7;\n$",
        CONSOLE,
        console_text_style,
        embedded_graphics::text::Baseline::Top,
    );

    console_text.draw(&mut display)?;

    let ui_a = ChannelUI::new(Point::new(40, 89), A_MASKS);

    ui_a.

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("YM2149 MIDI Interpreter", &output_settings).show_static(&display);

    Ok(())
}
