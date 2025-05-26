use std::time::{Duration, Instant};
use eframe::egui::{self, TextureHandle};
use egui::{RichText, Color32, Vec2, Button};
use eframe::egui::ImageButton;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 600.0])
            .with_min_inner_size([600.0, 600.0])
            .with_max_inner_size([600.0, 600.0])
            .with_transparent(true)
            .with_title("Ticktoro"),
        ..Default::default()
    };
    eframe::run_native("Ticktoro", options, Box::new(|_cc| Box::<MyApp>::default()))
}

struct MyApp {
    play_image: Option<TextureHandle>,
    status_label: Option<String>,
    left_time_message: String,
    minutes: u64,
    remaining_secs: u64,
    end_time: Option<Instant>,
    is_paused: bool,
    last_tick: Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            play_image: None,
            status_label: None,
            left_time_message: String::from("0\n0"),
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
        if self.play_image.is_none() {
            let bytes = include_bytes!("../assets/play.png");
            let image = image::load_from_memory(bytes).unwrap().to_rgba8();
            let size = [image.width() as usize, image.height() as usize];
            let pixels = image.into_vec();
            let texture = ctx.load_texture(
                "play_icon",
                egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                Default::default(),
            );
            self.play_image = Some(texture);
        }

        let mut background_color: Color32 = if !self.is_paused && !self.end_time.is_none() {
            Color32::from_rgb(255, 243, 242)
        } else if self.is_paused {
            Color32::from_rgb(255, 243, 242)
        } else {
            Color32::from_rgb(255, 255, 255)
        };

        let frame = egui::Frame::none().fill(background_color);

        self.status_label = if self.is_paused {
            Some(String::from("Paused"))
        } else if !self.is_paused && !self.end_time.is_none() {
            Some(String::from("Focus"))
        } else {
            None
        };

        self.left_time_message = if self.remaining_secs > 0 {
            format!("{} \n {}", self.remaining_secs / 60, self.remaining_secs % 60)
        } else {
            format!("{} \n 0", self.minutes)
        };

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
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

                // let color = if self.is_paused {
                //     Color32::from_rgb(0, 0, 255)
                // } else if !self.is_paused && !self.end_time.is_none() {
                //     Color32::from_rgb(255, 165, 0)
                // } else {
                //     Color32::from_rgb(100, 100, 100)
                // };

                if let Some(status) = &self.status_label {
                    ui.label(
                        RichText::new(status)
                            .size(15.0)
                    );
                } else {
                    ui.add_space(20.0);
                }

                ui.add_space(30.0);

                ui.label(
                    RichText::new(&self.left_time_message)
                        .size(100.0)
                );

                // ui.horizontal(|ui| {
                //     ui.add_space(85.0);
                //     ui.group(|ui| {
                //         ui.add_enabled(
                //             self.end_time.is_none(),
                //             Slider::new(&mut self.minutes, 1..=120).text("Minutes")
                //         );
                //     });
                // });

                ui.add_space(40.0);

                if (!self.is_paused && self.end_time.is_none()) {
                    let start_button: egui::Response = if let Some(texture) = &self.play_image {
                        let image_size = Vec2::new(20.0, 20.0);

                        ui.add_sized(
                            Vec2::new(60.0, 50.0),
                            Button::image((texture.id(), image_size))
                                .rounding(15.0)
                                .fill(Color32::from_rgb(255, 124, 124)),
                        )
                    } else {
                        ui.add_sized(
                            Vec2::new(60.0, 50.0),
                            Button::new(
                                RichText::new("Start")
                                    .size(20.0),
                            )
                            .rounding(15.0)
                            .fill(Color32::from_rgb(255, 124, 124)),
                        )
                    };

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
                            Vec2::new(60.0, 50.0),
                            Button::new(
                                RichText::new("Res")
                                    .size(20.0)
                            ).rounding(15.0)
                             .fill(Color32::from_rgb(255,124,124)),
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


