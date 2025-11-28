use std::io;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor};

fn set_high(mut register: u16, mut mask: u16) -> u16 {
    clear_terminal();
    println!("{:#018b} | {:#06X} | {}\n", register, register, register);
    loop {
        let bit_index: u32 = input("Set [High] | Bit Index [0 - 15]: ").parse().expect("Expected An Integer: u32");
        if bit_index > 15 {
            clear_terminal();
            println!("{} -> [{}]\n", "Invalid Index", bit_index);
            println!("{:#018b} | {:#06X} | {}\n", register, register, register);
            continue;
        } else {
            mask = mask << bit_index;
            register = register | mask;
            break register;
        }
    }
}

fn set_low(mut register: u16, mut mask: u16) -> u16 {
    clear_terminal();
    println!("{:#018b} | {:#06X} | {}\n", register, register, register);
    loop {
        let bit_index: u32 = input("Set [Low] | Bit Index [0 - 15]: ").parse().expect("Expected An Integer: u32");
        if bit_index > 15 {
            clear_terminal();
            println!("{} -> [{}]\n", "Invalid Index", bit_index);
            println!("{:#018b} | {:#06X} | {}\n", register, register, register);
            continue;
        } else {
            mask = mask << bit_index;
            register = register & !mask;
            break register;
        }
    }
}

fn toggle(mut register: u16, mut mask: u16) -> u16 {
    clear_terminal();
    println!("{:#018b} | {:#06X} | {}\n", register, register, register);
    loop {
        let bit_index: u32 = input("Toggle | Bit Index [0 - 15]: ").parse().expect("Expected An Integer: u32");
        if bit_index > 15 {
            clear_terminal();
            println!("{} -> [{}]\n", "Invalid Index", bit_index);
            println!("{:#018b} | {:#06X} | {}\n", register, register, register);
            continue;
        } else {
            mask = mask << bit_index;
            register = register ^ mask;
            break register;
        }
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.trim().to_string()
}

fn clear_terminal() {
    execute!(
        io::stdout(), 
        Clear(ClearType::FromCursorUp),
        cursor::MoveTo(0, 0)
    ).unwrap();
}

fn options(prompt: &str, register: u16) -> String {
    loop {
        let choice: String = input(prompt);
        if (choice == "t") | (choice == "T") {
            break "toogle".to_string();
        } else if (choice == "h") | (choice == "H") {
            break "set high".to_string();
        } else if (choice == "l") | (choice == "L") {
            break "set low".to_string();
        } else {
            clear_terminal();
            println!("{} -> [{}]\n", "Invalid Operation", choice);
            println!("{:#018b} | {:#06X} | {}\n", register, register, register);
            continue;
        }
    }
}

fn main() {
    let mut register: u16 = 0x0000;
    let mask: u16 = 0x0001;

    clear_terminal();
    println!("{:#018b} | {:#06X} | {}\n", register, register, register);
    loop {
        let choice: String = options(
            "[Options]:\n  Toggle: T\n  Set High: H\n  Set Low: L", 
            register
        );
        register = {
            if choice == "toogle" {
                toggle(register, mask)
            } else if choice == "set high" {
                set_high(register, mask)
            } else if choice == "set low" {
                set_low(register, mask)
            } else {
                register
            }
        };
        clear_terminal();
        println!("{:#018b} | {:#06X} | {}\n", register, register, register);
    }
}
