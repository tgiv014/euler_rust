// A pythagorean triplet is a set of numbers a<b<c for which a^2+b^2=c^2
// i.e. 3^2+4^2=5^2
// There's one triplet for which a + b + c = 1000
// Find the product abc

fn main() {
    for a in 1..1000 {
        for b in a+1..1000 {
            let a_f:f64 = a as f64;
            let b_f:f64 = b as f64;
            let c_f_sq = a_f.powi(2) + b_f.powi(2);
            let c_f = c_f_sq.sqrt();

            if a_f+b_f+c_f > 1000.0 {
                // We've gone too far... start over at the next a value
                break;
            }

            if c_f_sq.fract() != 0.0 {
                continue;
            }

            if a_f + b_f + c_f == 1000.0 {
                println!("a{} b{} c{}", a_f, b_f, c_f);
                println!("Product: {}", a_f*b_f*c_f);
                return;
            }

            // println!("a {} b {} c {}", a, b, c);
        }
    }
}