use std::io;
mod dynamo_db;

pub async fn cli_menu() {
    loop {
        println!("Please select an option:");
        println!("1. Create a solar system");
        println!("2. Delete a solar system");
        println!("3. List first 5 solar systems");
        println!("4. Exit");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };

        match option {
            1 => {
                println!("Creating a solar system.....");
                println!("Please enter a name for the solar system");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let name = name.trim();
                dynamo_db::create_new_solar_system(name).await.unwrap();
            },
            2 => {
                println!("Deleting a solar system.....");
                println!("Please enter an id for the solar system");
                let mut id = String::new();
                io::stdin()
                    .read_line(&mut id)
                    .expect("Failed to read line");
                let id = id.trim();
                dynamo_db::delete_solar_system(id).await.unwrap();
            },
            3 => {
                println!("Listing first 5 solar systems.....");
                dynamo_db::list_first_5_items().await.unwrap();
            },
            4 => {
                println!("Exiting.....");
                break;
            },
            _ => {
                println!("Please enter a valid option");
                continue;
            },
        }
    }
}

