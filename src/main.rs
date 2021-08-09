mod problem_002;

fn main() {
    println!("Project Euler!");

    loop {
        let mut choice = String::new();

        println!("Choose a challenge to run:");
        println!("2: Even Fibonacci numbers");
        println!("q: Quit");

        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read option");

        match choice.trim() {
            "q" => break,
            "2" => {
                problem_002::run();
                break;
            }
            _ => println!("Unknown option: {}", choice),
        }
    }
}
