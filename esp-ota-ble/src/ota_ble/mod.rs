use std::{
    borrow::Borrow,
    mem::size_of,
    rc::Rc,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
};

use anyhow::Result;
use esp_idf_svc::{
    bt::{
        ble::{
            gap::{AdvConfiguration, BleGapEvent, EspBleGap},
            gatt::{
                server::{EspGatts, GattsEvent},
                AutoResponse, GattCharacteristic, GattId, GattServiceId, GattStatus, Permission,
                Property,
            },
        },
        Ble, BtDriver,
    },
    hal::modem::BluetoothModem,
    ota::EspOta,
    sys::{
        esp_partition_find, esp_partition_get, esp_partition_iterator_release, esp_partition_next,
        esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_ANY,
        esp_partition_type_t_ESP_PARTITION_TYPE_APP,
    },
};
use lazy_static::lazy_static;

use self::uuids::GattUuids;

pub mod characteristic;
mod commands;
pub mod macros;
pub mod uuids;

type StaticBtDriver = BtDriver<'static, Ble>;
type StaticEspGatts = Arc<EspGatts<'static, Ble, &'static BtDriver<'static, Ble>>>;
type StaticEspBleGap = Arc<EspBleGap<'static, Ble, &'static BtDriver<'static, Ble>>>;

lazy_static! {
    pub static ref BT_DRIVER: StaticBtDriver =
        BtDriver::new(unsafe { BluetoothModem::new() }, None).unwrap();
    pub static ref GAP: StaticEspBleGap = Arc::new(EspBleGap::new((*BT_DRIVER).borrow()).unwrap());
    pub static ref GATT: StaticEspGatts = Arc::new(EspGatts::new((*BT_DRIVER).borrow()).unwrap());
    static ref OTA_BLE: Arc<Mutex<Option<Arc<OtaBle>>>> = Arc::new(Mutex::new(None));
}

pub struct BleParams {
    ota_app_id: u16,
    service_instance_id: u8,
    max_block_size: usize,
}

impl Default for BleParams {
    fn default() -> Self {
        Self {
            ota_app_id: 254,
            service_instance_id: 3,
            max_block_size: 512,
        }
    }
}

type GapCallback = Box<dyn FnMut(&BleGapEvent) + Send + 'static>;
type GattCallback = Box<dyn FnMut(u8, &GattsEvent) + Send + 'static>;

pub struct OtaBle {
    ble_uuids: GattUuids,
    ble_params: BleParams,
    service_handle: Mutex<Option<u16>>,

    gap_callbacks: Mutex<Vec<GapCallback>>,
    gatt_callbacks: Mutex<Vec<GattCallback>>,

    connected_peers: Mutex<Vec<u16>>,

    // esp_ota
    esp_ota: EspOta,
}

impl OtaBle {
    pub fn new(ble_params: BleParams, ble_uuids: GattUuids) -> Result<Arc<Self>> {
        let existing_ote = OTA_BLE.lock().unwrap();
        if existing_ote.is_some() {
            return Err(anyhow::anyhow!("OtaBle has already been initialized"));
        }

        // Verify if current runtime is ready for OTA
        let _max_ota_size = Self::get_max_ota_size()?;
        let esp_ota = EspOta::new()?;

        // Initialize blueroid stack
        lazy_static::initialize(&BT_DRIVER);
        lazy_static::initialize(&GATT);
        lazy_static::initialize(&GAP);

        // let (gat_sender, gat_receiver) = channel::<BleGapEvent>();
        // let (gatt_sender, gatt_receiver) = channel::<GattsEvent>();

        let ota_ble = Arc::new(Self {
            esp_ota,
            ble_uuids,
            ble_params,
            service_handle: Mutex::new(None),
            gap_callbacks: Mutex::new(Vec::new()),
            gatt_callbacks: Mutex::new(Vec::new()),
            connected_peers: Mutex::new(Vec::new()),
        });
        Self::init_ble(ota_ble.clone())?;

        Ok(ota_ble)
    }

    /// Subscribe to BLE (GAP, GATT) events
    fn init_ble(ota_ble: Arc<Self>) -> Result<()> {
        let ota_ble_clone = ota_ble.clone();
        GAP.subscribe(move |event| {
            log::info!("GAP Event: {:?}", event);

            // First call user defined callbacks
            ota_ble_clone
                .gap_callbacks
                .lock()
                .unwrap()
                .iter_mut()
                .for_each(|cb| cb(&event));

            // Then call default event handler
            if let Err(error) = ota_ble_clone.gap_event_handler(&event) {
                log::error!("Error handling GAP event: {:?}", error);
            }
        })?;

        let ota_ble_clone = ota_ble.clone();
        GATT.subscribe(move |(gatt_if, event)| {
            log::info!("GATT Event: {:?} {:?}", gatt_if, event);

            // First call user defined callbacks
            ota_ble_clone
                .gatt_callbacks
                .lock()
                .unwrap()
                .iter_mut()
                .for_each(|cb| cb(gatt_if, &event));

            // Then call default event handler
            if let Err(error) = ota_ble_clone.gatt_event_handler(gatt_if, &event) {
                log::error!("Error handling GATT event: {:?}", error);
            }
        })?;

        GAP.set_adv_conf(&AdvConfiguration::default())?;
        // GAP.set_conn_params_conf(addr, min_int_ms, max_int_ms, latency_ms, timeout_ms)

        // Register OTA app
        GATT.register_app(ota_ble.ble_params.ota_app_id)?;

        Ok(())
    }

    pub fn subscribe_gap_event<F>(&self, callback: F)
    where
        F: FnMut(&BleGapEvent) + Send + 'static,
    {
        self.gap_callbacks.lock().unwrap().push(Box::new(callback));
    }

    pub fn subscribe_gatt_event<F>(&self, callback: F)
    where
        F: FnMut(u8, &GattsEvent) + Send + 'static,
    {
        self.gatt_callbacks.lock().unwrap().push(Box::new(callback));
    }

    fn gap_event_handler(&self, event: &BleGapEvent) -> Result<()> {
        // match event {
        //     // BleGapEvent::
        //     _ => {}
        // }

        Ok(())
    }

    fn gatt_event_handler(&self, gatt_if: u8, event: &GattsEvent) -> Result<()> {
        match event {
            GattsEvent::ServiceRegistered { status, app_id } => {
                if *app_id == self.ble_params.ota_app_id {
                    if *status != GattStatus::Ok {
                        return Err(anyhow::anyhow!("Failed to register OTA GATT service"));
                    }

                    log::info!("OTA service registered");

                    GATT.create_service(
                        gatt_if,
                        &GattServiceId {
                            id: GattId {
                                uuid: self.ble_uuids.service.clone(),
                                inst_id: self.ble_params.service_instance_id,
                            },
                            is_primary: true,
                        },
                        8,
                    )?;
                }
            }
            GattsEvent::ServiceCreated {
                status,
                service_handle,
                service_id,
            } => {
                if service_id.id.uuid == self.ble_uuids.service {
                    if *status != GattStatus::Ok {
                        return Err(anyhow::anyhow!("Failed to create OTA GATT service"));
                    }

                    log::info!("OTA service created");

                    self.service_handle.lock().unwrap().replace(*service_handle);

                    // Create characteristics
                    self.add_ote_characteristics()?;
                }
            }
            GattsEvent::CharacteristicAdded {
                status,
                attr_handle,
                service_handle,
                char_uuid,
            } => {
                if let Some(ota_handle) = self.service_handle.lock().unwrap().as_ref() {
                    if *service_handle == *ota_handle {
                        if *status != GattStatus::Ok {
                            return Err(anyhow::anyhow!("Failed to add OTA characteristic"));
                        }

                        log::info!("OTA characteristic added: {:?}", char_uuid);

                        // GATT.start_service(*service_handle)?;
                    }
                }
            }
            GattsEvent::ServiceStarted {
                status,
                service_handle,
            } => {
                if let Some(ota_handle) = self.service_handle.lock().unwrap().as_ref() {
                    if *service_handle == *ota_handle {
                        if *status != GattStatus::Ok {
                            return Err(anyhow::anyhow!("Failed to start OTA service"));
                        }

                        log::info!("OTA service started");
                    }
                }
            }
            GattsEvent::PeerConnected { .. } => {
                // TODO: check if max connections reached before starting advertising
                // GAP.start_advertising().unwrap();
            }
            _ => {}
        }

        Ok(())
    }

    pub fn start_service(&self) -> Result<()> {
        GAP.start_advertising()?;

        Ok(())
    }

    fn add_ote_characteristics(&self) -> Result<()> {
        let Some(service_handle) = *self.service_handle.lock().unwrap() else {
            return Err(anyhow::anyhow!("Service handle not set yet"));
        };

        GATT.add_characteristic(
            service_handle,
            &GattCharacteristic {
                uuid: self.ble_uuids.file_block.clone(),
                permissions: Permission::Read | Permission::Write,
                properties: Property::Read | Property::Write,
                max_len: self.ble_params.max_block_size,
                auto_rsp: AutoResponse::ByGatt,
            },
            &[],
        )?;

        GATT.add_characteristic(
            service_handle,
            &GattCharacteristic {
                uuid: self.ble_uuids.file_hash.clone(),
                permissions: Permission::Read | Permission::Write,
                properties: Property::Read | Property::Write,
                max_len: 32,
                auto_rsp: AutoResponse::ByGatt,
            },
            &[],
        )?;

        GATT.add_characteristic(
            service_handle,
            &GattCharacteristic {
                uuid: self.ble_uuids.finished_upload.clone(),
                permissions: Permission::Read.into(),
                properties: Property::Read | Property::Notify,
                max_len: 8,
                auto_rsp: AutoResponse::ByGatt,
            },
            &[],
        )?;

        // GATT.set_attr(attr_handle, data)
        GATT.add_characteristic(
            service_handle,
            &GattCharacteristic {
                uuid: self.ble_uuids.status.clone(),
                permissions: Permission::Read.into(),
                properties: Property::Read | Property::Notify,
                max_len: 1,
                auto_rsp: AutoResponse::ByGatt,
            },
            &[],
        )?;

        GATT.add_characteristic(
            service_handle,
            &GattCharacteristic {
                uuid: self.ble_uuids.total_file_size.clone(),
                permissions: Permission::Read.into(),
                properties: Property::Read | Property::Notify,
                max_len: size_of::<usize>(),
                auto_rsp: AutoResponse::ByGatt,
            },
            &[],
        )?;

        Ok(())
    }

    /// Verifies that partitions table was correctly set up
    /// and return the maximum allowed OTA size (detected by the smallest OTA partition size)
    /// If no OTA partitions are found, an error is returned
    fn get_max_ota_size() -> Result<usize> {
        log::info!("Checking OTA partitions");
        let mut max_ota_size: usize = 0;

        unsafe {
            // Iterate over all OTA partitions
            let mut it = esp_partition_find(
                esp_partition_type_t_ESP_PARTITION_TYPE_APP,
                esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_ANY,
                std::ptr::null(),
            );

            while !it.is_null() {
                let partition = esp_partition_get(it);

                if !partition.is_null() {
                    let label = (*partition).label.as_ptr();
                    let size = (*partition).size as usize;

                    log::info!(
                        "Found OTA partition: {:?}, size: {}",
                        std::ffi::CStr::from_ptr(label).to_str()?,
                        size
                    );

                    if size > max_ota_size {
                        max_ota_size = size;
                    }
                }

                it = esp_partition_next(it);
            }

            esp_partition_iterator_release(it);
        }

        if max_ota_size == 0 {
            log::error!("No OTA partitions found");
            Err(anyhow::anyhow!(
                "No OTA partitions found, verify partition table"
            ))
        } else {
            log::info!("Max OTA size: {}", max_ota_size);
            Ok(max_ota_size)
        }
    }
}

impl Drop for OtaBle {
    fn drop(&mut self) {
        OTA_BLE.lock().unwrap().take();
    }
}

// impl OtaGattService {
//     pub fn new(server: &mut BLEServer, uuids: Option<OtaGattUuids>) -> Result<Arc<Mutex<Self>>> {
//         let max_ota_size = Self::get_max_ota_size()?;
//         let esp_ota = EspOta::new()?;

//         let uuids = uuids.unwrap_or_default();
//         let service = server.create_service(uuids.service);
//         let mut service = service.lock();

//         // service

//         let new_service = Self {
//             file_block: service.create_characteristic(uuids.file_block, NimbleProperties::WRITE),
//             total_file_size: service.create_characteristic(
//                 uuids.total_file_size,
//                 NimbleProperties::READ | NimbleProperties::WRITE | NimbleProperties::READ_ENC,
//             ),
//             file_hash: service.create_characteristic(
//                 uuids.file_hash,
//                 NimbleProperties::READ | NimbleProperties::WRITE,
//             ),
//             status: service.create_characteristic(uuids.status, NimbleProperties::READ),
//             command: service.create_characteristic(uuids.command, NimbleProperties::WRITE),
//             finished_upload: service
//                 .create_characteristic(uuids.finished_upload, NimbleProperties::WRITE),
//             esp_ota,
//             max_ota_size,
//         };
//         let new_service = Arc::new(Mutex::new(new_service));
//         Self::init_callbacks(new_service.clone())?;

//         Ok(new_service)
//     }

//     // Verifies that partitions table was correctly set up
//     // and return the maximum allowed OTA size

//     fn init_callbacks(ota_state: Arc<Mutex<Self>>) -> Result<()> {
//         // self.file_block.

//         let state_clone = ota_state.clone();
//         ota_state
//             .lock()
//             .command
//             .lock()
//             .on_write(move |args| Self::command_handler(state_clone.clone(), args));

//         let state_clone = ota_state.clone();
//         ota_state
//             .lock()
//             .status
//             .lock()
//             .on_read(move |attr, desc| Self::status_handler(state_clone.clone(), attr, desc));

//         Ok(())
//     }

//     fn command_handler(ota_state: Arc<Mutex<Self>>, args: &mut OnWriteArgs) {
//         // args.
//         // args.recv_data()
//     }

//     fn status_handler(ota_state: Arc<Mutex<Self>>, args: &mut AttValue, desc: &BLEConnDesc) {
//         // desc.
//         // desc.
//         // args.
//         // args.recv_data()
//     }
// }
