use std::io;

fn main() {
    println!("Clothes Picker!");
    println!("Insert add to add an item\nInsert remove to remove an item\nInsert list to see all your items\nInsert get to get an outfit today");

    let mut action = String::new();

    io::stdin().read_line(&mut action).unwrap();

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
