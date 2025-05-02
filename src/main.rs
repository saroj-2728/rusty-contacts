use std::io;

#[derive(Debug)]
struct Contact {
    name: String,
    phone: String,
    email: Option<String>,
}

fn main() {
    display_menu();
    add_contact();

    loop {
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        match choice {
            1 => some_fn(),
            2 => some_fn(),
            3 => some_fn(),
            4 => some_fn(),
            5 => break,
            _ => {
                println!("Please enter a valid number!");
            }
        }
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

    println!("Enter your choice: ");
}

fn some_fn() {
    return;
}

fn add_contact() {
    let mut contact = Contact {
        name: String::new(),
        phone: String::new(),
        email: Some(String::new())
    };
    
    println!("Enter details: ");

    print!("Name: ");
    io::stdin()
        .read_line(&mut contact.name)
        .expect("Please enter a valid name");
    contact.name = String::from(contact.name.trim());

    print!("Phone: ");
    io::stdin()
        .read_line(&mut contact.phone)
        .expect("Please enter a valid name");
    contact.phone = String::from(contact.phone.trim());

    let mut temp_email = String::new();
    print!("Email: ");
    io::stdin()
        .read_line(&mut temp_email)
        .expect("Please enter a valid name");
    temp_email = String::from(temp_email.trim());

    if temp_email != "" {
        contact.email = Some(temp_email);
    }
    else {
        contact.email = None
    }

    println!("Contact added: {:#?}", contact);
}
