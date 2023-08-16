
use std::{net::UdpSocket,str, time::Duration};
use serde::{Serialize};
use serde_json;


    // Data structure for the microcontroller
    #[derive(Serialize)]
    pub struct McuData {

        // McuData.uuid : this is a random uuid, it should be unique to each microcontroller.
        pub uuid : String,
        
        // McuData.ip : this is the ip address of the microcontroller on the local network
        pub ip : String
    }

    // Gets the ip addresses and uuids from the local network.
    // This way the microcontrollers' ip can be dynamic, but it can be obtained if needed.
    pub fn get_mcu_ip() -> Vec<McuData> {

        // Bind the socket to recive from every ip.
        let sock = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind address");

        // Set the broadcast to true.
        let _ = sock.set_broadcast(true);

        // The identification string, this must be the same as on the microcontroller.
        let ident_call = "iue";

        // Broadcast the identification string to the local network.
        let _ =  sock.send_to(ident_call.as_bytes(),"192.168.1.255:43770").expect("couldn't send data");

        // Buffer for the response
        let mut buf = [0; 50];

        // Set timeout for the socket response check, thus it isn't blocking forever
        let to_duration = Duration::new(2,0);
        sock.set_read_timeout(Some(to_duration)).expect("set_read_timeout call failed");
        
        // Init the vector that will hold the microcontroller infos
        let mut mcu_datas : Vec<McuData> = Vec::new();

        // Wait for the responses and parse them. Maximum blocking time is the given timeout above.
        while let Ok((_ , addr)) = sock.recv_from(&mut buf) {

            let str_addr = addr.to_string();

            let split = str_addr.split(":");

            let split_vec : Vec<&str> = split.collect();

            let mcu_ip = split_vec[0];

            let resp = str::from_utf8(&buf).unwrap();

            let cur_mcu = McuData {
                uuid : String::from(&resp[6..]),
                ip : String::from(mcu_ip)
            };

            mcu_datas.push(cur_mcu);

        }

        // Return the available microcontrollers on the local network
        mcu_datas
        
    }

    // Returns the microcontrollers' data as json, for easier implementation of rest api in the future.
    pub fn mcus_as_json(mcus : &Vec<McuData>) -> String {
        
        serde_json::to_string(mcus).unwrap().replace("\\u0000", "")
    }