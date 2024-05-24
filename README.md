# ESP BlueDroid 

This project aims to provide a simple and easy to use API for the BlueDroid stack on the ESP32 platform. Using official `esp-rs` safe rust wrappers (`esp-idf-svc`).

## Structure
- `esp-bluedroid` - BlueDroid abstraction layer using `esp-idf-svc` bindings
- `esp-ota-ble` - BLE GATT service for OTA updates, using `esp-bluedroid`
- `esp-ota-ble-cli` - binary for OTA updates over BLE from CLI