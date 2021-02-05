use std::env;

fn main() {
    
    let atomic_mass = [0.0, 1.008, 4.0026, 6.94, 9.0122, 10.81, 12.011, 14.007, 15.999, 18.998, 20.180, 22.990, 24.305, 26.982, 28.085, 30.074, 32.06, 35.45, 39.948];
    let args: Vec<String> = env::args().collect();
    let quantities = make_quantities(args);
    let mut mm = 0.0;
    for i in 1..quantities.len() {
        mm += quantities[i] as f32 * atomic_mass[i];
    }
    println!("Molar Mass: {} g/mol", mm);
    

}

fn make_quantities (args: Vec<String>) -> Vec<i32> {
    let mut quant = vec![0; 19];
    let mut zero = true;
    for i in args {
        if !zero {
            match i.as_str() {
                "h" => quant[1] += 1,
                "he" => {quant[2] += 1},
                "li" => {quant[3] += 1}, 
                "be" => {quant[4] += 1}, 
                "b" => {quant[5] += 1}, 
                "c" => {quant[6] += 1}, 
                "n" => {quant[7] += 1}, 
                "o" => {quant[8] += 1}, 
                "f" => {quant[9] += 1}, 
                "ne" => {quant[10] += 1}, 
                "na" => {quant[11] += 1}, 
                "mg" => {quant[12] += 1}, 
                "al" => {quant[13] += 1}, 
                "si" => {quant[14] += 1}, 
                "p" => {quant[15] += 1}, 
                "s" => {quant[16] += 1}, 
                "cl" => {quant[17] += 1}, 
                "ar" => {quant[18] += 1}, 
                _ => println!("Symbol not recognized!"),
            }
        } else {
            zero = false;
        }
    } 
    quant
}
