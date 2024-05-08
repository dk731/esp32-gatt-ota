use esp32_nimble::{utilities::BleUuid, uuid128};
use uuid::Uuid;

pub struct OtaGattUuids {
    pub service: BleUuid,
    pub file_block: BleUuid,
    pub total_file_size: BleUuid,
    pub file_hash: BleUuid,
    pub status: BleUuid,
    pub command: BleUuid,
    pub finished_upload: BleUuid,
}

impl Default for OtaGattUuids {
    fn default() -> Self {
        Self {
            service: uuid128!("81ea96fb-1117-4ea4-9df0-d30cd73e0e76"),
            file_block: uuid128!("c5b46514-cd62-4724-9bb0-6f3368ba9ccb"),
            total_file_size: uuid128!("f24a3175-9c51-49ad-a1c6-2fe655c5378d"),
            file_hash: uuid128!("6020ac05-81c0-4a68-925d-f468daa7435d"),
            status: uuid128!("f4f3b121-b0e0-49c9-8290-465b060c3ce9"),
            command: uuid128!("332bf171-2717-4636-8042-3b0313f661aa"),
            finished_upload: uuid128!("484490cc-643d-4ab7-aa95-a6cae13622fa"),
        }
    }
}

impl OtaGattUuids {
    pub fn random() -> Self {
        Self {
            service: BleUuid::Uuid128(*Uuid::new_v4().as_bytes()),
            file_block: BleUuid::Uuid128(*Uuid::new_v4().as_bytes()),
            total_file_size: BleUuid::Uuid128(*Uuid::new_v4().as_bytes()),
            file_hash: BleUuid::Uuid128(*Uuid::new_v4().as_bytes()),
            status: BleUuid::Uuid128(*Uuid::new_v4().as_bytes()),
            command: BleUuid::Uuid128(*Uuid::new_v4().as_bytes()),
            finished_upload: BleUuid::Uuid128(*Uuid::new_v4().as_bytes()),
        }
    }
}
