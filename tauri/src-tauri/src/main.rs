// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use tauri::{Manager, State};
use tauri_plugin_dialog::DialogExt;
mod proc;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use proc::proc::*;
mod my_def;
use my_def::constants::*;

struct ProcessorState {
    processor: Mutex<Processor>,
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn file_test(file: &str) -> String {
    println!("file: {}", file);
    file.to_string()
}

#[tauri::command]
fn clock_processor(state: State<'_, Arc<ProcessorState>>) -> String {
    let mut processor = state.processor.lock().unwrap();
    processor.clock();
    let new_state = processor.get_state_serialized();
    new_state
}

#[tauri::command]
fn get_state(state: State<'_, Arc<ProcessorState>>) -> String {
    let processor = state.processor.lock().unwrap();
    processor.get_state_serialized()
}

#[tauri::command]
fn set_num_representation(state: State<'_, Arc<ProcessorState>>, representation: String) {
    let mut processor = state.processor.lock().unwrap();
    processor.set_num_rep(representation);
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let file_path = app
                .dialog()
                .file()
                .blocking_pick_file()
                .expect("Failed to open file")
                .into_path()
                .unwrap();

            // let file_path = PathBuf::from("C:\\Git Repositories\\proc_simulator\\src\\rom.dat");

            let proc = Processor::new(file_path, "hex".to_string());

            let processor_state = ProcessorState {
                processor: Mutex::new(proc),
            };

            app.manage(Arc::new(processor_state)); // Correctly manage the ProcessorState

            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            file_test,
            clock_processor,
            get_state,
            set_num_representation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
