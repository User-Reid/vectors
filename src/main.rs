fn main() {
    let pepperoni: String = String::from("Pepperoni");
    let mushroom: String = String::from("Mushroom");
    let sausage: String = String::from("Sausage");

    let pizza_toppings: Vec<String> = vec![pepperoni, mushroom, sausage];
    let mut delicious_toppings: Vec<String> = pizza_toppings;

    let topping_reference: &String = &delicious_toppings[1];
    println!("The topping is {topping_reference}");

    delicious_toppings.push(String::from("Olives"));
}
