// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;
use evalexpr::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_answer(answer: &str, question: &str) -> (String, bool) {
    let x = eval_int(question).unwrap();
    if answer == x.to_string() {
        (format!("{} is correct! Good job!", answer), true)
    }
    else {
        (format!("{} is incorrect! Try again.", answer), false)
    }
}

#[tauri::command]
fn get_new_question(range: String) -> String {
    let mut rng = rand::thread_rng();

    let range_num = eval(&range).unwrap().as_int().unwrap();

    let num1: i64 = rng.gen_range(-range_num..range_num);
    let mut num2: i64 = rng.gen_range(-range_num..range_num);
    let operand_array = ["+", "-", "*", "/"];

    let operand: usize = rng.gen_range(0..operand_array.len());

    if operand == 3 && num2 == 0 {
        num2 += 1;
    }
    
    format!("{} {} {}", num1, operand_array[operand], num2)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_answer, get_new_question])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
