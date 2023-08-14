// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use g_rust::extension::extension::{Extension, ExtensionInfo};
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::thread;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_json() -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open("data.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, Default)]
struct Test {}

#[derive(Debug, Serialize, Deserialize)]
struct WardrobeItem {
    gender: String,
    figure: String,
}

fn main() {
    // Criar um canal
    let (sender, receiver) = mpsc::channel();

    // Criar uma nova thread/
    let handle = thread::spawn(move || {
        let mut ext: Extension<Test> = Extension::new();
        ext.info = ExtensionInfo {
            name: "Extended Wardrobe".to_string(),
            description: "Save actual clothes to a external wardrobe without limit (:ew)"
                .to_string(),
            author: "!K2".to_string(),
            version: "0.1.0".to_string(),
        };

        ext.run();
        // Enviar uma mensagem para o canal
        sender.send("Olá da thread 2").unwrap();
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // Receber a mensagem da thread 2
    let received_msg = receiver.recv().unwrap();
    println!("Mensagem recebida: {}", received_msg);

    handle.join().unwrap();
}
