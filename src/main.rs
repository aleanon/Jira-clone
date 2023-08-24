use std::rc::Rc;
use anyhow::Result;

mod models;

mod db;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() -> Result<()> {
    // TODO: create database and navigator
    let db = db::JiraDatabase::new("./data/db.json".to_string());

    let mut nav = Navigator::new(Rc::new(db));
    
    loop {
        clearscreen::clear().unwrap();

        let page = match nav.get_current_page() {
            Some(page) => page,
            None => break
        };

        if let Err(error) = page.draw_page() {
            println!("Error rendering page: {}\nPress any key to continue...", error);
            wait_for_key_press();
        };

        let input = get_user_input();

        let action = match page.handle_input(input.trim()) {
            Ok(input) => {
                match input {
                    Some(action) => action,
                    None => continue,
                }
            }
            Err(e) => {
                println!("Error handeling input: {}\nPress any key to continue...", e);
                wait_for_key_press();
                continue
            }
        };

        if let Err(e) = nav.handle_action(action) {
            println!("Error handeling action: {}\nPress any key to continue...", e);
            wait_for_key_press();
            continue
        }
    }
    Ok(())
}