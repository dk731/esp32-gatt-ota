pub enum OtaGattCommands {
    // Starts new file transfer, in case of an ongoing transfer, will result in an error
    StartTransfer = 0x01,

    // Clears the ongoing transfer, in case of no ongoing transfer, will result in an error
    ClearTransfer = 0x02,

    // Resets the device
    ResetDevice = 0x03,

    // ClearTransfer + StartTransfer in one command
    StartForceTransfer = 0x04,
}
