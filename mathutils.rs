// mathutils.rs

fn eval_poly(p: &[f64],  x: f64) -> f64 {
    // Compute polynomial P(x) where P is a vector of coefficients, highest
    // order coefficient at P[0].  Uses Horner's Method.
    let mut res = 0.0;
    for c in p.iter() {
        res = x * res + c;
    }
    return res
}

// for i in (5..-5).step_by(-1) {
//     println!("The value of i is: {}", i);
// }

fn deriv_poly(p: &[f64]) -> Vec<f64> {
    // Compute derivative of a polynomial given coefficients 
    // highest order first P[0].
    let mut result = vec![];
    let mut d = 0.0;
    let n = p.len();
    for (i, &c) in p.iter().enumerate().take(n - 1) {
        d = (n - i - 1) as f64;
        result.push(c * d);
    }
    return result
}

fn newtons_method_poly(p: &[f64], x0: f64, epsilon: f64, max_iter: i32) -> f64 {
        // p: a list of coefficients (high to low) the polynomial function for which you want to find the root.
        // x0: an initial approximation of the root.
        // epsilon: a small number, which represents the maximum error tolerance.
        let mut steps = 0;
        let mut x = x0;
        let deriv = deriv_poly(p);
        loop {
            let new_x = x - eval_poly(p, x) / eval_poly(&deriv, x);
            if (new_x - x).abs() < epsilon {
                return new_x;
            }
            x = new_x;
            steps += 1;
            if (steps) > max_iter {
                println!("Max Iterations Reached");
                break 0.0;
            }
        }
}

fn main() {
    // let arr = [1.0, 2.0, 3.0];
    let arr = [-100.0, 39.0, 59.0, 55.0, 20.0];
    let root = newtons_method_poly(&arr, 1.0, 1e-6, 1000);
    println!("Root: {:?}", root);
    // let t =  eval_poly(&arr, 2.0);
    // // can debug print
    // println!("t {:?}", t);

    // let d = deriv_poly(&arr);
    // let r =  eval_poly(&d, 2.0);
    // // can debug print
    // println!("r {:?}", r);
}