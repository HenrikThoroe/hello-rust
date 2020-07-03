mod math;

use math::funcs::pow as pow;
use math::funcs::mult as mult;

fn main() {
    let power = pow(&2, &3);
    let prod = mult(&2, &3);
    println!("{}, {}", power, prod);
}
