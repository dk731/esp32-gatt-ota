#[macro_export]
macro_rules! uuid128 {
    ($uuid_str:expr) => {{
        use esp_idf_svc::bt::BtUuid;
        use uuid::Uuid;

        // Parse the UUID string
        let parsed_uuid = Uuid::parse_str($uuid_str).expect("Invalid UUID string");

        BtUuid::uuid128(parsed_uuid.as_u128())
    }};
    () => {{
        use esp_idf_svc::bt::BtUuid;
        use uuid::Uuid;

        BtUuid::uuid128(Uuid::new_v4().as_u128())
    }};
}
