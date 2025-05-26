use std::time::{Duration, Instant};
use eframe::{egui::{self, Context, Response, TextureHandle, Rounding, Sense, TextureId, Ui, Layout, Align}};
use egui::{RichText, Color32, Vec2, Button};
use egui_extras::image::load_svg_bytes;

struct MyApp {
    play_image_bytes: &'static [u8],
    pause_image_bytes: &'static [u8],
    stop_image_bytes: &'static [u8],
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
            play_image_bytes: include_bytes!("../assets/play.svg"),
            pause_image_bytes: include_bytes!("../assets/pause.svg"),
            stop_image_bytes: include_bytes!("../assets/stop.svg"),
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

                ui.columns(2, |columns| {
                    columns[0].with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let texture = if !self.is_paused && !self.end_time.is_none() {
                            load_svg_texture(ctx, self.pause_image_bytes)
                        } else {
                            load_svg_texture(ctx, self.play_image_bytes)
                        };
                        let size_icon = Vec2::new(20.0, 20.0);
                        let size_button = Vec2::new(60.0, 50.0);
        
                        let play_button = centered_image_button(
                            ui,
                            texture.id(),
                            size_icon,
                            size_button,
                            20.0,
                            Color32::from_rgb(255, 124, 124),
                        );
        
                        if play_button.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
        
                        if play_button.clicked() {
                            if self.end_time.is_none() && !self.is_paused {
                                self.remaining_secs = self.minutes * 60;
                                self.end_time = Some(Instant::now());
                                self.last_tick = Instant::now();
                                self.is_paused = false;
                            } else {
                                self.is_paused = !self.is_paused;
                                // При паузе фиксируем last_tick
                                if !self.is_paused {
                                    self.last_tick = Instant::now();
                                }
                            }
                        }
                    });
                        if !self.end_time.is_none() {
                            columns[1].with_layout(Layout::left_to_right(Align::Center), |ui| {
                                let stop_texture = load_svg_texture(ctx, self.stop_image_bytes);
                                let size_stop_icon = Vec2::new(30.0, 30.0);
                                let size_button = Vec2::new(55.0, 45.0);
            
                                let next_stage = centered_image_button(
                                    ui,
                                    stop_texture.id(),
                                    size_stop_icon,
                                    size_button,
                                    20.0,
                                    Color32::from_rgb(255, 124, 124),
                                );
            
                                if next_stage.hovered() {
                                    ui.ctx().set_cursor_icon(egui::CursorIcon::NotAllowed);
                                }
                                // if stop_button.clicked() {
                                //         self.remaining_secs = 0;
                                //         self.end_time = None;
                                //         self.is_paused = false;
                                //         self.last_tick = Instant::now();
                                //     }
                        });
                    }
                });
            });

            ctx.request_repaint();
        });
    }
}

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

fn load_svg_texture(ctx: &Context, bytes: &[u8]) -> TextureHandle {
    let svg = load_svg_bytes(bytes).expect("Failed to load SVG");
    ctx.load_texture("svg", svg, Default::default())
}

fn centered_image_button(
    ui: &mut Ui,
    image_id: TextureId,
    image_size: Vec2,
    button_size: Vec2,
    rounding: f32,
    bg_color: Color32,
) -> Response {
    // Выделяем место под кнопку
    let (rect, response) = ui.allocate_exact_size(button_size, Sense::click());

    // Рисуем фон кнопки
    let visuals = if response.hovered() {
        ui.visuals().widgets.hovered
    } else if response.clicked() {
        ui.visuals().widgets.active
    } else {
        ui.visuals().widgets.inactive
    };

    ui.painter().rect(
        rect,
        Rounding::same(rounding),
        bg_color,
        visuals.bg_stroke,
    );

    // Центрируем изображение
    let image_pos = rect.center() - image_size / 2.0;
    let image_rect = egui::Rect::from_min_size(image_pos, image_size);

    ui.painter().image(
        image_id,
        image_rect,
        egui::Rect::from_min_size([0.0, 0.0].into(), [1.0, 1.0].into()),
        Color32::WHITE,
    );

    response
}
