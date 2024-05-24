fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    loop {
        esp_idf_svc::hal::delay::FreeRtos::delay_ms(1000);
    }

    Ok(())
}
