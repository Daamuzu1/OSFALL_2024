const FREEZINGPOINT: f64 = 32.0; //Freezing point of water in fahrenheit

fn fahrenheit_to_celsius(f: f64) -> f64 {
     (f - FREEZINGPOINT)*5.0/9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZINGPOINT
}

fn main() {
        let mut tempf = 77.0;
        let x = fahrenheit_to_celsius(tempf);
        println!("The temperature in degree celsius is = {}",x);

        for _ in 1..=5 {
            tempf += 1.0;
            let x = fahrenheit_to_celsius(tempf);
            println!("The temperature in degree celsius is = {}",x);
        }

}
