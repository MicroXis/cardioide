use eframe::egui;
use std::f32::consts::TAU;

fn hsv_color(hue: f32, alpha: f32) -> egui::Color32 {
    let h = hue.rem_euclid(1.0) * 6.0;
    let x = 1.0 - ((h % 2.0) - 1.0).abs();

    let (r, g, b) = match h as u32 {
        0 => (1.0, x, 0.0),
        1 => (x, 1.0, 0.0),
        2 => (0.0, 1.0, x),
        3 => (0.0, x, 1.0),
        4 => (x, 0.0, 1.0),
        _ => (1.0, 0.0, x),
    };

    let a = (alpha.clamp(0.0, 1.0) * 255.0) as u8;

    egui::Color32::from_rgba_premultiplied(
        (r * a as f32) as u8,
        (g * a as f32) as u8,
        (b * a as f32) as u8,
        a,
    )
}

pub fn draw(
    painter: &egui::Painter,
    center: egui::Pos2,
    radius: f32,
    points: usize,
    factor: f32,
    opacity: f32,
) {
    if points == 0 || radius <= 0.0 {
        return;
    }

    for i in 0..points {
        let a = TAU * i as f32 / points as f32;
        let b = a * factor;

        let p1 = center + radius * egui::vec2(a.cos(), a.sin());
        let p2 = center + radius * egui::vec2(b.cos(), b.sin());

        let hue = i as f32 / points as f32;

        painter.line_segment([p1, p2], egui::Stroke::new(0.8, hsv_color(hue, opacity)));
    }
}
