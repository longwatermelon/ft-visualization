use macroquad::prelude::*;
use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }
}

pub fn sample_points(f: fn(f32) -> f32, domain: (f32, f32), samples: i32) -> Vec<f32> {
    let mut data: Vec<f32> = Vec::new();
    for i in 0..samples {
        data.push(f((domain.1 - domain.0) / samples as f32 * i as f32));
    }

    return data;
}

pub fn draw_fn(f: fn(f32) -> f32, rect: Rect, domain: (f32, f32)) {
    let data: Vec<f32> = sample_points(f, domain, 1000);
    draw_points(&data, rect, WHITE);
}

pub fn draw_points(data: &Vec<f32>, rect: Rect, col: Color) {
    let mut ymin: f32 = f32::INFINITY;
    let mut ymax: f32 = f32::NEG_INFINITY;
    for &y in data {
        if y < ymin {
            ymin = y;
        }
        if y > ymax {
            ymax = y;
        }
    }

    let upp_y: f32 = (ymax - ymin) / (rect.h as f32);

    let mut prev: (f32, f32) = (0., 0.);
    for i in 0..data.len() {
        let mut p: (f32, f32) = (
            rect.x as f32 + (rect.w as f32 / data.len() as f32) * i as f32,
            (rect.y + rect.h) as f32 - data[i] / upp_y
        );

        if p.1 as i32 > rect.y + rect.h {
            p.1 = (rect.y + rect.h) as f32;
        }

        if i == 0 {
            draw_line(p.0, p.1, p.0, p.1, 2., col);
        } else {
            draw_line(prev.0, prev.1, p.0, p.1, 2., col);
        }
        prev = p;
    }
}

pub fn sample_polar(f: fn(f32) -> f32, domain: (f32, f32), freq: f32, samples: i32) -> Vec<(f32, f32)> {
    let mut coords: Vec<(f32, f32)> = Vec::new();
    let dt: f32 = (domain.1 - domain.0) / (samples as f32);
    for i in 0..samples {
        let t: f32 = i as f32 * dt;
        coords.push((
            f32::cos(2. * PI * freq * t) * f(t),
            -f32::sin(2. * PI * freq * t) * f(t)
        ));
    }

    return coords;
}

// Return ppu
pub fn draw_polar(f: fn(f32) -> f32, center: (f32, f32), radius: f32, domain: (f32, f32), freq: f32) -> f32 {
    let samples: i32 = 1000;
    let data: Vec<(f32, f32)> = sample_polar(f, domain, freq, samples);

    let dt: f32 = (domain.1 - domain.0) / (samples as f32);

    let mut vuy: Vec<f32> = Vec::new();
    for i in 0..samples {
        vuy.push(f(i as f32 * dt));
    }

    let mut max_y = -f32::INFINITY;
    for i in 0..vuy.len() {
        if vuy[i] > max_y {
            max_y = vuy[i];
        }
    }

    let ppu: f32 = radius / max_y;

    let mut prev: (f32, f32) = (center.0, center.1);
    for i in 0..samples {
        let point: (f32, f32) = (
            center.0 + data[i as usize].0 * ppu,
            center.1 + data[i as usize].1 * ppu
        );

        if i == 0 {
            draw_line(point.0, point.1, point.0, point.1, 2., GOLD);
        } else {
            draw_line(prev.0, prev.1, point.0, point.1, 2., Color::new(1., 1., 0.5, 1.));
        }

        prev = point;
    }

    return ppu;
}

