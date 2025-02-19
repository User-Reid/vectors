fn main() {
    let mut pizza_diameters: Vec<u8> = vec![8, 10, 12, 14];

    let pepperoni: String = String::from("Pepperoni");
    let mushroom: String = String::from("Mushroom");
    let sausage: String = String::from("Sausage");
    let pizza_toppings: Vec<String> = vec![pepperoni, mushroom, sausage];

    let option = pizza_toppings.get(2);
    println!("{}", option.unwrap());
}
