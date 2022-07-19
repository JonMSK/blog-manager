use std::fs;
use eframe::egui;
use std::process::Command;

pub mod functions;

pub struct MyApp {
    content: String,
    file_name: String,
    delete_file : bool
}

impl Default for MyApp {    
    fn default() -> Self {
        Self {
            content: String::from(""),
            file_name: String::from(""),
            delete_file: false
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("New blog post");

                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.file_name);
                    if ui.button("Save to file").clicked() {
                        functions::save_post(self);
                    }
                    if ui.button("Load file").clicked() {
                        self.content = fs::read_to_string(format!("E:/Programming/Web/git_blog/website/posts/{}.html", self.file_name)).unwrap();
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("Update index.html").clicked() {
                        functions::update_index();
                    }
                    ui.checkbox(&mut self.delete_file, "");
                    if ui.button("Delete post").clicked() && self.delete_file {
                        fs::remove_file(format!("E:/Programming/Web/git_blog/website/posts/{}.html", self.file_name))
                            .expect("Failed to delete file");
                    }
                    if ui.button("Update GitHub").clicked() {
                        functions::update_index();
                        Command::new("cmd")
                            .args(["/C", "bash update.sh"])
                            .output()
                            .expect("failed to execute process");
                    }
                });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.content));
                });
            });
        });
    }
}