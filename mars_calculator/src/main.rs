use std::io;

fn main() {
    let mut input = String::new();

    print!("Enter your Earth weight in kg");
    io::stdin().read_line(&mut input).unwrap();

    //convert string to float??
        // remove all whitespace and new line things from beginning and end of line
        // do with trim function
    let weight = input.trim().parse::<f32>().unwrap();
                                            // unwrap function lets us leverage something called the "panic"
                                                // panic is a failure abstraction
                    // but how do we handle panics?
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}", mars_weight);
   
}

fn calculate_weight_on_mars(weight: f32) -> f32{
    (weight/9.811) * 3.111
}