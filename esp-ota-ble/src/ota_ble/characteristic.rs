use anyhow::Result;
use esp_idf_svc::{
    bt::BtUuid,
    sys::{esp, esp_ble_gatts_get_attr_value},
};
use std::marker::PhantomData;

pub struct OtaCharacteristic<T> {
    uuid: BtUuid,
    attribute_handle: Option<u16>,

    _phantom: PhantomData<T>,
}

impl<T> OtaCharacteristic<T> {
    pub fn new(uuid: BtUuid) -> Self {
        Self {
            uuid,
            attribute_handle: None,
            _phantom: PhantomData,
        }
    }

    pub fn set_attribute_handle(&mut self, handle: u16) {
        self.attribute_handle = Some(handle);
    }

    pub fn is_initialized(&self) -> bool {
        self.attribute_handle.is_some()
    }

    pub fn get_value<'a>(&self) -> Result<T>
    where
        T: TryFrom<&'a [u8]>,
    {
        let raw_bytes = self.get_raw_value()?;

        T::try_from(raw_bytes).map_err(|_| anyhow::anyhow!("Failed to convert data: "))
    }

    pub fn get_raw_value<'a>(&self) -> Result<&'a [u8]> {
        let attr_handle = self
            .attribute_handle
            .ok_or_else(|| anyhow::anyhow!("Attribute handle not set"))?;

        let mut len: u16 = 0;
        let mut data: *const u8 = core::ptr::null_mut();

        let raw_data = unsafe {
            esp!(esp_ble_gatts_get_attr_value(
                attr_handle,
                &mut len,
                &mut data
            ))?;

            core::slice::from_raw_parts(data, len as _)
        };

        Ok(raw_data)
    }
}
