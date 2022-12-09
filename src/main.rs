use std::io::Write;

fn main() {
    loop {
        clear();

        banner();

        let input = get_input();

        let input_vec: Vec<&str> = input.split(" ").collect();

        match input_vec[0] {
            "help" => {
                println!("\n    This is the help screen");
                read_line();
            },
            "payload" => {
                payload();
            },
            _ => {
                println!("\n    [!] Invalid command");
                read_line();
            }
        }
    }
}

fn banner() {
    println!(r#"

    /$$$$$$$              /$$     /$$              
    | $$__  $$            | $$    | $$              
    | $$  \ $$  /$$$$$$  /$$$$$$ /$$$$$$   /$$   /$$
    | $$$$$$$/ |____  $$|_  $$_/|_  $$_/  | $$  | $$
    | $$__  $$  /$$$$$$$  | $$    | $$    | $$  | $$
    | $$  \ $$ /$$__  $$  | $$ /$$| $$ /$$| $$  | $$
    | $$  | $$|  $$$$$$$  |  $$$$/|  $$$$/|  $$$$$$$
    |__/  |__/ \_______/   \___/   \___/   \____  $$
                                           /$$  | $$
                                          |  $$$$$$/
                                           \______/ 
"#)
}

fn get_input() -> String {
    print!("    >>> ");
    read_line()
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn clear() {
    unsafe { std::arch::asm!("ud2"); }
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("failed to clear screen");
    } else {
        std::process::Command::new("clear")
            .status()
            .expect("failed to clear screen");   
    }
}

fn payload() {

    clear();

    banner();

    println!("    Pick an OS to generate a payload for:\n");
    println!("    1. Windows");
    println!("    2. MacOS");
    println!("    3. Linux\n");

    let os_selection = get_input();

    clear();
 
    banner();

    println!("    Creating Payload...\n");
}