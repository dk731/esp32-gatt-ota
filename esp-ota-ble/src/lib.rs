// mod gatt_service;
pub mod ota_ble;
// use anyhow::Result;

// use esp32_nimble::{BLEDevice, BLEServer};
// use esp_idf_svc;
// use gatt_service::OtaGattService;

// /// Initialized new GATT server with OTA service
// ///
// pub fn init_new_server() -> Result<()> {
//     let ble_device = BLEDevice::take();
//     let ble_advertising = ble_device.get_advertising();
//     let server = ble_device.get_server();

//     server.on_connect(|server, desc| {
//         log::info!("Client connected: {:?}", desc);

//         server
//             .update_conn_params(desc.conn_handle(), 24, 48, 0, 60)
//             .unwrap();

//         if server.connected_count() < (esp_idf_svc::sys::CONFIG_BT_NIMBLE_MAX_CONNECTIONS as _) {
//             log::info!("Multi-connect support: start advertising");
//             ble_advertising.lock().start().unwrap();
//         }
//     });

//     // server.on_authentication_complete(callback)

//     server.on_disconnect(|_desc, reason| {
//         log::info!("Client disconnected ({:?})", reason);
//     });

//     // server.on_confirm_pin(callback)

//     let ota_gatt_service = OtaGattService::new(server, None)?;

//     log::info!("Successfully initialized GATT server with OTA service");

//     Ok(())
// }

pub fn qwe() {}
