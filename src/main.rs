fn main() {
    let pepperoni: String = String::from("Pepperoni");
    let mushroom: String = String::from("Mushroom");
    let sausage: String = String::from("Sausage");
    let mut pizza_toppings: Vec<String> = vec![pepperoni, mushroom, sausage];

    pizza_toppings[1] = String::from("Olives");
    println!("{pizza_toppings:?}");

    let target_topping: &mut String = &mut pizza_toppings[2];
    target_topping.push_str(" and Meatballs");
    let x: &mut String = &mut pizza_toppings[1];
    println!("{}", x);
    let y: &String = &pizza_toppings[1];
    println!("{pizza_toppings:?}");
}
