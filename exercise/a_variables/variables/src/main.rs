const STARTING_MISSILES: i32 = 8;
const READY_COUNT: i32 = 2;

fn main() {
    // let mut missiles = STARTING_MISSILES;
    // let ready = READY_COUNT;
    let (mut missiles, ready) = (STARTING_MISSILES, READY_COUNT);

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles -= ready;
    println!("{} missiles left", missiles);
}
