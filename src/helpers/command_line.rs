use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

use std::io::{stdin, stdout};

#[derive(Debug, PartialEq)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_msg(&self, agent_role: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        //Choose color based on match
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        //Print agent statement in specific color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {} ", agent_role);

        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        print!("{} ", agent_statement);

        //Reset color
        stdout.execute(ResetColor).unwrap();
    }
}

//Get user response that code is safe to execute
pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        //Print question in specified color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        print!("WARNING: You are about to run code written entirely by AI. ");
        print!("Review your code and confirm that you wish to continue. ");

        //Reset Color
        stdout.execute(ResetColor).unwrap();

        //Present Options with diferent colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] Continue");
        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2] Stop this project");

        // Reset Color
        stdout.execute(ResetColor).unwrap();

        //Read user input
        let mut usr_res: String = String::new();
        stdin()
            .read_line(&mut usr_res)
            .expect("Failed to read user response");

        //Trim whitespace and convert to lowercase
        let usr_res: String = usr_res.trim().to_lowercase();

        //Match response
        match usr_res.as_str() {
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("Invalid input. Please select either '1' or '2'");
            }
        }
    }
}

//get request from user
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    //Print question in specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    //Reset Color
    stdout.execute(ResetColor).unwrap();
    //Read user input
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    return user_response.trim().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_prints_colored_msg() {
        PrintCommand::AICall
            .print_agent_msg("Managing Agent", "Testing somthing, processing something")
    }
}
