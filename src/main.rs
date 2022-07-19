//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(string_remove_matches)]

pub mod buttons;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("github.com", options, Box::new(|_cc| Box::new(buttons::MyApp::default())));
}

/*
struct MyApp {
    content: String,
    file_name: String
}

impl Default for MyApp {    
    fn default() -> Self {
        Self {
            content: String::from(""),
            file_name: String::from("")
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
                        // Write to blog post file.
                        let mut out = fs::File::create(format!("E:/Programming/Web/git_blog/website/posts/{}.html", self.file_name))
                            .unwrap();
                        write!(out, "<!DOCTYPE html>
<head>
    <link rel='stylesheet' href='../style.css' />
</head>
<body>
    <div id='wrapper'>
        <div id='sidebar'>
            <a href='../index.html'><p>Jon's<br>blog</p></a>
        </div>
        <div id='main-content'>
        {}
        </div>
    </div>
</body>", self.content)
                            .expect("Failed to write to post file");
                    }

                    if ui.button("Update index.html").clicked() {
                        // Update the index.html file.
                        let mut out = fs::File::create("E:/Programming/Web/git_blog/website/index.html").unwrap();
                        let post_paths = fs::read_dir("E:/Programming/Web/git_blog/website/posts").unwrap();
                        let mut stringlist = String::from("<ul>");
                        for post_path in post_paths {
                            stringlist.push_str(format!("<li><a href='posts/{0}'>{0}</a></li>", post_path.unwrap().file_name().into_string().unwrap()).as_str());
                        }
                        stringlist.push_str("</ul>");
                        write!(out, "<!DOCTYPE html>
<head>
    <link rel='stylesheet' href='style.css' />
</head>
<body>
    <div id='wrapper'>
        <div id='sidebar'>
            <a href='index.html'><p>Jon's<br>blog</p></a>
        </div>
        <div id='main-content'>
        {}
        </div>
    </div>
</body>", stringlist)
                        .expect("Failed to write to post file");
                    }
                    
                    if ui.button("Push to the internet!").clicked() {
                        Command::new("cmd")
                            .args(["/C", "bash update.sh"])
                            .output()
                            .expect("Failed to execute command.");
                    }
                });

                //ui.text_edit_multiline(&mut self.content);
                ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.content));
            });
        });
    }
}
*/