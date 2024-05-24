lazy_static! {
    pub static ref BT_DRIVER: StaticBtDriver =
        BtDriver::new(unsafe { BluetoothModem::new() }, None).unwrap();
    pub static ref GAP: StaticEspBleGap = Arc::new(EspBleGap::new((*BT_DRIVER).borrow()).unwrap());
    pub static ref GATT: StaticEspGatts = Arc::new(EspGatts::new((*BT_DRIVER).borrow()).unwrap());
    static ref OTA_BLE: Arc<Mutex<Option<Arc<OtaBle>>>> = Arc::new(Mutex::new(None));
}
