// fn main() {
//     println!("Hello, world!");
// }

const SECONDS_IN_MINUTE: u32 = 60;
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

fn main() {
    // let mut total: i32 = 30;
    // println!("{} number", total);
    // total = 44;
    // println!("{} number", total);
    // let total: &str = "Texto";
    // println!("{} string", total);

    // {
    //     let total: i32 = 50;
    //     println!("Escopo interno {} number", total);
    // }

    // println!("Escopo externo {} number", total);

    let total = 30;
    let total_em_segundos: u32 = total * SECONDS_IN_HOUR;

    println!("Trabalhou {} segundos", total_em_segundos);
}
