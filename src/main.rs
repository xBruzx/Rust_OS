fn main() {
    println!("Hello, world!");
    joe();
    compound_types();
}

// Data types: i32, u32, f64, bool, char.
fn joe() {
    let x: i32 = -5;
    let y: u32 = 10;

    println!("Assigned x: {}", x);
    println!("Unassigned y: {}", y);

    let z: f64 = 3.14;
    let is_active: bool = true;
    let letter: char = 'A';
    println!("Float z: {}, Boolean is_active: {}, Char letter: {}", z, is_active, letter);
}

//compound types: tuples, arrays.
fn compound_types() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);
    let mut fruits: [&str; 3] = ["apple", "Orange", "Banana"];
    println!("Fruits: {:?}", fruits);

    let human: (&str, i32, bool) = ("Joey", 21, false);
    print!("Tuple: {:?}", human);

    let my_mixed_tuple: (&str, i32, bool) = ("Kratos", 3000, true);
    println!("My mixed tuple: {:?}", my_mixed_tuple);
    println!("{:?}", my_mixed_tuple);

    fruits[0] = "Mango";
    println!("Updated fruits: {:?}", fruits);

    let mut stone_cold: String = String::from("Austin 3:16 says ");
    stone_cold.push_str("I just whipped your ass!");
    stone_cold.push_str(" and thats the bottom line!");

    println!("Stone cold Steve Austin says: {}", stone_cold);

}