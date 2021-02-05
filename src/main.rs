use std::env;

fn main() {
    
    let atomic_mass = [0.0, 1.008, 4.0026, 6.94, 9.0122, 10.81, 12.011, 14.007, 15.999, 18.998, 20.180, 22.990, 24.305, 26.982, 28.085, 30.074, 32.06, 35.45, 39.948, 39.098, 40.078, 44.956, 47.867, 50.942, 51.996, 54.938, 55.845, 58.933, 58.693, 63.546, 65.38, 69.723, 72.630, 74.922, 78.971, 79.904, 83.798, 85.468, 87.62];
    let args: Vec<String> = env::args().collect();
    let quantities = make_quantities(args);
    let mut mm = 0.0;
    for i in 1..quantities.len() {
        mm += quantities[i] as f32 * atomic_mass[i];
    }
    println!("Molar Mass: {} g/mol", mm);
    

}

fn make_quantities (args: Vec<String>) -> Vec<i32> {
    let mut quant = vec![0; 39];
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
                "k" => {quant[19] += 1}, 
                "ca" => {quant[20] += 1}, 
                "sc" => {quant[21] += 1}, 
                "ti" => {quant[22] += 1}, 
                "v" => {quant[23] += 1}, 
                "cr" => {quant[24] += 1}, 
                "mn" => {quant[25] += 1}, 
                "fe" => {quant[26] += 1}, 
                "co" => {quant[27] += 1}, 
                "ni" => {quant[28] += 1}, 
                "cu" => {quant[29] += 1}, 
                "zn" => {quant[30] += 1}, 
                "ga" => {quant[31] += 1}, 
                "ge" => {quant[32] += 1}, 
                "as" => {quant[33] += 1}, 
                "se" => {quant[34] += 1}, 
                "br" => {quant[35] += 1}, 
                "kr" => {quant[36] += 1}, 
                "rb" => {quant[37] += 1}, 
                "sr" => {quant[38] += 1}, 
                _ => println!("Symbol not recognized!"),
            }
        } else {
            zero = false;
        }
    } 
    quant
}
