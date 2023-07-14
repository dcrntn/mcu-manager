mod mcu_handler;

fn main() {
    let mcus : Vec<mcu_handler::McuData> = mcu_handler::get_mcu_ip();
    for mcu in &mcus {
        println!("uuid: {} ip: {} ", mcu.uuid, mcu.ip);
    }
    let json_mcus = mcu_handler::mcus_as_json(&mcus);

    println!("{}",json_mcus);
}
