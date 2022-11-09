use std::io;

struct TEMPERATURE {
    symbol: String,
    name: String,
}
struct TemperatureList {
    c: TEMPERATURE,
    f: TEMPERATURE,
}

fn convert_temperature() {
    let fahreinheit = TEMPERATURE{
        symbol: "F".to_owned(),
        name: "Fahreinheit".to_owned()
    };
    let celcius = TEMPERATURE{
        symbol: "C".to_owned(),
        name: "Celcius".to_owned()
    };
    let temperature = TemperatureList{
        c: celcius,
        f: fahreinheit
    };

    println!("Please input your temperature number");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read temperature number");
    let temp: f64 = temp.trim().parse().expect("Please type a number");

    println!("Please input your temperature type");
    let mut temp_type = String::new();
    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to read temperature type");
    let temp_type = temp_type.trim().to_uppercase();

    let mut temp_calc_result: f64 = (temp - 18 as f64) / 1.8;
    let mut temp_name: String = temperature.f.name;
    let mut temp_sym: String = temperature.f.symbol;
    
    if temp_type == temp_sym {
        temp_calc_result = (temp * 1.8) + 32 as f64;
        temp_name = temperature.c.name;
        temp_sym = temperature.c.symbol;
    }

    println!("{temp}°{temp_type} converted to {temp_name} is: {temp_calc_result}°{temp_sym}");
}

fn main() {
    convert_temperature();
}
