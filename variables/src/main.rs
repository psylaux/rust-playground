const STARTING_MISSLES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    //Part 1

    let mut missles = STARTING_MISSLES;
    let ready = READY_AMOUNT;
    println!("Firing {} of my {} missles", ready, missles);

    //Part 2

    missles = missles - ready;
    println!("{} missles left", missles);
}
