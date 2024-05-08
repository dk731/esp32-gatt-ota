use std::{
    borrow::Borrow,
    sync::{Arc, Mutex},
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
        modem::{self, Modem},
        peripheral::Peripheral,
    },
    nvs::{EspDefaultNvsPartition, EspNvsPartition, NvsDefault},
    sys::{esp_log_level_set, esp_log_level_t_ESP_LOG_NONE},
};

use esp_idf_svc::bt::BtDriver;

const PROFILE_A_APP_ID: u16 = 0;
const PROFILE_B_APP_ID: u16 = 1;

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // bt::

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // 1. Initialize NVS
    let _nvs_default_partition = EspDefaultNvsPartition::take()?;

    // 2. Initialize bluetooth stack (blueroid)
    let modem = unsafe { Modem::new() };
    let (_, bt_modem) = modem.split();
    let driver: BtDriver<Ble> = BtDriver::new(bt_modem, None)?;

    // Obtain gatt and gap instances
    let gatt = EspGatts::new(driver.borrow())?;
    let gap: EspBleGap<Ble, &BtDriver<Ble>> = EspBleGap::new(driver.borrow())?;

    let gatt_if_a: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(None));
    let gatt_if_b: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(None));

    // 3. Initialize gap and gatt callbacks
    gap.subscribe(|event| {
        log::info!("GAP Event: {:?}", event);
    })?;

    let (a_tmp, b_tmp) = (gatt_if_a.clone(), gatt_if_b.clone());
    gatt.subscribe(move |(gatt_if, event)| {
        log::info!("GATT Event: {:?} {:?}", gatt_if, event);

        #[allow(clippy::single_match)]
        match event {
            GattsEvent::ServiceRegistered { status, app_id } => {
                if status == GattStatus::Ok {
                    if app_id == PROFILE_A_APP_ID {
                        *a_tmp.lock().unwrap() = Some(gatt_if);
                    } else if app_id == PROFILE_B_APP_ID {
                        *b_tmp.lock().unwrap() = Some(gatt_if);
                    }
                }
            }
            _ => {}
        };
    })?;

    // 4. Register GATT application profiles
    gatt.register_app(PROFILE_A_APP_ID)?;
    // gatt.register_app(PROFILE_B_APP_ID)?;

    log::info!("GAP and GATT initialized");

    // 5. Set advertising configuration
    gap.set_adv_conf(&AdvConfiguration {
        set_scan_rsp: true,
        include_name: true,
        include_txpower: true,
        min_interval: 0,
        max_interval: 0,
        appearance: AppearanceCategory::ControlDevice,
        service_uuid: Some(BtUuid::uuid128(28632491779680083979757574010553820695)),
        service_data: None,
        manufacturer_data: None,
        ..Default::default()
    })?;

    log::info!("Set GAP advertising configuration");

    // 6. Start advertising
    gap.start_advertising()?;

    log::info!("Started GAP advertising");

    // 7. Create GATT service and characteristic
    let service_id = GattServiceId {
        id: GattId {
            uuid: BtUuid::uuid128(248923085985797927161606609401596650095),
            inst_id: 3,
        },
        is_primary: true,
    };

    let char_id = GattCharacteristic {
        uuid: BtUuid::uuid128(313649116642616186956252207059869165533),
        permissions: Permission::Read | Permission::Write,
        properties: Property::Read | Property::Write,
        max_len: 8,
        auto_rsp: AutoResponse::ByApp,
    };

    log::info!("Prepared GATT Service and Characteristic IDs");

    gatt.create_service(gatt_if_a.lock().unwrap().unwrap(), &service_id, 4)?;
    // gatt.create_service(gatt_if_b.lock().unwrap().unwrap(), &service_id, 4)?;

    log::info!("Created GATT Service");

    gatt.start_service(40)?;
    // gatt.start_service(44)?;

    log::info!("Started GATT Service");

    gatt.add_characteristic(40, &char_id, "1010".as_bytes())?;
    // gatt.add_characteristic(44, &char_id, "AAAA".as_bytes())?;

    log::info!("Added GATT Characteristic");

    // gatt.start_service()?;

    // gatt.create_service(PROFILE_A_APP_ID, &service_id, 1)?;

    // log::info!("GAP and GATT initialized");

    // log::info!("Starting GAP");

    // gap.subscribe(|event| {
    //     log::info!("GAP Event: {:?}", event);
    // })?;

    // log::info!("Subscribed to GAP events");

    // gap.set_adv_conf(&AdvConfiguration::default())?;

    // log::info!("Set GAP advertising configuration");

    // gap.start_advertising()?;

    // log::info!("Started GAP advertising");

    // let service_id = GattServiceId {
    //     id: GattId {
    //         uuid: BtUuid::uuid128(241716678439078648716531568473180149994),
    //         inst_id: 0,
    //     },
    //     is_primary: true,
    // };

    // let char_id = GattCharacteristic {
    //     uuid: BtUuid::uuid128(8241668614845853411578711250896091271),
    //     permissions: Permission::Read | Permission::Write,
    //     properties: Property::Read | Property::Write,
    //     max_len: 8,
    //     auto_rsp: AutoResponse::ByApp,
    // };

    // log::info!("Prepared GATT Service and Characteristic IDs");

    // // gatt.register_app(0)?;

    // gatt.subscribe(|(gatt_if, event)| {
    //     log::info!("GATT Event: {:?} {:?}", gatt_if, event);
    // })?;

    // log::info!("Subscribed to GATT events");

    // gatt.create_service(0, &service_id, 1)?;

    // log::info!("Created GATT Service");

    // gatt.add_characteristic(0, &char_id, &[0, 0, 0, 0])?;

    // log::info!("Added GATT Characteristic");

    // gatt.start_service(0)?;

    // log::info!("Started GATT Service");

    // driver.

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
