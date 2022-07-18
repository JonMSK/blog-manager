/*
ui.heading("Heading");
ui.horizontal(|ui| {
    ui.label("Label");
    ui.text_edit_singleline(&mut self.name);
});
    ui.add(egui::Slider::new(&mut self.age, 0..=120).text("Slider"));
    if ui.button("Button").clicked() {
        self.age += 1;
    }
ui.label(format!("Hello '{}', age {}", self.name, self.age));
*/