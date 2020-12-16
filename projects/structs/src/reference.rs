// Structs are similar to tuples, you can use multiple data types
// structs require you to name each piece of data with values
// Naming convention as follows -> struct <Name> {Data: Datatype}



//defined the parameters of the struct named "User"
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let user1 = User { // saying this struct is named user1 and is using format of User
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
} 

// getting values from a struct uses . (dot) notation
// say if we want the users username

user1.username 

// if you want to change something then the struct must be mutable

let mut user1 = User { // saying this struct is named user1 and is using format of User
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
} 

user1.username = String::from("anotherusername111");

// You can also build struct functions 
// first define the struct Params such as above
fn build_user(email: String, username: String) -> User {
    User{
        email: email, // you can use the field init shorthand syntax and do
        // email,
        // username,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Creating Instances from Other Instances with Struct Update Syntax

let user2 = User {
    email: String::from("anotherexample"),
    username: String::from("another321"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
    /* using the shorthand syntax we can write 
    ..user1 ( .. specifies that the remaining fields should have the same value )
    */
}


// you can build tuple structs which are structs that look like tuples

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Color(0, 0, 0);


//