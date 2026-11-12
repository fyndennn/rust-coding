use std::io;
use std::num::ParseFloatError;

fn main() {
    let mut u_tmp = String::new();
    let mut u_unit = String::new();
    println!("Provide a temperature");
    io::stdin()
        .read_line(&mut u_tmp)
        .expect("Invalid tmp provided {u_tmp}");

    println!("Provide a unit (either C or F)");
    io::stdin()
        .read_line(&mut u_unit)
        .expect("Invalid unit provided {u_unit}");

    match tmp_converter(&u_tmp, &u_unit) {
        Ok((tmp, unit)) => println!("This is your converted temperature: {tmp}{unit}"),
        Err(err) => println!("We got this problem: {err}"),
    }
}

fn tmp_converter(tmp: &str, unit: &str) -> Result<(f64, &'static str), ParseFloatError> {
    let tmp: f64 = match tmp.trim().parse() {
        Ok(num) => num,
        Err(e) => return Err(e),
    };

    let (tmp, unit) = match unit.trim() {
        "C" | "c" => (tmp * 9. / 5. + 32., "F"),
        "F" | "f" => ((tmp - 32.) / (9. / 5.), "C"),
        _ => (f64::NAN, "?"),
    };

    Ok((tmp, unit))
}
