use crate::cardioid;
use eframe::egui;
use std::collections::VecDeque;
use std::time::Instant;

struct TrailFrame {
    factor: f32,
    created_at: Instant,
}

pub struct CardioidApp {
    factor: f32,
    points: usize,
    speed: f32,
    playing: bool,
    trail_duration: f32,
    history: VecDeque<TrailFrame>,
    last_update: Instant,
    sample_accumulator: f32,
    fullscreen: bool,
    show_controls: bool,
}

impl Default for CardioidApp {
    fn default() -> Self {
        Self {
            factor: 2.0,
            points: 250,
            speed: 0.5,
            playing: false,
            trail_duration: 1.5,
            history: VecDeque::new(),
            last_update: Instant::now(),
            sample_accumulator: 0.0,
            fullscreen: false,
            show_controls: true,
        }
    }
}

impl CardioidApp {
    fn animate(&mut self, ctx: &egui::Context) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32().min(0.1);

        self.last_update = now;

        if self.playing {
            // Sauvegarde un état toutes les 50 ms.
            self.sample_accumulator += dt;

            if self.sample_accumulator >= 0.05 {
                self.history.push_back(TrailFrame {
                    factor: self.factor,
                    created_at: now,
                });

                self.sample_accumulator = 0.0;
            }

            self.factor += self.speed * dt;

            if self.factor > 10.0 {
                self.factor = 1.0;
                self.history.clear();
            }

            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }

        // Supprime les états trop anciens.
        while let Some(frame) = self.history.front() {
            if now.duration_since(frame.created_at).as_secs_f32() > self.trail_duration {
                self.history.pop_front();
            } else {
                break;
            }
        }

        // Continue à effacer progressivement la traînée en pause.
        if !self.playing && !self.history.is_empty() {
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        let (f11, tab, escape) = ctx.input(|input| {
            (
                input.key_pressed(egui::Key::F11),
                input.key_pressed(egui::Key::Tab),
                input.key_pressed(egui::Key::Escape),
            )
        });

        if f11 {
            self.fullscreen = !self.fullscreen;

            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));

            self.show_controls = !self.fullscreen;
        }

        if tab {
            self.show_controls = !self.show_controls;
        }

        if escape && self.fullscreen {
            self.fullscreen = false;
            self.show_controls = true;

            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        }
    }

    fn controls(&mut self, ui: &mut egui::Ui) {
        ui.heading(
            egui::RichText::new("CARDIOID STUDIO")
                .strong()
                .color(egui::Color32::from_rgb(120, 190, 255)),
        );

        ui.label(
            egui::RichText::new("Visualisation mathématique")
                .small()
                .color(egui::Color32::GRAY),
        );

        ui.add_space(18.0);
        ui.separator();
        ui.add_space(12.0);

        // Transport
        ui.horizontal(|ui| {
            let label = if self.playing {
                "⏸ Pause"
            } else {
                "▶ Lecture"
            };

            if ui.button(label).clicked() {
                self.playing = !self.playing;
                self.last_update = std::time::Instant::now();
            }

            if ui.button("↺ Reset").clicked() {
                self.playing = false;
                self.factor = 2.0;
                self.history.clear();
                self.sample_accumulator = 0.0;
            }
        });

        ui.add_space(20.0);

        // Géométrie
        ui.label(
            egui::RichText::new("GÉOMÉTRIE")
                .strong()
                .color(egui::Color32::LIGHT_BLUE),
        );

        ui.add_space(8.0);

        ui.label("Multiplicateur");

        if ui
            .add(egui::Slider::new(&mut self.factor, 1.0..=10.0).fixed_decimals(2))
            .changed()
        {
            self.history.clear();
        }

        ui.add_space(8.0);
        ui.label("Nombre de points");

        ui.add(egui::Slider::new(&mut self.points, 50..=500));

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(12.0);

        // Animation
        ui.label(
            egui::RichText::new("ANIMATION")
                .strong()
                .color(egui::Color32::LIGHT_BLUE),
        );

        ui.add_space(8.0);
        ui.label("Vitesse");

        ui.add(egui::Slider::new(&mut self.speed, 0.1..=3.0).suffix(" /s"));

        ui.add_space(8.0);
        ui.label("Persistance");

        ui.add(egui::Slider::new(&mut self.trail_duration, 0.0..=5.0).suffix(" s"));
        if ui.button("⛶ Plein écran (F11)").clicked() {
            self.fullscreen = !self.fullscreen;

            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));

            self.show_controls = !self.fullscreen;
        }
    }

    fn canvas(&self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();

        let radius = (rect.width().min(rect.height()) / 2.0 - 20.0).max(0.0);

        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(12, 16, 25));

        painter.circle_stroke(
            center,
            radius,
            egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
        );

        let now = Instant::now();

        // Dessin de la traînée temporelle.
        if self.trail_duration > 0.0 {
            for frame in &self.history {
                let age = now.duration_since(frame.created_at).as_secs_f32();

                let remaining = (1.0 - age / self.trail_duration).clamp(0.0, 1.0);

                let opacity = 0.3 * remaining.powi(2);

                cardioid::draw(&painter, center, radius, self.points, frame.factor, opacity);
            }
        }

        // Figure actuelle.
        cardioid::draw(&painter, center, radius, self.points, self.factor, 0.9);
    }
}
impl eframe::App for CardioidApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.handle_shortcuts(ui.ctx());
        self.animate(ui.ctx());

        if !self.show_controls {
            self.canvas(ui);
            return;
        }

        let available = ui.available_size();
        let sidebar_width = 250.0_f32.min(available.x * 0.45);
        let spacing = 12.0;
        let canvas_width = (available.x - sidebar_width - spacing).max(0.0);

        ui.horizontal(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(sidebar_width, available.y),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgb(30, 36, 48))
                        .inner_margin(16.0)
                        .corner_radius(8.0)
                        .show(ui, |ui| {
                            ui.set_min_width((sidebar_width - 32.0).max(0.0));

                            // Occupe toute la hauteur disponible,
                            // en tenant compte des marges du panneau.
                            ui.set_min_height((available.y - 32.0).max(0.0));

                            self.controls(ui);
                        });
                },
            );

            ui.allocate_ui_with_layout(
                egui::vec2(canvas_width, available.y),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    self.canvas(ui);
                },
            );
        });
    }
}
