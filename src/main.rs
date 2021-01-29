use std::env;

fn main() {
    
    let atomic_mass = [0.0, 1.008, 4.0026, 6.94, 9.0122, 10.81, 12.011, 14.007, 15.999, 18.998, 20.180];
    let args: Vec<String> = env::args().collect();
    let quantities = make_quantities(args);
    let mut mm = 0.0;
    for i in 1..11 {
        mm += quantities[i] as f32 * atomic_mass[i];
    }
    println!("Molar Mass: {} g/mol", mm);
    

}

fn make_quantities (args: Vec<String>) -> Vec<i32> {
    let mut quant = vec![0; 11];
    let mut zero = true;
    for i in args {
        if !zero {
            if i == "h" {quant[1] += 1} 
            if i == "he" {quant[2] += 1}
            if i == "li" {quant[3] += 1} 
            if i == "be" {quant[4] += 1} 
            if i == "b" {quant[5] += 1} 
            if i == "c" {quant[6] += 1} 
            if i == "n" {quant[7] += 1} 
            if i == "o" {quant[8] += 1} 
            if i == "f" {quant[9] += 1} 
            if i == "ne" {quant[10] += 1} 
        } else {
            zero = false;
        }
    } 
    quant
}
