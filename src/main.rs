use std::io;

fn main() {

    println!("Fibonacci numbers.");

    println!("Enter first number: ");

    let mut n1 = String::new();

    io::stdin()
        .read_line(&mut n1)
        .expect("Failed to read line. ");

    let n1 = n1.trim().parse::<u32>().expect("Error encountered. ");

    println!("Enter second number: ");

    let mut n2 = String::new();

    io::stdin()
        .read_line(&mut n2)
        .expect("Failed to read line. ");

    let n2 = n2.trim().parse::<u32>().expect("Error encountered. ");

    println!("Enter how many times this loop should continue: ");

    let mut final_var = String::new();

    io::stdin()
        .read_line(&mut final_var)
        .expect("Failed to read line. ");

    let final_var = final_var.trim().parse::<u32>().expect("Error encountered. ");

    print!("\n");

    fibonacci(n1, n2, final_var);

}

fn fibonacci(mut n1: u32, mut n2: u32, final_var: u32) {

    println!("{}", n1);
    println!("{}", n2);

    for _x in 1..final_var {
        let n3 = n1 + n2;
        println!("{}", n3);
        n1 = n2;
        n2 = n3;
    }
    
}
