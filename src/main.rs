fn main() {
    println!("Project Euler!");

    println!("Choose a challenge to run:");

    let mut choice = String::new();

    std::io::stdin()
    .read_line(&mut choice)
    .expect("Failed to read option");

    println!("You chose: {}", choice);

}
