use anyhow::{Ok, Result};
// use esp32_nimble::{uuid128, BLEAdvertisementData, BLEDevice};
use esp_idf_svc::{
    self,
    sys::{esp_log_level_set, esp_log_level_t_ESP_LOG_NONE},
};

// use esp_idf_svc::bt::ble::gatt

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // bt::

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // let gatt_server = esp32_gatt_ote::init_new_server()?;

    // let ble_device = BLEDevice::take();
    // let ble_advertising = ble_device.get_advertising();
    // let server = ble_device.get_server();

    // ble_advertising
    //     .lock()
    //     .set_data(BLEAdvertisementData::new().name("ESP32-GATT-Server"))
    //     .unwrap();
    // ble_advertising.lock().start().unwrap();

    // server.ble_gatts_show_local();

    let mut counter = 0;
    loop {
        esp_idf_svc::hal::delay::FreeRtos::delay_ms(1000);
    }

    Ok(())
}
