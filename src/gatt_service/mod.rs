use std::sync::Arc;

use anyhow::Result;
use esp32_nimble::{
    utilities::{mutex::Mutex, BleUuid},
    uuid128, BLECharacteristic, BLEServer, NimbleProperties, OnWriteArgs,
};
use esp_idf_svc::{
    ota::EspOta,
    sys::{
        esp_partition_find, esp_partition_get, esp_partition_iterator_release, esp_partition_next,
        esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_ANY,
        esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_DATA_OTA,
        esp_partition_type_t_ESP_PARTITION_TYPE_APP, BLE_ATT_MTU_MAX,
    },
};
use uuid::Uuid;

mod uuids;

pub use uuids::OtaGattUuids;

pub struct OtaGattService {
    file_block: Arc<Mutex<BLECharacteristic>>,
    total_file_size: Arc<Mutex<BLECharacteristic>>,
    file_hash: Arc<Mutex<BLECharacteristic>>,
    status: Arc<Mutex<BLECharacteristic>>,
    command: Arc<Mutex<BLECharacteristic>>,
    finished_upload: Arc<Mutex<BLECharacteristic>>,

    // esp_ota
    esp_ota: EspOta,
    max_ota_size: usize,
}

impl OtaGattService {
    pub fn new(server: &mut BLEServer, uuids: Option<OtaGattUuids>) -> Result<Arc<Mutex<Self>>> {
        let max_ota_size = Self::get_max_ota_size()?;
        let esp_ota = EspOta::new()?;

        let uuids = uuids.unwrap_or_default();
        let service = server.create_service(uuids.service);
        let mut service = service.lock();

        let new_service = Self {
            file_block: service.create_characteristic(uuids.file_block, NimbleProperties::WRITE),
            total_file_size: service.create_characteristic(
                uuids.total_file_size,
                NimbleProperties::READ | NimbleProperties::WRITE,
            ),
            file_hash: service.create_characteristic(
                uuids.file_hash,
                NimbleProperties::READ | NimbleProperties::WRITE,
            ),
            status: service.create_characteristic(uuids.status, NimbleProperties::READ),
            command: service.create_characteristic(uuids.command, NimbleProperties::WRITE),
            finished_upload: service
                .create_characteristic(uuids.finished_upload, NimbleProperties::WRITE),
            esp_ota,
            max_ota_size,
        };
        let new_service = Arc::new(Mutex::new(new_service));
        Self::init_callbacks(new_service.clone())?;

        Ok(new_service)
    }

    // Verifies that partitions table was correctly set up
    // and return the maximum allowed OTA size
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

    fn init_callbacks(ota_state: Arc<Mutex<Self>>) -> Result<()> {
        // self.file_block.

        let state_clone = ota_state.clone();
        ota_state
            .lock()
            .command
            .lock()
            .on_write(move |args| Self::command_handler(state_clone.clone(), args));

        Ok(())
    }

    fn command_handler(ota_state: Arc<Mutex<Self>>, args: &mut OnWriteArgs) {}
}
