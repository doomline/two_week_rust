// Control Flow

// IF expressions 
// made up of if <condition> {<code>} 
// sometimes we will include an else statement so if ____ is true do this else(if not true) do this______
// theres also else if, when working with numbers always start from the highest and work your way down
fn main() {
    let number = 3;
    if number < 5 {
        println!("this is true");
    } else {
        println!("this is false");
    }

}


// LOOPS 

// loops can be initiated by just using the word loop 

fn main() {
    loop {
        println!("loop!")
    }
}

// return values from loops 

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }
}

// conditional loops will use WHILE
// this is a count down, so while the number doesnt = 0 count backwards
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number -1;
    } 

    println!("LIFTOFF!!");
}

// you can also use for loops 

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}