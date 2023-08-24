use crate::{models::{Epic, Story, Status}, io_utils::get_user_input};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>
}

impl Prompts {
    pub fn new() -> Self {
        Self { 
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt)
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("----------------------------");
    println!("Epic Name:");
    
    let name = get_user_input();

    println!("Epic description:");

    let description = get_user_input();

    Epic::new(name.trim().to_owned(), description.trim().to_owned())
}

fn create_story_prompt() -> Story {
    println!("----------------------------------------");
    println!("Story Name:");

    let name = get_user_input();

    println!("Story description:");

    let description = get_user_input();

    Story::new(name.trim().to_owned(), description.trim().to_owned())
}

fn delete_epic_prompt() -> bool {
    loop {
        println!("----------------------------");
        println!("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]:");

        let answer = get_user_input();
        
        match answer.trim() {
            "y" | "Y" => return true,
            "n" | "N" => return false,
            _ => { 
                println!("Invalid input");
                continue
            }
        }
    }

}

fn delete_story_prompt() -> bool {
    loop {
        println!("----------------------------");
        println!("Are you sure you want to delete this story? [Y/n]:");

        let answer = get_user_input();
        
        match answer.trim() {
            "y" | "Y" => return true,
            "n" | "N" => return false,
            _ => { 
                println!("Invalid input");
                continue
            }
        }
    }
}

fn update_status_prompt() -> Option<Status> {
    println!("----------------------------");
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED):");

    let answer = get_user_input();

    match answer.trim() {
        "1" => Some(Status::Open),
        "2" => Some(Status::InProgress),
        "3" => Some(Status::Resolved),
        "4" => Some(Status::Closed),
        _ => None
    }
}