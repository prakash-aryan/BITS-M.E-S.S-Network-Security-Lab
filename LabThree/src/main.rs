use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 700.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "Cipher Application",
        options,
        Box::new(|cc| Box::new(CipherApp::new(cc))),
    )
}

struct CipherApp {
    plaintext: String,
    key: String,
    ciphertext: String,
    cipher_type: CipherType,
    matrix: Vec<Vec<char>>,
}

#[derive(PartialEq, Clone, Copy)]
enum CipherType {
    Additive,
    RailFence,
}

impl CipherApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize text styles
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(28.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Button, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
        ].into();
        cc.egui_ctx.set_style(style);

        Self::default()
    }

    fn reset(&mut self) {
        self.plaintext.clear();
        self.key.clear();
        self.ciphertext.clear();
        self.matrix.clear();
    }
}

impl Default for CipherApp {
    fn default() -> Self {
        Self {
            plaintext: String::new(),
            key: String::new(),
            ciphertext: String::new(),
            cipher_type: CipherType::Additive,
            matrix: Vec::new(),
        }
    }
}

impl eframe::App for CipherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Cipher Application");
            });
            
            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.label("Cipher Type:");
                ui.radio_value(&mut self.cipher_type, CipherType::Additive, "Additive");
                ui.radio_value(&mut self.cipher_type, CipherType::RailFence, "Rail Fence");
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label("Plaintext:");
                ui.add(egui::TextEdit::multiline(&mut self.plaintext).desired_rows(3));
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label("Key:");
                ui.add(egui::TextEdit::singleline(&mut self.key));
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button("Encrypt").clicked() {
                    self.ciphertext = match self.cipher_type {
                        CipherType::Additive => additive_cipher(&self.plaintext, self.key.parse().unwrap_or(0)),
                        CipherType::RailFence => {
                            let rail_fence = RailFence::new(self.key.parse().unwrap_or(2));
                            self.matrix = rail_fence.encode_with_matrix(&self.plaintext);
                            rail_fence.encode(&self.plaintext)
                        },
                    };
                }
                if ui.button("Reset").clicked() {
                    self.reset();
                }
            });

            ui.add_space(20.0);

            ui.group(|ui| {
                ui.label("Ciphertext:");
                ui.add(egui::TextEdit::multiline(&mut self.ciphertext).desired_rows(3));
            });

            if self.cipher_type == CipherType::RailFence && !self.matrix.is_empty() {
                ui.add_space(20.0);
                ui.label("Matrix Visualization:");
                let cell_size = egui::vec2(25.0, 25.0);
                let grid_width = self.plaintext.len() as f32 * cell_size.x;
                let grid_height = self.matrix.len() as f32 * cell_size.y;

                egui::ScrollArea::horizontal().show(ui, |ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        let (response, painter) = ui.allocate_painter(egui::vec2(grid_width.max(ui.available_width()), grid_height), egui::Sense::hover());
                        let rect = response.rect;

                        // Draw grid cells
                        for (row, rail) in self.matrix.iter().enumerate() {
                            for (col, &ch) in rail.iter().enumerate() {
                                let cell_rect = egui::Rect::from_min_size(
                                    rect.left_top() + egui::vec2(col as f32 * cell_size.x, row as f32 * cell_size.y),
                                    cell_size,
                                );
                                painter.rect_stroke(cell_rect, 0.0, ui.style().visuals.widgets.noninteractive.bg_stroke);
                                
                                if ch != ' ' {
                                    painter.text(
                                        cell_rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        ch.to_string(),
                                        egui::TextStyle::Body.resolve(ui.style()),
                                        ui.style().visuals.text_color(),
                                    );
                                }
                            }
                        }
                    });
                });
            }
        });
    }
}

pub fn additive_cipher(plaintext: &str, key: i32) -> String {
    plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let offset = ((c as u8 - base) as i32 + key).rem_euclid(26) as u8;
                (base + offset) as char
            } else {
                c
            }
        })
        .collect()
}

pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut fence = vec![Vec::new(); self.rails];
        for (c, i) in text.chars().zip(self.zigzag()) {
            fence[i].push(c);
        }
        fence.into_iter().flatten().collect()
    }

    pub fn encode_with_matrix(&self, text: &str) -> Vec<Vec<char>> {
        let mut fence = vec![vec![' '; text.len()]; self.rails];
        for (idx, (c, i)) in text.chars().zip(self.zigzag()).enumerate() {
            fence[i][idx] = c;
        }
        fence
    }

    fn zigzag(&self) -> impl Iterator<Item = usize> {
        (0..self.rails - 1).chain((1..self.rails).rev()).cycle()
    }
}