fn main() {
  
    let number_list = vec![34, 50, 25, 100, 65];
    
    let result = largest(&number_list);
    println!("The largest number is {}", result)

}


fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn largest_char(list: &[char]) -> char {

    let mut largest = list[0]

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

}