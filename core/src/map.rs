use crate::geo::Vec2D;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LaneCorridor {
    pub length: f32,
    pub width: f32,
    pub median_position: f32, // The dividing line coordinates
    pub stop_line_pos: f32,   // Where vehicles must stop for red lights
}

pub struct RoadMap {
    pub corridor: LaneCorridor,
}

impl RoadMap {
    pub fn new() -> Self {
        Self {
            corridor: LaneCorridor {
                length: 800.0,
                width: 120.0,       // Total road width
                median_position: 0.0, // Center divider
                stop_line_pos: -30.0, // Stop line before the intersection
            },
        }
    }
}