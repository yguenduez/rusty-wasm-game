use crate::engine::{Image, Rect, SpriteSheet};
use crate::game::{
    Barrier, Obstacle, Platform, Point, FIRST_PLATFORM, HIGH_PLATFORM, LOW_PLATFORM,
};
use std::rc::Rc;
use web_sys::HtmlImageElement;

pub fn stone_and_platform(
    stone: HtmlImageElement,
    sprite_sheet: Rc<SpriteSheet>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    const INITIAL_STONE_OFFSET: i16 = 150;
    vec![
        Box::new(Barrier::new(Image::new(
            stone,
            Point {
                x: offset_x + INITIAL_STONE_OFFSET,
                y: STONE_ON_GROUND,
            },
        ))),
        Box::new(create_floating_platform(
            sprite_sheet,
            Point {
                x: offset_x + FIRST_PLATFORM,
                y: LOW_PLATFORM,
            },
        )),
    ]
}

pub fn other_platform(sprite_sheet: Rc<SpriteSheet>, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
    const INITIAL_STONE_OFFSET: i16 = 150;
    vec![Box::new(create_cliff_platform(
        sprite_sheet,
        Point {
            x: offset_x + FIRST_PLATFORM,
            y: HIGH_PLATFORM,
        },
    ))]
}

pub const STONE_ON_GROUND: i16 = 550;
pub const FLOATING_PLATFORM_SPRITES: [&str; 3] = ["13.png", "14.png", "15.png"];
pub const FLOATING_PLATFORM_BOUNDING_BOXES: [Rect; 3] = [
    Rect::new_from_x_y(0, 0, 60, 54),
    Rect::new_from_x_y(60, 0, 384 - (60 * 2), 93),
    Rect::new_from_x_y(384 - 60, 0, 60, 54),
];

fn create_floating_platform(sprite_sheet: Rc<SpriteSheet>, position: Point) -> Platform {
    Platform::new(
        sprite_sheet,
        position,
        &FLOATING_PLATFORM_SPRITES,
        &FLOATING_PLATFORM_BOUNDING_BOXES,
    )
}

pub const CLIFF_SPRITES: [&str; 3] = ["1.png", "1.png", "3.png"];
fn create_cliff_platform(sprite_sheet: Rc<SpriteSheet>, position: Point) -> Platform {
    Platform::new(
        sprite_sheet,
        position,
        &CLIFF_SPRITES,
        &FLOATING_PLATFORM_BOUNDING_BOXES,
    )
}
