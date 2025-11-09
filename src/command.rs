pub mod command
{
    const ERROR_MESSAGE: &str = "Invalid query";
    pub fn parse_user_input(user_input: &str) {
        if user_input.is_empty() {
            eprintln!("{}", ERROR_MESSAGE);
        }
        // Tokenise input
        let tokenized_command = user_input.split_whitespace().collect::<Vec<_>>();
        match tokenized_command.first() {
            Some(x) => match x.to_lowercase().as_str() {
                "load" => parse_load_command(&tokenized_command),
                "list" => parse_list_command(&tokenized_command),
                _ => eprintln!("{}", ERROR_MESSAGE),
            }
            None => eprintln!("{}", ERROR_MESSAGE),
        }
    }

    fn parse_load_command(user_input: &Vec<&str>) {
        assert_eq!(user_input.first().unwrap().to_lowercase().as_str(), "load");
        if user_input.len() == 1 {
            eprintln!("Invalid file path");
            return;
        }
        let file_path = &user_input[1];
        if std::path::Path::new(file_path).exists() == false {
            eprintln!("Invalid file path");
            return;
        }
    }

    fn parse_list_command(user_input: &Vec<&str>) {
        assert_eq!(user_input.first().unwrap().to_lowercase().as_str(), "list");
        println!("Listing...");
    }
}