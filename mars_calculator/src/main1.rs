use std::io;

fn main() {
        // this is on the stack
    //let a = 5;
        // when we do this, the value at a is copied to a new
            // location on the stack. b points to this new value.
        // b does NOT point to a. 
    //let b = a;

    // When both owners go out of scope, a "double free" occurs and may create a double free error
    let mut input = String::new();
        // we are trying to tell the compiler to copy a string and make a new reference to it.
            // string is a complex type. Complex types do not implement the Copy trait
    
    // rust moves ownership of the value pointed to by input to the 
        // argument var inside of some_fn.
    // when some_fn ends, the variable is deallocated.
        // This means that we cannot use 'input' after this point.
    //some_fn(input);
        // we get around this with the &. 

    // let's make two references to input

    let ref1 = & input;
    let ref2 = & ref1;

    // but, why would I make multiple immutable references to a mutable variable if I couldn't change the mutable variable?
        // does rust think I should not?
    //input += " more!";

    //BUT --> if we pass an &, we tell the compiler to let the function borrow
        // the variable passed to it
    mut_tester(&ref1, &ref2);
    input.push_str(" tagged");
    mut_tester(&input, &input);

    //mut_tester(& ref1, &ref2);
        // if we want to make the value mutable, we need to pass mut tag on both ends

    //io::stdin().read_line(&mut input)
    // rust compiler infers types.
    let mars_weight = calculate_weight_on_mars(100.0);
    println!("Weight on Mars: {}", mars_weight);
    // println! is a macro. ! denotes a macro.
        // what is a macro? Code that writes more code.
    
}
                                    // define the type
fn calculate_weight_on_mars(weight: f32) -> f32{
    (weight/9.811) * 3.111
    // this value is implicitly returned.
        // return keyword is used mostly to return early from a function
}


    // rust moves ownership of the value at 
            // the $ tag specifies that this function will 'borrow' the parameter passed
                // and then return ownership to the variable that was passed.
fn mut_tester(ref1: & String, ref2: &String){
    println!("ref1 {} \t ref2: {}",ref1,ref2);

    // BUT! borrowed values are IMMUTABLE by default. 
    // Pass the &mut tag in order to change the value.
    //ref1.push_str("a");

}


// mut restriction provides a great benefit:
    // there cannot be data races at compile time.

// this asks a writer to create a single variable for each use.

