use esp_idf_svc::bt::BtUuid;

use crate::uuid128;

pub struct GattUuids {
    pub service: BtUuid,
    pub file_block: BtUuid,
    pub total_file_size: BtUuid,
    pub file_hash: BtUuid,
    pub status: BtUuid,
    pub command: BtUuid,
    pub finished_upload: BtUuid,
}

impl Default for GattUuids {
    fn default() -> Self {
        Self {
            service: uuid128!("81ea96fb-1117-4ea4-9df0-d30cd73e0e76"),
            file_block: uuid128!("075e8648-5b20-42c9-a492-b0ce7548be7c"),
            total_file_size: uuid128!("92e8d217-f306-418e-b75b-894b288b6664"),
            file_hash: uuid128!("923930e3-686a-409e-a1e0-c7bbd8bb3d50"),
            status: uuid128!("e4ccad22-e983-42a9-9c95-7f4909ff885f"),
            command: uuid128!("92fa0fe8-35ff-442f-a00c-010ebd91ef6a"),
            finished_upload: uuid128!("e6b7ae4f-d7ff-43f6-a378-86cf740040db"),
        }
    }
}

impl GattUuids {
    pub fn random() -> Self {
        Self {
            service: uuid128!(),
            file_block: uuid128!(),
            total_file_size: uuid128!(),
            file_hash: uuid128!(),
            status: uuid128!(),
            command: uuid128!(),
            finished_upload: uuid128!(),
        }
    }
}
