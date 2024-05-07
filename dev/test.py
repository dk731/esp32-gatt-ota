import asyncio
from bleak import BleakScanner


async def scan_ble_devices():
    print("Scanning for BLE devices...")

    while True:
        devices = await BleakScanner.discover()
        for device in devices:
            if device.address == "7C:DF:A1:E8:7B:CE":
                print(
                    f"Device found: {device.name}, Address: {device.address}, Details: {device.details()}"
                )

        await asyncio.sleep(1)


loop = asyncio.get_event_loop()
loop.run_until_complete(scan_ble_devices())
