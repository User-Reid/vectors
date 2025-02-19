fn main() {
    let mut pizza_diameters: Vec<u8> = vec![8, 10, 12, 14];
    pizza_diameters.push(12);
    println!("{pizza_diameters:?}");

    pizza_diameters.insert(2, 31);
    println!("{pizza_diameters:?}");

    let last_pizza_diamater = pizza_diameters.pop();
    println!("{:?}", last_pizza_diamater.unwrap());

    pizza_diameters.remove(3);
    println!("{:?}", pizza_diameters)
}
