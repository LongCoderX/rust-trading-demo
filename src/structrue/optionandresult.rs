fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("The divisor is not zero."));
    }
    return Ok(a / b);
}

fn saft_sqrt(n: f64) -> Option<f64> {
    if n < 0.0 {
        return None;
    }
    return Some(n.sqrt());
}

pub fn test_option_and_result() {
    println!("============== No.4 Option and Result practice ==============");

    let a = 10.23;
    let b = 3.3;
    match divide(a, b) {
        Ok(ret) => {
            println!("The {a} / {b} = {ret}.");
        },
        Err(err) => {
            println!("{err}");
        }
    }

    let a = 10.23;
    let b = 0.0;
    match divide(a, b) {
        Ok(ret) => {
            println!("The {a} / {b} = {ret}.");
        },
        Err(err) => {
            println!("{err}");
        }
    }

    let n = 100.0;
    if let Some(ret) = saft_sqrt(n) {
        println!("The {n} sqrt is {ret}");
    } else {
        println!("The n is {n}, but not sqrt.");
    }


    let n = -100.0;
    if let Some(ret) = saft_sqrt(n) {
        println!("The {n} sqrt is {ret}");
    } else {
        println!("The n is {n}, but not sqrt.");
    }
}