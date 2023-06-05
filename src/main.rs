use macroquad::prelude::*;
use std::f32::consts::PI;

mod graph;

fn f(x: f32) -> f32 {
    f32::cos(2. * PI * 3. * x) + 1.
}

fn sample_coords(domain: (f32, f32), freq_range: (f32, f32)) -> Vec<(f32, f32)> {
    let mut vcm: Vec<(f32, f32)> = Vec::new();

    for i in 0..2000 {
        let freq: f32 = freq_range.0 + (freq_range.1 - freq_range.0) / 2000. * i as f32;
        let coords: Vec<(f32, f32)> = graph::sample_polar(f, domain, freq, 1000);

        let mut cm: (f32, f32) = (0., 0.);
        for &coord in &coords {
            cm.0 += coord.0;
            cm.1 += coord.1;
        }
        cm.0 /= coords.len() as f32;
        cm.1 /= coords.len() as f32;
        vcm.push(cm);
    }

    return vcm;
}

#[macroquad::main(window_conf)]
async fn main() {
    let domain: (f32, f32) = (0., 4.);
    let mut freq: f32;

    let coords: Vec<(f32, f32)> = sample_coords(domain, (0.05, 4.05));
    let mut xcoords: Vec<f32> = Vec::new();
    let mut ycoords: Vec<f32> = Vec::new();
    for &coord in &coords {
        xcoords.push(coord.0 + 0.4);
        ycoords.push(-coord.1 + 0.4);
    }

    loop {
        freq = 2. * f32::sin(macroquad::time::get_time() as f32 / 2.) + 2.05;

        clear_background(BLACK);

        graph::draw_fn(
            f,
            graph::Rect::new(0, 0, 800, 200),
            domain
        );
        draw_line(0., 200., 800., 200., 2., WHITE);

        let ppu: f32 = 800 as f32 / (domain.1 - domain.0);
        for i in 0..50 {
            let x: f32 = i as f32 / freq * ppu;
            if x > 800. {
                break;
            }

            draw_line(x, 0., x, 200., 2., RED);
        }

        let ppu: f32 = graph::draw_polar(
            f,
            (150., 350.),
            100.,
            domain,
            freq
        );
        draw_text(format!("Frequency: {:.2} cycles/second", freq).as_str(), 10., 600. - 24., 24., WHITE);

        let freq_rect: graph::Rect = graph::Rect::new(400, 270, 350, 350);
        graph::draw_points(&xcoords, freq_rect, WHITE);
        graph::draw_points(&ycoords, freq_rect, GRAY);
        let freq_x: f32 = 400. + (350. / (domain.1 - domain.0)) * freq;
        draw_line(freq_x, 600., freq_x, 200., 2., GREEN);

        let polar_data: Vec<(f32, f32)> = graph::sample_polar(f, domain, freq, 1000);
        let mut cm: (f32, f32) = (0., 0.);
        for &coord in &polar_data {
            cm.0 += coord.0;
            cm.1 += coord.1;
        }
        cm.0 /= polar_data.len() as f32;
        cm.1 /= polar_data.len() as f32;
        let circle: (f32, f32) = (150. + cm.0 * ppu, 350. + cm.1 * ppu);
        draw_circle(circle.0, circle.1, 5., GREEN);

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_resizable: false,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

