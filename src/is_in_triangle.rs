extern crate rand;
use rand::prelude::*;

pub fn triangle_test (px: f32,py: f32, ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32) -> bool {
    let v0x: f32 = cx - ax;
    let v0y: f32 = cy - ay;
    let v1x: f32 = bx - ax;
    let v1y: f32 = by - ay;
    let v2x: f32 = px - ax;
    let v2y: f32 = py - ay;

    let dot00: f32 = (v0x * v0x) + (v0y * v0y);
    let dot01: f32 = (v0x * v1x) + (v0y * v1y);
    let dot02: f32 = (v0x * v2x) + (v0y * v2y);
    let dot11: f32 = (v1x * v1x) + (v1y * v1y);
    let dot12: f32 = (v1x * v2x) + (v1y * v2y);

    let inv_denom: f32 = 1.0 / (dot00 * dot11 - dot01 * dot01);

    let u: f32 = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v: f32 = (dot00 * dot12 - dot01 * dot02) * inv_denom;

    (u >= 0.0) && (v >= 0.0) && (u + v < 1.0)
}

pub fn dumb_test (px: f32,py: f32, ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32) -> () {
}

pub struct TestStuff {
    pub px: Vec<f32>,
    pub py: Vec<f32>,
    pub ax: Vec<f32>,
    pub ay: Vec<f32>,
    pub bx: Vec<f32>,
    pub by: Vec<f32>,
    pub cx: Vec<f32>,
    pub cy: Vec<f32>
}

fn make_vec () -> Vec<f32> {
    const LENGTH: usize = 1000;
    let mut rng = thread_rng();
    (0..LENGTH).map(|_| {
        rng.gen::<f32>()
    }).collect()
}

impl TestStuff {
    pub fn new() -> TestStuff {
        TestStuff {
            px: make_vec(),
            py: make_vec(),
            ax: make_vec(),
            ay: make_vec(),
            bx: make_vec(),
            by: make_vec(),
            cx: make_vec(),
            cy: make_vec(),
        }
    }
}