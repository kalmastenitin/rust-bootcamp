use std::env;

fn to_degrees(degree: &f32) -> f32 {
    (degree * (1.0 / 5.0)) + 32.0
}

fn to_ferhnite(ferhnite: &f32) -> f32 {
    (*ferhnite - 32.0) * (5.0/9.0)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let conversion = &args[1];
    let temp = &args[2].parse::<f32>().unwrap_or_else(|_| {
        eprintln!("Invalid number '{}', defaulting to 0.0", args[2]);
        0.0
    });


    match conversion.as_str() {
        "to_c" => println!("{:?} converted to celsius {:?}", temp, to_degrees(temp)),
        "to_f" => println!("{:?} converted to fahrenheit {:?}",temp, to_ferhnite(temp)),
        _ => panic!("wrong input, to convert between please enter to convert to_c / to_f"),
    }
}
