use eframe::{egui, App, Frame, NativeOptions, run_native};
use rfd::FileDialog;
use std::fs;

use crate::crypto;

pub fn launch_gui() {
    run_native(
        "Secure File Encryptor",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(EncryptorApp::default()))),
    )
    .expect("Failed to launch GUI");
}

#[derive(Default)]
struct EncryptorApp {
    input_path: String,
    output_path: String,
    password: String,
    status: String,
}

impl App for EncryptorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Secure File Encryptor");
            ui.separator();

            if ui.button("📁 Select Input File").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.input_path = path.display().to_string();
                }
            }
            ui.label(format!("Input: {}", self.input_path));

            if ui.button("💾 Select Output File").clicked() {
                if let Some(path) = FileDialog::new().save_file() {
                    self.output_path = path.display().to_string();
                }
            }
            ui.label(format!("Output: {}", self.output_path));

            ui.horizontal(|ui| {
                ui.label("🔑 Password:");
                ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
            });

            ui.horizontal(|ui| {
                if ui.button("🔒 Encrypt").clicked() {
                    self.status = encrypt_action(&self.input_path, &self.output_path, &self.password);
                }
                if ui.button("🔓 Decrypt").clicked() {
                    self.status = decrypt_action(&self.input_path, &self.output_path, &self.password);
                }
            });

            ui.separator();
            ui.label(format!("Status: {}", self.status));
        });
    }
}

fn encrypt_action(input: &str, output: &str, password: &str) -> String {
    match fs::read(input) {
        Ok(data) => {
            let encrypted = crypto::encrypt_data(&data, password);
            match fs::write(output, encrypted) {
                Ok(_) => "Encryption successful!".to_string(),
                Err(e) => format!("Failed to write output: {}", e),
            }
        }
        Err(e) => format!("Failed to read input: {}", e),
    }
}

fn decrypt_action(input: &str, output: &str, password: &str) -> String {
    match fs::read(input) {
        Ok(data) => match crypto::decrypt_data(&data, password) {
            Ok(decrypted) => match fs::write(output, decrypted) {
                Ok(_) => "Decryption successful!".to_string(),
                Err(e) => format!("Failed to write output: {}", e),
            },
            Err(e) => format!("Decryption failed: {}", e),
        },
        Err(e) => format!("Failed to read input: {}", e),
    }
}