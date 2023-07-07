#![windows_subsystem = "windows"]

use eframe::{
    egui::{self, Color32, Vec2},
    epaint::Pos2,
};
use std::{
    io::Error,
    path::Path,
    process::Command,
    thread::{self, JoinHandle},
};

mod filter;

fn filter_spawn(app: &mut MyApp) {
    let _infile = app.infile.to_owned();
    let _cats = app.cats.to_owned();
    let _outfile = app.outfile.to_owned();
    let _cats_column = app.cats_column;

    app.thread_handle = Some(thread::spawn(move || {
        filter::filter_file(_infile, _cats, _cats_column, _outfile)
    }));
}

struct MyApp {
    infile: String,
    cats: String,
    cats_column: usize,
    outfile: String,
    is_done: bool,
    result: Result<bool, Error>,
    thread_handle: Option<JoinHandle<Result<bool, Error>>>,
}

impl MyApp {
    fn new() -> Self {
        Self {
            infile: String::new(),
            cats: "files/cats.txt".to_string(),
            cats_column: 3,
            is_done: false,
            outfile: String::new(),
            result: Result::Ok(true),
            thread_handle: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.thread_handle.is_some() && self.thread_handle.as_ref().unwrap().is_finished() {
                self.is_done = true;

                if let Some(res) = self.thread_handle.take() {
                    self.result = res.join().unwrap()
                }
            }

            let mut style: egui::Style = (*ctx.style()).clone();

            style.spacing.item_spacing = egui::vec2(10.0, 20.0);
            style.visuals.override_text_color = Some(Color32::from_rgb(245, 66, 81));

            ctx.set_style(style);

            ui.heading("Choose file to trim");

            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.infile = path.display().to_string();
                    self.outfile = path.with_extension("").to_string_lossy().to_string()
                        + "_trimmed."
                        + &path.extension().unwrap().to_string_lossy();
                }
            }

            ui.horizontal(|ui| {
                ui.colored_label(Color32::WHITE, format!("Cats file: {}", &self.cats));
                if ui.button("Select cats... (optional)").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.cats = path.display().to_string();
                    }
                }
            });

            ui.add(egui::Slider::new(&mut self.cats_column, 1..=10).text("Category column"))
                .on_hover_text("Pick the column which contains log category in your file");

            ui.colored_label(
                Color32::WHITE,
                format!(
                    "File: {} \nCats: {} \nOutfile: {}",
                    &self.infile, &self.cats, &self.outfile
                ),
            );

            if self.infile.is_empty() || self.outfile.is_empty() {
                ui.colored_label(Color32::RED, "Files are not chosen");
            } else if self.thread_handle.is_some()
                && !self.thread_handle.as_ref().unwrap().is_finished()
            {
                ui.colored_label(Color32::LIGHT_RED, "Processing");
            } else if ui
                .button("Trim!!!")
                .on_hover_text("Perform the trimming operation")
                .clicked()
            {
                self.is_done = false;
                filter_spawn(self);
            }

            match self.result {
                Ok(_) => {
                    if self.is_done {
                        ui.colored_label(Color32::GREEN, "Done");
                        if ui.button("Open directory").clicked() {
                            let path = Path::new(&self.outfile)
                                .parent()
                                .unwrap()
                                .display()
                                .to_string();
                            Command::new("explorer").arg(path).spawn().unwrap();
                        }
                    } else {
                        ui.colored_label(Color32::RED, "Not done/Never used");
                    }
                }
                Err(ref err) => {
                    ui.colored_label(Color32::RED, err.to_string());
                }
            }
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let options = eframe::NativeOptions {
        resizable: true,
        always_on_top: false,
        icon_data: None,
        initial_window_pos: Some(Pos2::new(400.0, 400.0)),
        initial_window_size: Some(Vec2::new(325.0, 270.0)),
        min_window_size: Some(Vec2::new(300.0, 250.0)),
        max_window_size: Some(Vec2::new(900.0, 900.0)),
        decorated: true,
        drag_and_drop_support: false,
        transparent: false,
        maximized: false,
        fullscreen: false,
        vsync: false,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        run_and_return: false,
    };

    eframe::run_native(
        "LogTrimmer",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    );
}
