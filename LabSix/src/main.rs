use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 1000.0)),
        resizable: true,
        ..Default::default()
    };
    eframe::run_native(
        "RC4 Cipher Application",
        options,
        Box::new(|cc| Box::new(RC4App::new(cc))),
    )
}

struct RC4App {
    n: usize,
    plaintext: String,
    key: String,
    ciphertext: String,
    decrypted: String,
    state_vector: Vec<u8>,
    plaintext_vector: Vec<u8>,
    key_vector: Vec<u8>,
    key_stream: Vec<u8>,
    ksa_steps: String,
    prga_steps: String,
    is_binary_mode: bool,
}

impl RC4App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
        self.n = 1;
        self.plaintext.clear();
        self.key.clear();
        self.ciphertext.clear();
        self.decrypted.clear();
        self.state_vector.clear();
        self.plaintext_vector.clear();
        self.key_vector.clear();
        self.key_stream.clear();
        self.ksa_steps.clear();
        self.prga_steps.clear();
    }

    fn binary_to_decimal(binary: &str) -> u8 {
        u8::from_str_radix(binary, 2).unwrap_or(0)
    }

    fn decimal_to_binary(decimal: u8, width: usize) -> String {
        format!("{:0width$b}", decimal, width = width)
    }

    fn ksa(&mut self) {
        if self.is_binary_mode {
            self.state_vector = (0..2u32.pow(self.n as u32)).map(|i| i as u8).collect();
        } else {
            self.state_vector = (0..256).map(|i| i as u8).collect();
        }
        let mut j: usize = 0;
        for i in 0..self.state_vector.len() {
            j = (j + self.state_vector[i] as usize + self.key_vector[i % self.key_vector.len()] as usize) % self.state_vector.len();
            self.state_vector.swap(i, j);
            self.ksa_steps.push_str(&format!("{}  {:?}\n", i, self.state_vector));
        }
        self.ksa_steps.push_str(&format!("\nThe initial permutation array is : {:?}\n", self.state_vector));
    }

    fn prga(&mut self) {
        let mut i = 0;
        let mut j = 0;
        self.key_stream.clear();
        for k in 0..self.plaintext_vector.len() {
            i = (i + 1) % self.state_vector.len();
            j = (j + self.state_vector[i] as usize) % self.state_vector.len();
            self.state_vector.swap(i, j);
            let t = (self.state_vector[i] as usize + self.state_vector[j] as usize) % self.state_vector.len();
            self.key_stream.push(self.state_vector[t]);
            self.prga_steps.push_str(&format!("{}  {:?}\n", k, self.state_vector));
        }
        self.prga_steps.push_str(&format!("Key stream : {:?}\n", self.key_stream));
    }

    fn xor(&self, text: &[u8], key_stream: &[u8]) -> Vec<u8> {
        text.iter().zip(key_stream.iter()).map(|(&t, &k)| t ^ k).collect()
    }

    fn encrypt(&mut self) {
        if self.is_binary_mode {
            self.plaintext_vector = self.plaintext.as_bytes()
                .chunks(self.n)
                .map(|chunk| Self::binary_to_decimal(std::str::from_utf8(chunk).unwrap_or("0")))
                .collect();
            self.key_vector = self.key.as_bytes()
                .chunks(self.n)
                .map(|chunk| Self::binary_to_decimal(std::str::from_utf8(chunk).unwrap_or("0")))
                .collect();
        } else {
            self.plaintext_vector = self.plaintext.as_bytes().to_vec();
            self.key_vector = self.key.as_bytes().to_vec();
        }

        self.ksa_steps.clear();
        self.prga_steps.clear();

        self.ksa();
        self.prga();

        let encrypted = self.xor(&self.plaintext_vector, &self.key_stream);
        if self.is_binary_mode {
            self.ciphertext = encrypted.iter()
                .map(|&b| Self::decimal_to_binary(b, self.n))
                .collect::<Vec<String>>()
                .join("");
        } else {
            self.ciphertext = encrypted.iter()
                .map(|&b| format!("{:02x}", b))
                .collect::<String>();
        }
    }

    fn decrypt(&mut self) {
        let ciphertext_bytes: Vec<u8> = if self.is_binary_mode {
            self.ciphertext.as_bytes()
                .chunks(self.n)
                .map(|chunk| Self::binary_to_decimal(std::str::from_utf8(chunk).unwrap_or("0")))
                .collect()
        } else {
            self.ciphertext.chars()
                .collect::<Vec<char>>()
                .chunks(2)
                .map(|chunk| u8::from_str_radix(&chunk.iter().collect::<String>(), 16).unwrap_or(0))
                .collect()
        };

        if self.is_binary_mode {
            self.key_vector = self.key.as_bytes()
                .chunks(self.n)
                .map(|chunk| Self::binary_to_decimal(std::str::from_utf8(chunk).unwrap_or("0")))
                .collect();
        } else {
            self.key_vector = self.key.as_bytes().to_vec();
        }

        self.ksa_steps.clear();
        self.prga_steps.clear();

        self.ksa();
        self.prga();

        let decrypted = self.xor(&ciphertext_bytes, &self.key_stream);
        if self.is_binary_mode {
            self.decrypted = decrypted.iter()
                .map(|&b| Self::decimal_to_binary(b, self.n))
                .collect::<Vec<String>>()
                .join("");
        } else {
            self.decrypted = String::from_utf8_lossy(&decrypted).to_string();
        }
    }
}

impl Default for RC4App {
    fn default() -> Self {
        Self {
            n: 1,
            plaintext: String::new(),
            key: String::new(),
            ciphertext: String::new(),
            decrypted: String::new(),
            state_vector: Vec::new(),
            plaintext_vector: Vec::new(),
            key_vector: Vec::new(),
            key_stream: Vec::new(),
            ksa_steps: String::new(),
            prga_steps: String::new(),
            is_binary_mode: true,
        }
    }
}

impl eframe::App for RC4App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("RC4 Cipher Application");
                });
                
                ui.add_space(20.0);

                ui.checkbox(&mut self.is_binary_mode, "Binary Mode");

                ui.add_space(10.0);

                if self.is_binary_mode {
                    ui.group(|ui| {
                        ui.label("n (Plaintext and Key length will be a multiple of n bits):");
                        if ui.add(egui::DragValue::new(&mut self.n).speed(0.1).clamp_range(1..=8)).changed() {
                            self.n = self.n.max(1);
                        }
                    });
                }

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label(if self.is_binary_mode {
                        format!("Plaintext (binary, multiple of {} bits):", self.n)
                    } else {
                        "Plaintext (string):".to_string()
                    });
                    ui.text_edit_multiline(&mut self.plaintext);
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label(if self.is_binary_mode {
                        format!("Key (binary, multiple of {} bits):", self.n)
                    } else {
                        "Key (string):".to_string()
                    });
                    ui.text_edit_multiline(&mut self.key);
                });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Encrypt").clicked() {
                        self.encrypt();
                    }
                    if ui.button("Decrypt").clicked() {
                        self.decrypt();
                    }
                    if ui.button("Reset").clicked() {
                        self.reset();
                    }
                });

                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.label(if self.is_binary_mode {
                        "Ciphertext (binary):".to_string()
                    } else {
                        "Ciphertext (hex):".to_string()
                    });
                    ui.text_edit_multiline(&mut self.ciphertext);
                });

                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.label(if self.is_binary_mode {
                        "Decrypted text (binary):".to_string()
                    } else {
                        "Decrypted text (string):".to_string()
                    });
                    ui.text_edit_multiline(&mut self.decrypted);
                });

                ui.add_space(20.0);

                ui.collapsing("KSA Steps", |ui| {
                    ui.label(&self.ksa_steps);
                });

                ui.collapsing("PRGA Steps", |ui| {
                    ui.label(&self.prga_steps);
                });

                ui.add_space(20.0);

                ui.label("Plaintext Vector:");
                ui.label(format!("{:?}", self.plaintext_vector));

                ui.add_space(10.0);

                ui.label("Key Vector:");
                ui.label(format!("{:?}", self.key_vector));
            });
        });
    }
}