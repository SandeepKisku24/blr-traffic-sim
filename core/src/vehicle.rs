use crate::geo::Vec2D;
use serde::{Serialize, Deserialize};

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum VehicleClass {
    TwoWheeler = 0,
    Auto = 1,
    Car = 2,
    Bus = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Agent {
    pub id: u32,
    pub class_type: VehicleClass,
    pub pos: Vec2D,
    pub z_layer: i8,
    pub velocity: Vec2D,
    pub speed: f32,
    pub length: f32,
    pub width: f32,
    pub impatience: f32,
    pub is_wrong_side: bool,
}

impl Agent {
    pub fn new(id: u32, class_type: VehicleClass, pos: Vec2D, z_layer: i8, is_wrong_side: bool) -> Self {
        let (length, width, max_speed) = match class_type {
            VehicleClass::TwoWheeler => (1.8, 0.8, 5.0),
            VehicleClass::Auto => (2.6, 1.3, 3.8),
            VehicleClass::Car => (4.2, 1.8, 4.2),
            VehicleClass::Bus => (11.0, 2.5, 2.8),
        };

        Self {
            id,
            class_type,
            pos,
            z_layer,
            velocity: Vec2D::new(0.0, 0.0),
            speed: max_speed,
            length,
            width,
            impatience: 0.0,
            is_wrong_side,
        }
    }
}