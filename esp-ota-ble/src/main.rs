use lazy_static::lazy_static;
use std::{
    borrow::Borrow,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use anyhow::{Ok, Result};
// use esp32_nimble::{uuid128, BLEAdvertisementData, BLEDevice};
use esp_idf_svc::{
    self,
    bt::{
        ble::{
            gap::{AdvConfiguration, AppearanceCategory, EspBleGap},
            gatt::{
                server::{EspGatts, GattsEvent},
                AutoResponse, GattCharacteristic, GattId, GattServiceId, GattStatus, Permission,
                Property,
            },
        },
        Ble, BtMode, BtUuid,
    },
    hal::{
        modem::{self, BluetoothModem, Modem},
        peripheral::Peripheral,
    },
    nvs::{EspDefaultNvsPartition, EspNvsPartition, NvsDefault},
    sys::{esp_log_level_set, esp_log_level_t_ESP_LOG_NONE},
};

use esp_idf_svc::bt::BtDriver;

const PROFILE_A_APP_ID: u16 = 0;
const PROFILE_B_APP_ID: u16 = 1;

use esp32_gatt_ote::ota_ble::{uuids::GattUuids, BleParams, OtaBle};

lazy_static! {
    // static ref QWE: Arc<Mutex<BluetoothModem>> =
    //     Arc::new(Mutex::new(unsafe { BluetoothModem::new() }));
    static ref DRIVER: BtDriver<'static, Ble> = BtDriver::new(unsafe { BluetoothModem::new() }, None).unwrap();
}

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // bt::

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // 1. Initialize NVS
    // let _nvs_default_partition = EspDefaultNvsPartition::take()?;

    let ota_ble = OtaBle::new(BleParams::default(), GattUuids::default())?;
    ota_ble.subscribe_gap_event(|ev| {
        log::info!("GAP Event (FROM MAIN): {:?}", ev);
    });

    ota_ble.subscribe_gatt_event(|gatt_if, ev| {
        log::info!("GATT Event (FROM MAIN): {:?} {:?}", gatt_if, ev);
    });

    ota_ble.start_service()?;

    // // Obtain gatt and gap instances
    // let gatt: Arc<EspGatts<Ble, &BtDriver<'_, Ble>>> = Arc::new(EspGatts::new((*DRIVER).borrow())?);
    // let gap: Arc<EspBleGap<Ble, &BtDriver<'_, Ble>>> =
    //     Arc::new(EspBleGap::new((*DRIVER).borrow())?);

    // let gatt_if_a: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(None));
    // let gatt_if_b: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(None));

    // let a = gap.clone();

    // // 3. Initialize gap and gatt callbacks
    // gap.subscribe(move |event| {
    //     log::info!("GAP Event: {:?}", event);
    // })?;

    // let (a_tmp, b_tmp) = (gatt_if_a.clone(), gatt_if_b.clone());
    // unsafe {
    //     gatt.subscribe_nonstatic(|(gatt_if, event)| {
    //         log::info!("GATT Event: {:?} {:?}", gatt_if, event);

    //         #[allow(clippy::single_match)]
    //         match event {
    //             GattsEvent::ServiceRegistered { status, app_id } => {
    //                 if status == GattStatus::Ok {
    //                     if app_id == PROFILE_A_APP_ID {
    //                         *a_tmp.lock().unwrap() = Some(gatt_if);
    //                     } else if app_id == PROFILE_B_APP_ID {
    //                         *b_tmp.lock().unwrap() = Some(gatt_if);
    //                     }
    //                 }
    //             }
    //             GattsEvent::PeerConnected {
    //                 conn_id,
    //                 link_role,
    //                 addr,
    //                 conn_params,
    //             } => {
    //                 // gatt.send_response(gatt_if, conn_id, 0, GattStatus::Ok, &[])?;
    //                 gap.start_advertising().unwrap();
    //                 // gap_clone.start_advertising();
    //             }
    //             _ => {}
    //         };
    //     })
    // }?;

    // // 4. Register GATT application profiles
    // gatt.register_app(PROFILE_A_APP_ID)?;
    // gatt.register_app(PROFILE_B_APP_ID)?;

    // log::info!("GAP and GATT initialized");

    // // 5. Set advertising configuration
    // gap.set_adv_conf(&AdvConfiguration {
    //     set_scan_rsp: true,
    //     include_name: true,
    //     include_txpower: true,
    //     min_interval: 0,
    //     max_interval: 0,
    //     appearance: AppearanceCategory::ControlDevice,
    //     service_uuid: Some(BtUuid::uuid128(28632491779680083979757574010553820695)),
    //     service_data: None,
    //     manufacturer_data: None,
    //     ..Default::default()
    // })?;

    // log::info!("Set GAP advertising configuration");

    // // 7. Create GATT service and characteristic
    // let service_id = GattServiceId {
    //     id: GattId {
    //         uuid: BtUuid::uuid128(248923085985797927161606609401596650095),
    //         inst_id: 3,
    //     },
    //     is_primary: true,
    // };

    // let char_id = GattCharacteristic {
    //     uuid: BtUuid::uuid128(313649116642616186956252207059869165533),
    //     permissions: Permission::Read | Permission::Write,
    //     properties: Property::Read | Property::Write,
    //     max_len: 8,
    //     auto_rsp: AutoResponse::ByApp,
    // };

    // log::info!("Prepared GATT Service and Characteristic IDs");

    // gatt.create_service(gatt_if_a.lock().unwrap().unwrap(), &service_id, 4)?;
    // gatt.create_service(gatt_if_b.lock().unwrap().unwrap(), &service_id, 4)?;

    // log::info!("Created GATT Service");

    // gatt.start_service(40)?;
    // gatt.start_service(44)?;

    // log::info!("Started GATT Service");

    // gatt.add_characteristic(40, &char_id, "1010".as_bytes())?;
    // gatt.add_characteristic(44, &char_id, "AAAA".as_bytes())?;

    // log::info!("Added GATT Characteristic");

    // // 6. Start advertising
    // gap.start_advertising()?;

    // log::info!("Started GAP advertising");

    loop {
        esp_idf_svc::hal::delay::FreeRtos::delay_ms(1000);
    }

    Ok(())
}
