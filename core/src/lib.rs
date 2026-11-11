pub mod geo;
pub mod map;
pub mod vehicle;

use wasm_bindgen::prelude::*;
use map::RoadMap;
use vehicle::{Agent, VehicleClass};
use geo::{Vec2D, ANCHOR_LAT};
use rand::Rng;

#[wasm_bindgen]
pub struct SimulationEngine {
    roadmap: RoadMap,
    agents: Vec<Agent>,
    next_id: u32,
    signal_timer: f32,
    eastbound_green: bool,
    westbound_green: bool,
}

#[wasm_bindgen]
impl SimulationEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut engine = Self {
            roadmap: RoadMap::new(),
            agents: Vec::new(),
            next_id: 1,
            signal_timer: 0.0,
            eastbound_green: true,
            westbound_green: false,
        };
        engine.spawn_left_side_traffic();
        engine
    }

    pub fn get_anchor_lat(&self) -> f64 {
        ANCHOR_LAT
    }

    pub fn spawn_left_side_traffic(&mut self) {
        let mut rng = rand::thread_rng();

        // 1. Eastbound Traffic (Moving Right on the LEFT side of the road: y in [-48..-12])
        for i in 0..16 {
            let id = self.next_id;
            self.next_id += 1;
            let class_type = match rng.gen_range(0..4) {
                0 => VehicleClass::TwoWheeler,
                1 => VehicleClass::Auto,
                2 => VehicleClass::Car,
                _ => VehicleClass::Bus,
            };
            let x = -420.0 + (i as f32 * 35.0);
            let y = rng.gen_range(-48.0..-12.0); // Left side for eastbound in India
            let speed = rng.gen_range(4.0..6.5);

            let agent = Agent {
                id,
                class_type,
                pos: Vec2D::new(x, y),
                z_layer: 0,
                velocity: Vec2D::new(speed, 0.0),
                speed,
                length: match class_type { VehicleClass::TwoWheeler => 1.8, VehicleClass::Auto => 2.6, VehicleClass::Car => 4.2, _ => 10.0 },
                width: match class_type { VehicleClass::TwoWheeler => 0.8, VehicleClass::Auto => 1.3, VehicleClass::Car => 1.8, _ => 2.4 },
                impatience: rng.gen_range(0.0..1.0),
                is_wrong_side: false,
            };
            self.agents.push(agent);
        }

        // 2. Westbound Traffic (Moving Left on the LEFT side of the road: y in [12..48])
        for i in 0..16 {
            let id = self.next_id;
            self.next_id += 1;
            let class_type = match rng.gen_range(0..4) {
                0 => VehicleClass::TwoWheeler,
                1 => VehicleClass::Auto,
                2 => VehicleClass::Car,
                _ => VehicleClass::Bus,
            };
            let x = 420.0 - (i as f32 * 35.0);
            let y = rng.gen_range(12.0..48.0); // Left side for westbound in India
            let speed = rng.gen_range(4.0..6.5);

            let agent = Agent {
                id,
                class_type,
                pos: Vec2D::new(x, y),
                z_layer: 0,
                velocity: Vec2D::new(-speed, 0.0),
                speed,
                length: match class_type { VehicleClass::TwoWheeler => 1.8, VehicleClass::Auto => 2.6, VehicleClass::Car => 4.2, _ => 10.0 },
                width: match class_type { VehicleClass::TwoWheeler => 0.8, VehicleClass::Auto => 1.3, VehicleClass::Car => 1.8, _ => 2.4 },
                impatience: rng.gen_range(0.0..1.0),
                is_wrong_side: false,
            };
            self.agents.push(agent);
        }
    }

    pub fn step(&mut self, _dt: f32) {
        let frame_dt = 0.016;
        self.signal_timer += frame_dt;

        if self.signal_timer >= 6.0 {
            self.signal_timer = 0.0;
            let temp = self.eastbound_green;
            self.eastbound_green = self.westbound_green;
            self.westbound_green = temp;
        }

        let agent_count = self.agents.len();
        for i in 0..agent_count {
            let agent = &self.agents[i];
            let mut target_speed = agent.speed;
            let is_eastbound = agent.velocity.x > 0.0;

            let current_green = if is_eastbound { self.eastbound_green } else { self.westbound_green };
            let stop_x = if is_eastbound { -35.0 } else { 35.0 };

            if !current_green {
                let dist_to_stop = if is_eastbound { stop_x - agent.pos.x } else { agent.pos.x - stop_x };
                if dist_to_stop > 0.0 && dist_to_stop < 50.0 {
                    target_speed *= (dist_to_stop / 50.0).clamp(0.0, 1.0);
                } else if dist_to_stop <= 0.0 && dist_to_stop > -15.0 {
                    target_speed = 0.0;
                }
            }

            for j in 0..agent_count {
                if i == j { continue; }
                let other = &self.agents[j];
                let same_dir = (agent.velocity.x > 0.0) == (other.velocity.x > 0.0);
                if !same_dir { continue; }

                let dx = (other.pos.x - agent.pos.x) * agent.velocity.x.signum();
                let dy = (other.pos.y - agent.pos.y).abs();

                if dx > 0.0 && dx < 7.0 && dy < 2.5 {
                    target_speed = target_speed.min(other.speed * 0.4);
                    if dx < 4.0 {
                        target_speed = 0.0;
                    }
                }
            }

            let ag = &mut self.agents[i];
            ag.pos.x += ag.velocity.x.signum() * target_speed * frame_dt;

            // Strict Indian Left-Side Clamping
            if ag.velocity.x > 0.0 {
                ag.pos.y = ag.pos.y.clamp(-52.0, -5.0); // Eastbound on bottom/left side
            } else {
                ag.pos.y = ag.pos.y.clamp(5.0, 52.0);  // Westbound on top/left side
            }

            if ag.pos.x > 450.0 { ag.pos.x = -450.0; }
            if ag.pos.x < -450.0 { ag.pos.x = 450.0; }
        }
    }

    pub fn get_agents_buffer_ptr(&self) -> *const f32 {
        let mut raw_data: Vec<f32> = Vec::with_capacity(self.agents.len() * 4);
        for agent in &self.agents {
            raw_data.push(agent.pos.x);
            raw_data.push(agent.pos.y);
            raw_data.push(agent.z_layer as f32);
            raw_data.push(agent.class_type as u8 as f32);
        }
        let ptr = raw_data.as_ptr();
        std::mem::forget(raw_data);
        ptr
    }

    pub fn get_agents_count(&self) -> usize {
        self.agents.len()
    }

    pub fn get_eastbound_light(&self) -> bool {
        self.eastbound_green
    }

    pub fn get_westbound_light(&self) -> bool {
        self.westbound_green
    }
}