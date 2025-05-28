use eframe::{
    egui::{self},
    Error, NativeOptions,
};
use egui::{
    Align, Align2, Area, CentralPanel, Color32, Context, FontId, Frame, Id, LayerId, Layout,
    Margin, Order, Response, RichText, Rounding, Sense, Stroke, TextStyle, TextureHandle,
    TextureId, Ui, Vec2, ViewportBuilder,
};
use egui_extras::image::load_svg_bytes;
use std::time::{Duration, Instant};

struct MyApp {
    play_image_bytes: &'static [u8],
    pause_image_bytes: &'static [u8],
    stop_image_bytes: &'static [u8],
    brain_image_bytes: &'static [u8],
    cup_image_bytes: &'static [u8],
    dots_image_bytes: &'static [u8],
    status_label: Option<String>,
    left_time_message: String,
    minutes: u64,
    remaining_secs: u64,
    end_time: Option<Instant>,
    is_paused: bool,
    last_tick: Instant,
    show_settings: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            play_image_bytes: include_bytes!("../assets/play.svg"),
            pause_image_bytes: include_bytes!("../assets/pause.svg"),
            stop_image_bytes: include_bytes!("../assets/stop.svg"),
            brain_image_bytes: include_bytes!("../assets/brain.svg"),
            cup_image_bytes: include_bytes!("../assets/cup.svg"),
            dots_image_bytes: include_bytes!("../assets/dots.svg"),
            status_label: None,
            left_time_message: String::from("00\n00"),
            minutes: 25,
            remaining_secs: 0,
            end_time: None,
            is_paused: false,
            last_tick: Instant::now(),
            show_settings: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let is_break = self.is_paused && !self.end_time.is_none();

        let primary_color = if !is_break {
            Color32::from_rgb(255, 124, 124)
        } else {
            Color32::from_rgb(140, 202, 255)
        };
        let second_color = if !is_break {
            Color32::from_rgb(255, 217, 217)
        } else {
            Color32::from_rgb(217, 238, 255)
        };
        let third_color = if !is_break {
            Color32::from_rgb(255, 243, 242)
        } else {
            Color32::from_rgb(242, 249, 255)
        };

        let frame = egui::Frame::none().fill(third_color);

        self.status_label = if self.is_paused {
            Some(String::from("Paused"))
        } else if !self.is_paused && !self.end_time.is_none() {
            Some(String::from("Focus"))
        } else {
            None
        };

        let minutes = self.remaining_secs / 60;
        let seconds = if self.remaining_secs % 60 == 0 {
            String::from("00")
        } else {
            String::from(format!("{}", self.remaining_secs % 60))
        };

        self.left_time_message = if self.remaining_secs > 0 {
            format!("{} \n {}", minutes, seconds)
        } else {
            format!("{} \n {}", self.minutes, seconds)
        };

        show_custom_modal(ctx, &mut self.show_settings, &mut self.minutes, third_color);

        CentralPanel::default().frame(frame).show(ctx, |ui| {
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
                    let texture = if !is_break {
                        load_svg_texture(ctx, self.brain_image_bytes)
                    } else {
                        load_svg_texture(ctx, self.cup_image_bytes)
                    };

                    badge_ui(ui, status, texture.id(), second_color);
                } else {
                    ui.add_space(38.0);
                }

                ui.add_space(30.0);

                ui.label(RichText::new(&self.left_time_message).size(100.0));

                ui.add_space(30.0);

                ui.horizontal(|ui| {
                    ui.add_space(200.0);

                    // settings
                    {
                        let texture = load_svg_texture(ctx, self.dots_image_bytes);
                        let size_icon = Vec2::new(20.0, 20.0);
                        let size_button = Vec2::new(60.0, 50.0);

                        let settings_button = centered_image_button(
                            ui,
                            texture.id(),
                            size_icon,
                            size_button,
                            20.0,
                            second_color,
                        );

                        if (self.is_paused || self.end_time.is_none()) && settings_button.hovered()
                        {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        } else if settings_button.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::NotAllowed);
                        }

                        if (self.is_paused || self.end_time.is_none()) && settings_button.clicked()
                        {
                            self.show_settings = !self.show_settings;
                        }
                    }
                    // play/pause
                    {
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
                            primary_color,
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
                    }
                    // stop
                    {
                        if !self.end_time.is_none() {
                            let stop_texture = load_svg_texture(ctx, self.stop_image_bytes);
                            let size_stop_icon = Vec2::new(30.0, 30.0);
                            let size_button = Vec2::new(55.0, 45.0);

                            let next_stage = centered_image_button(
                                ui,
                                stop_texture.id(),
                                size_stop_icon,
                                size_button,
                                20.0,
                                second_color,
                            );

                            if next_stage.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                            }

                            if next_stage.clicked() {
                                self.remaining_secs = 0;
                                self.end_time = None;
                                self.is_paused = false;
                                self.last_tick = Instant::now();
                            }
                        }
                    }
                });
            });

            ctx.request_repaint();
        });
    }
}

fn main() -> Result<(), Error> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_min_inner_size([600.0, 500.0])
            .with_max_inner_size([600.0, 500.0])
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
    let (rect, response) = ui.allocate_exact_size(button_size, Sense::click());

    let visuals = if response.hovered() {
        ui.visuals().widgets.hovered
    } else if response.clicked() {
        ui.visuals().widgets.active
    } else {
        ui.visuals().widgets.inactive
    };

    ui.painter()
        .rect(rect, Rounding::same(rounding), bg_color, visuals.bg_stroke);

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

fn badge_ui(
    ui: &mut egui::Ui,
    text: &str,
    image_id: egui::TextureId,
    bg_color: egui::Color32,
) -> egui::Response {
    let padding = 12.0;
    let spacing = 2.0;
    let icon_size = egui::vec2(15.0, 15.0);

    let font_id = TextStyle::Body.resolve(ui.style());
    let galley = ui
        .ctx()
        .fonts(|f| f.layout_no_wrap(text.to_string(), font_id.clone(), egui::Color32::WHITE));
    let text_size = galley.size();

    let total_width = padding * 2.0 + icon_size.x + spacing + text_size.x + 20.0;
    let height = f32::max(icon_size.y, text_size.y) + 20.0;
    let badge_size = Vec2::new(total_width, height);

    let (rect, response) = ui.allocate_exact_size(badge_size, egui::Sense::click());

    ui.painter().rect(
        rect,
        Rounding::same(20.0),
        bg_color,
        Stroke::new(1.5, egui::Color32::from_rgb(71, 21, 21)),
    );

    ui.allocate_ui_at_rect(rect, |ui| {
        ui.horizontal_centered(|ui| {
            ui.add_space(padding);
            ui.image((image_id, icon_size));
            ui.label(
                RichText::new(text)
                    .size(15.0)
                    .color(Color32::from_rgb(71, 21, 21)),
            );
        });
    });

    response
}

fn show_custom_modal(
    ctx: &egui::Context,
    is_open: &mut bool,
    minutes: &mut u64,
    background_color: Color32,
) {
    if !*is_open {
        return;
    }

    enable_disabled_layer(ctx);

    let screen_rect = ctx.screen_rect();
    let bg_color = Color32::from_rgba_unmultiplied(0, 0, 0, 128);
    let painter = ctx.layer_painter(LayerId::new(Order::Background, Id::new("modal_bg")));
    painter.rect_filled(screen_rect, 0.0, bg_color);
    let close_icon = include_bytes!("../assets/close.svg");
    let close_icon_texture = load_svg_texture(ctx, close_icon);

    Area::new("custom_modal_window".into())
        .order(Order::Foreground)
        .movable(false)
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, -100.0))
        .show(ctx, |ui| {
            Frame::window(&ctx.style())
                .fill(background_color)
                .stroke(Stroke::new(1.5, Color32::WHITE))
                .rounding(Rounding::same(8.0))
                .inner_margin(Margin::same(10.0))
                .show(ui, |ui| {
                    ui.set_max_width(300.0);

                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Settings")
                                .heading()
                                .color(Color32::from_rgb(71, 21, 21)),
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let close_button = centered_image_button(
                                ui,
                                close_icon_texture.id(),
                                Vec2::new(15.0, 15.0),
                                Vec2::new(20.0, 20.0),
                                20.0,
                                Color32::WHITE,
                            );

                            if close_button.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                            }

                            if close_button.clicked() {
                                *is_open = false;
                            }
                        });
                    });

                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Minutes for focus:").size(18.0));
                        ui.add_space(10.0);
                        ui.add(
                            egui::DragValue::new(minutes)
                                .speed(1.0)
                                .clamp_range(1..=60)
                                .suffix(" min")
                                .custom_formatter(|n, _| format!("{:.0}", n))
                                .prefix("  "),
                        );
                    });
                });
        });
}

fn enable_disabled_layer(ctx: &egui::Context) {
    let screen_rect = ctx.screen_rect();

    ctx.layer_painter(egui::LayerId::new(
        egui::Order::Background,
        egui::Id::new("modal_blocker"),
    ))
    .rect_filled(
        screen_rect,
        0.0,
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 128),
    );

    let _ = egui::Area::new("modal_input_blocker".into())
        .order(egui::Order::Middle)
        .fixed_pos(screen_rect.left_top())
        .show(ctx, |ui| {
            ui.allocate_rect(screen_rect, egui::Sense::click());
        });
}
