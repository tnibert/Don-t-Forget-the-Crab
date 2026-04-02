pub enum Command {
    ListRecipes,
    CreateShoppingList(Vec<String>),
    Exit
}

pub fn parse_args() -> Command {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        return Command::Exit
    }
    if args.contains(&String::from("--list")) {
        return Command::ListRecipes;
    } else {
        // pop application name off arg list
        let slice = &args[1..args.len()];
        return Command::CreateShoppingList(slice.to_vec());
    }
}
