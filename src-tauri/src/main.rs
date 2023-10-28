// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;
use evalexpr::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_answer(answer: &str, question: &str) -> (String, bool) {
    let x: f64 = eval_float(question).unwrap();
    if answer == x.to_string() {
        (format!("{} is correct! Good job!", answer), true)
    }
    else {
        (format!("{} is incorrect! Try again.", answer), false)
    }
}

#[tauri::command]
fn skip_question(question: &str, range: String) -> (f64, String) {
    let x: f64 = eval_float(question).unwrap();
    let y = get_new_question(range);
    (x, y)
}

#[tauri::command]
fn get_new_question(range: String) -> String {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    let mut range_num: i64 = eval(&range).unwrap().as_int().unwrap();

    range_num = range_num.clamp(2, 1000000000);

    let num1: i64 = rng.gen_range(-range_num..range_num);
    let mut num2: i64 = rng.gen_range(-range_num..range_num);

    let operand_array = ["+", "-", "*", "/"];

    let operand: usize = rng.gen_range(0..operand_array.len());

    if operand_array[operand] == "/" && num2 == 0 {
        num2 += rng.gen_range(1..range_num);
    }
    
    format!("{}.0 {} {}.0", num1, operand_array[operand], num2)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_answer, get_new_question, skip_question])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
