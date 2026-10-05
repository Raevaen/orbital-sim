// Logic here
mod physics;
mod vec2;
use physics::circular_speed;

fn main() {
    println!("Orbital simulation starting...");

    let speed = circular_speed(90_000.0, 0.0);
    match speed {
        Some(s) => println!("Circular orbital speed: {}", s),
        None => println!("Invalid input for circular speed calculation"),
    }
}
