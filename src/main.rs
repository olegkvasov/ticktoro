use std::time::{Duration, Instant};
use eframe::egui::{self, Slider};
use egui::{RichText, Color32, Vec2, Button, Layout, Align};
use std::io::BufReader;

fn main() -> Result<(), eframe::Error> {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let file = std::fs::File::open("assets/sound.wav").unwrap();
    let sound = stream_handle.play_once(BufReader::new(file)).unwrap();
    sound.set_volume(0.3);
    sound.detach();
    println!("Started sound");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 350.0])
            .with_min_inner_size([400.0, 350.0])
            .with_max_inner_size([400.0, 350.0])
            .with_title("Pomodoro Timer"),
        ..Default::default()
    };
    eframe::run_native("Pomodoro Timer", options, Box::new(|_cc| Box::<MyApp>::default()))
}

struct MyApp {
    minutes: u64,
    remaining_secs: u64,
    end_time: Option<Instant>,
    is_paused: bool,
    last_tick: Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            minutes: 25,
            remaining_secs: 0,
            end_time: None,
            is_paused: false,
            last_tick: Instant::now(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);

                if !self.is_paused {
                    let now = Instant::now();
                    if now.duration_since(self.last_tick) >= Duration::from_secs(1) {
                        if self.remaining_secs > 0 {
                            self.remaining_secs -= 1;
                            self.last_tick = now;
                        } else {
                            self.end_time = None;
                        }
                    }
                }

                let message = if self.is_paused {
                    String::from("Timer is paused")
                } else if !self.is_paused && !self.end_time.is_none() {
                    String::from(format!("Time left: {} sec", self.remaining_secs))
                } else {
                    String::from(format!("Ready: {} min.", self.minutes))
                };

                let color = if self.is_paused {
                    Color32::from_rgb(0, 0, 255)
                } else if !self.is_paused && !self.end_time.is_none() {
                    Color32::from_rgb(255, 165, 0)
                } else {
                    Color32::from_rgb(100, 100, 100)
                };

                ui.label(
                    RichText::new(message)
                        .size(30.0)
                        .color(color)
                );

                ui.add_space(30.0);

                ui.horizontal(|ui| {
                    ui.add_space(85.0);
                    ui.group(|ui| {
                        ui.add_enabled(
                            self.end_time.is_none(),
                            Slider::new(&mut self.minutes, 1..=120).text("Minutes")
                        );
                    });
                });

                ui.add_space(40.0);

                if (!self.is_paused && self.end_time.is_none()) {
                    let start_button = ui.add_sized(
                        Vec2::new(200.0, 40.0),
                        Button::new(
                            RichText::new("Start timer")
                                .size(20.0)
                                .color(Color32::BLACK)
                                .strong(),
                        ),
                    );

                    if start_button.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }

                    if start_button.clicked() {
                        self.remaining_secs = self.minutes * 60;
                        self.end_time = Some(Instant::now());
                        self.last_tick = Instant::now();
                        self.is_paused = false;
                    }

                    println!("Timer: {}", self.remaining_secs);

                } else if !self.end_time.is_none() {
                        let resume_button = ui.add_sized(
                            Vec2::new(200.0, 40.0),
                            Button::new(
                                RichText::new(if self.is_paused { "Resume" } else { "Pause" })
                                    .size(20.0)
                                    .color(Color32::BLACK)
                                    .strong(),
                            ),
                        );
                        if resume_button.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        if resume_button.clicked() {
                            self.is_paused = !self.is_paused;
                            // При паузе фиксируем last_tick
                            if !self.is_paused {
                                self.last_tick = Instant::now();
                            }
                        }

                        ui.add_space(10.0);

                        let reset_button = ui.add_sized(
                            Vec2::new(200.0, 40.0),
                            Button::new(
                                RichText::new("Reset timer")
                                    .size(20.0)
                                    .color(Color32::BLACK)
                                    .strong(),
                            ),
                        );
                        if reset_button.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        if reset_button.clicked() {
                            self.remaining_secs = 0;
                            self.end_time = None;
                            self.is_paused = false;
                            self.last_tick = Instant::now();
                        }
                }

                ctx.request_repaint();
            });
        });
    }
}


