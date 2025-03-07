use std::io;

fn main() {
    println!("Clothes Picker!");

    let mut action = String::new();

    io::stdin().read_line(&mut action)
        .expect("Failed to read line");

    let action = action.trim();

    match action {
        "add" => add_item(),
        "remove" => remove_item(),
        "list" => list_items(),
        "get" => get_random_item(),
        _ => println!("Invalid choice"),
    }
}

fn add_item() {
    println!("Adding a new item");
}

fn remove_item() {
    println!("Removing an item");
}

fn list_items() {
    println!("Listing all items");
}

fn get_random_item() {
    println!("Getting a random item");
}
