// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

use ding_machine::*;

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);

    print_difference(coords.0, coords.1);

    let coords_arr = [coords.0, coords.1]; // create an array literal out of parts of `coord` here
    print_array(coords_arr); // and pass it in here (this line doesn't need to change)

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[6]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);

    print_distance(coords);
}
