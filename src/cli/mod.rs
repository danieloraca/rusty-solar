use std::env;
mod dynamo_db;

pub async fn cli() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No arguments passed.....");
        dynamo_db::list_first_5_items().await.unwrap();
        return;
    }

    if args[1] == "create-solar-system" {
        println!("Creating a solar system.....");
        if args.len() == 2 {
            println!("No name provided for solar system");
            return;
        }

        let name = &args[2];
        dynamo_db::create_new_solar_system(name).await.unwrap();
    }

    if args[1] == "delete-solar-system" {
        println!("Deleting a solar system.....");
        if args.len() == 2 {
            println!("No id provided for solar system");
            return;
        }

        let id = &args[2];
        dynamo_db::delete_solar_system(id).await.unwrap();
    }

    if args[1] == "help" {
        println!("Help");
    } else if args[1] == "version" {
        println!("Version");
    } else {
        println!("No known arguments passed");
    }

    println!("{:?}", args);
}

