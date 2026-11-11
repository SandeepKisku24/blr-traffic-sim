use serde::{Serialize, Deserialize};


// Sarjapur Signal Reference Point
pub const ANCHOR_LAT: f64 = 12.920566;
pub const ANCHOR_LON: f64 = 77.665225;

const EARTH_RADIUS_M: f64 = 6_371_000.0;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Vec2D {
    pub x: f32, // Easting in meters relative to anchor
    pub y: f32, // Northing in meters relative to anchor
}

impl Vec2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Converts raw GPS (lat, lon) to local Cartesian meters relative to ANCHOR
    pub fn from_lat_lon(lat: f64, lon: f64) -> Self {
        let lat_rad = lat.to_radians();
        let lon_rad = lon.to_radians();
        let ref_lat_rad = ANCHOR_LAT.to_radians();
        let ref_lon_rad = ANCHOR_LON.to_radians();

        let x = EARTH_RADIUS_M * (lon_rad - ref_lon_rad) * ref_lat_rad.cos();
        let y = EARTH_RADIUS_M * (lat_rad - ref_lat_rad);

        Self {
            x: x as f32,
            y: y as f32,
        }
    }

    pub fn distance(&self, other: &Vec2D) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}