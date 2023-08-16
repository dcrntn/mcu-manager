mod mcu_get_ip;

fn main() {
    let mcus : Vec<mcu_get_ip::McuData> = mcu_get_ip::get_mcu_ip();
    for mcu in &mcus {
        println!("uuid: {} ip: {} ", mcu.uuid, mcu.ip);
    }
    let json_mcus = mcu_get_ip::mcus_as_json(&mcus);

    println!("{}",json_mcus);
}
