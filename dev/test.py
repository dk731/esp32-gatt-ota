import asyncio
from bleak import BleakClient, BleakScanner


async def scan_ble_devices():
    while True:
        print("Scanning for BLE devices...")
        devices = await BleakScanner.discover()
        for device in devices:
            if device.name == "ESP32":

                print(f"Device found: {device.name}, Address: {device.address}")

                async with BleakClient(device.address) as client:
                    services = await client.get_services()
                    print(
                        f"Services and Characteristics for device at {device.address}:"
                    )
                    for service in services:
                        print(f"\nService: {service}")
                        characteristics = service.characteristics
                        for characteristic in characteristics:
                            print(f"  - Characteristic: {characteristic}")

        print()
        print()
        # await asyncio.sleep(1)


loop = asyncio.get_event_loop()
loop.run_until_complete(scan_ble_devices())


# import uuid


# def uuid128_to_int128(uuid_str):
#     # Parse the UUID string
#     uuid_obj = uuid.UUID(uuid_str)

#     # Get the components of the UUID
#     time_low = uuid_obj.time_low
#     time_mid = uuid_obj.time_mid
#     time_hi_and_version = uuid_obj.time_hi_version
#     clock_seq_hi_and_reserved = uuid_obj.clock_seq_hi_variant
#     clock_seq_low = uuid_obj.clock_seq_low
#     node = uuid_obj.node

#     # Concatenate the components into a single int128
#     int128_value = (
#         (time_low << 96)
#         | (time_mid << 80)
#         | (time_hi_and_version << 64)
#         | (clock_seq_hi_and_reserved << 56)
#         | (clock_seq_low << 48)
#         | node
#     )

#     return int128_value


# def generate_new_uuid_and_int128():
#     # Generate a new UUID
#     new_uuid = uuid.uuid4()

#     # Convert the UUID to a string
#     uuid_str = str(new_uuid)

#     # Convert the UUID string to int128
#     int128_value = uuid128_to_int128(uuid_str)

#     return new_uuid, int128_value


# # Generate a new UUID and its corresponding int128 value
# new_uuid, int128_value = generate_new_uuid_and_int128()
# print("New UUID:", new_uuid)
# print("Corresponding int128 value:", int128_value)
