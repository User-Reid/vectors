fn main() {
    let mut pizza_diameters: Vec<u8> = vec![8, 10, 12, 14];

    let pepperoni: String = String::from("Pepperoni");
    let mushroom: String = String::from("Mushroom");
    let sausage: String = String::from("Sausage");
    let pizza_toppings: Vec<String> = vec![pepperoni, mushroom, sausage];

    let value = &pizza_toppings[1..];
    println!("{value:?}")
}
