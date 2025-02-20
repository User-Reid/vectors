fn main() {
    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!(
        "Length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    seasons.push("Summer");
    seasons.push("Spring");
    seasons.push("Fall");
    seasons.push("Winter");

    println!("{seasons:?}");
    println!(
        "Length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    seasons.push("Banana");
    println!("{seasons:?}");
    println!(
        "Length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );
}
