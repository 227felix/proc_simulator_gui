// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use tauri::State;

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

fn main() {
    let file_path = tauri::api::dialog::blocking::FileDialogBuilder::new()
        .pick_file()
        .expect("No file selected");

    let proc = Processor::new(file_path);

    let processor_state = ProcessorState {
        processor: Mutex::new(proc),
    };

    tauri::Builder::default()
        .manage(Arc::new(processor_state)) // Correctly manage the ProcessorState
        .invoke_handler(tauri::generate_handler![greet, file_test, clock_processor])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
