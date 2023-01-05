// strcutmulti.rs

struct Result {
    add: f64,
    multi: f64
}


fn add_mul(x: f64, y: f64) -> Result {
    Result{
        add: x + y,
        multi: x * y
    }
}

fn main() {
    let res = add_mul(2.0,10.0);

    // can _extract_ values
    println!("add {} mul {}", res.add, res.multi);
}
