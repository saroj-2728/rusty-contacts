use std::process::Command;
use std::{
    io::{self, Write},
    vec,
};

#[derive(Debug)]
struct Contact {
    name: String,
    phone: String,
    email: Option<String>,
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn pause() {
    print!("(Press Enter to continue)");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}

fn main() {
    let mut contacts: Vec<Contact> = vec![];

    loop {
        display_menu();
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                pause();
                clear_terminal();
                continue;
            }
        };

        match choice {
            1 => add_contact(&mut contacts),
            2 => view_contacts(&contacts),
            3 => search_contact(&contacts),
            4 => delete_contact(&mut contacts),
            5 => break,
            _ => {
                println!("Please enter a valid number!");
                pause();
                clear_terminal();
            }
        }

        clear_terminal();
    }
}

fn display_menu() {
    println!("------------------------------------");
    println!("|          RUSTY-CONTACTS          |");
    println!("------------------------------------");

    println!("Options: ");
    println!(
        "1. Add contact \n2. View contacts \n3. Search contact \n4. Delete contact \n5. Exit \n"
    );

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

fn add_contact(contacts: &mut Vec<Contact>) {
    let mut name = String::new();
    let mut phone = String::new();
    let mut email = String::new();

    println!("\nEnter details: ");

    print!("Name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("Please enter a valid name");

    print!("Phone: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut phone)
        .expect("Please enter a valid name");

    print!("Email: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut email)
        .expect("Please enter a valid name");

    let contact = Contact {
        name: name.trim().to_string(),
        phone: phone.trim().to_string(),
        email: {
            let trimmed_email = email.trim();

            if trimmed_email.is_empty() {
                None
            } else {
                Some(trimmed_email.to_string())
            }
        },
    };

    contacts.push(contact);
    println!("\nContact added successfully\n");

    pause();
}

fn view_contacts(contacts: &Vec<Contact>) {
    println!("\nAll Contacts:");
    for (index, contact) in contacts.iter().enumerate() {
        println!(
            "{}. Name: {} | Phone: {} | Email: {}",
            index,
            contact.name,
            contact.phone,
            match &contact.email {
                None => String::from("Not Set"),
                Some(email) => email.to_string(),
            }
        )
    }

    println!();
    pause();
}

fn search_contact(contacts: &Vec<Contact>) {
    let mut search_query = String::new();

    print!("\nEnter the name or partial name to search: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut search_query)
        .expect("Failed to read line!");

    println!();

    let search_query = search_query.trim().to_lowercase();
    let mut found = false;

    println!("Matching contacts: ");
    for contact in contacts {
        if contact.name.to_lowercase().contains(&search_query) {
            println!(
                "Name: {} | Phone: {} | Email: {}",
                contact.name,
                contact.phone,
                match &contact.email {
                    None => String::from("Not Set"),
                    Some(email) => email.to_string(),
                }
            );
            found = true;
        }
    }

    if !found {
        println!("No contact found matching '{}'", search_query);
    }

    println!();
    pause();
}

fn delete_contact(contacts: &mut Vec<Contact>) {
    view_contacts(contacts);

    print!("Please enter the index of the contact to delete: ");
    io::stdout().flush().unwrap();

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: Option<usize> = match index.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("\nPlease enter a number!");
            None
        }
    };

    match index {
        None => println!("An error occured!"),
        Some(num) => {
            if num > contacts.len() - 1 {
                println!("\nInvalid index!! Please try again.");
            } else {
                contacts.remove(num);
                println!("\nContact deleted successfully!");
            }
        }
    }

    pause();
}
