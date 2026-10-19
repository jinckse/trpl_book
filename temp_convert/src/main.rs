use std::io;

fn main() {
    loop {
        // Ask for user input
        println!("Enter a temperature in Fahrenheit: ");
        let mut f_temp = String::new();
        io::stdin()
            .read_line(&mut f_temp)
            .expect("Failed to read line");

        // Check that user input is valid
        let f_temp: f32 = match f_temp.trim().parse() {
            //Err(_) => println!("Temperature entered was not a number"),
            Ok(num) => num,
            Err(_) => continue,
            };

        // If valid perform conversion
        let c_temp: f32 = (f_temp - 32.0) * (5.0 / 9.0);

        // Display converted input temp
        println!("Entered temp {f_temp}F is {:.0}C", c_temp);
    }
}
