# Microcontroller manager software
A small homemade project to manage multiple microcontrollers on your local network.

Sum up of what this software should do:
- Get the IP of each microcontroller on the local network without messing with the home router. (Retriev the dynamic IPs on the local network)
- Work as a "data gateway". (Be able to read and write the microcontrollers. Also be able to communicate with the "outworld" if needed)
- Be able to do OTA updates for the microcontrollers.
- Have some sort of GUI to present the datas, and offer some interractions with the controllers.

As of now i'm using ESP32 for the development, so the sample programs will be written around that type of controller. In the future other controllers might be added. 

## Note
This is a get to know RUST project, the implementation and approach might not please everyone.

This project is intended to be run on a SBC e.g (raspberry). However as of now nothing restricts the program to be able to run elsewhere, but running in VMs or containers might give complications while retrieveing the local IPs.

## Features
The planned and already implemented functions of the software. In the end all function should be combined into an ESP32 library for easier use.
### Implemented
1. Get mcu's IP with UDP packet on local network. [esp32-udp-ip-get](https://github.com/dcrntn/esp32-udp-ip-get)

### Not implemented
1. API to read and write the controllers.
2. OTA update
3. GUI 
4. ESP32 library

## Installation
1. Clone or download this repo.
2. Build with Cargo
3. Run

## Usage
*As of now*

If you run the generated binary **AND** you have microcontrollers, that are running the [esp32-udp-ip-get](https://github.com/dcrntn/esp32-udp-ip-get) program, than you should retrieve the controllers' uuid, and the mcus' local network ip address.
