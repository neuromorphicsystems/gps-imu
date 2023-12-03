#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/ft4222_bindings.rs"));

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("InvalidHandle")]
    InvalidHandle,
    #[error("DeviceNotFound")]
    DeviceNotFound,
    #[error("DeviceNotOpened")]
    DeviceNotOpened,
    #[error("IoError")]
    IoError,
    #[error("InsufficientResources")]
    InsufficientResources,
    #[error("InvalidParameter")]
    InvalidParameter,
    #[error("InvalidBaudRate")]
    InvalidBaudRate,
    #[error("DeviceNotOpenedForErase")]
    DeviceNotOpenedForErase,
    #[error("DeviceNotOpenedForWrite")]
    DeviceNotOpenedForWrite,
    #[error("FailedToWriteDevice")]
    FailedToWriteDevice,
    #[error("EepromReadFailed")]
    EepromReadFailed,
    #[error("EepromWriteFailed")]
    EepromWriteFailed,
    #[error("EepromEraseFailed")]
    EepromEraseFailed,
    #[error("EepromNotPresent")]
    EepromNotPresent,
    #[error("EepromNotProgrammed")]
    EepromNotProgrammed,
    #[error("InvalidArgs")]
    InvalidArgs,
    #[error("NotSupported")]
    NotSupported,
    #[error("OtherError")]
    OtherError,
    #[error("DeviceListNotReady")]
    DeviceListNotReady,
    #[error("Unknown status {0:?}")]
    Unknown(FT_STATUS),
}

fn check(value: FT_STATUS) -> Result<(), Error> {
    match value {
        FT_OK => Ok(()),
        FT_INVALID_HANDLE => Err(Error::InvalidHandle),
        FT_DEVICE_NOT_FOUND => Err(Error::DeviceNotFound),
        FT_DEVICE_NOT_OPENED => Err(Error::DeviceNotOpened),
        FT_IO_ERROR => Err(Error::IoError),
        FT_INSUFFICIENT_RESOURCES => Err(Error::InsufficientResources),
        FT_INVALID_PARAMETER => Err(Error::InvalidParameter),
        FT_INVALID_BAUD_RATE => Err(Error::InvalidBaudRate),
        FT_DEVICE_NOT_OPENED_FOR_ERASE => Err(Error::DeviceNotOpenedForErase),
        FT_DEVICE_NOT_OPENED_FOR_WRITE => Err(Error::DeviceNotOpenedForWrite),
        FT_FAILED_TO_WRITE_DEVICE => Err(Error::FailedToWriteDevice),
        FT_EEPROM_READ_FAILED => Err(Error::EepromReadFailed),
        FT_EEPROM_WRITE_FAILED => Err(Error::EepromWriteFailed),
        FT_EEPROM_ERASE_FAILED => Err(Error::EepromEraseFailed),
        FT_EEPROM_NOT_PRESENT => Err(Error::EepromNotPresent),
        FT_EEPROM_NOT_PROGRAMMED => Err(Error::EepromNotProgrammed),
        FT_INVALID_ARGS => Err(Error::InvalidArgs),
        FT_NOT_SUPPORTED => Err(Error::NotSupported),
        FT_OTHER_ERROR => Err(Error::OtherError),
        FT_DEVICE_LIST_NOT_READY => Err(Error::DeviceListNotReady),
        value => Err(Error::Unknown(value)),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Speed {
    HighSpeed,
    FullSpeed,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DeviceType {
    FTBM,
    FTAM,
    FT100AX,
    FT2232C,
    FT232R,
    FT2232H,
    FT4232H,
    FT232H,
    FTX,
    FT4222H0,
    FT4222H12,
    FT4222H3,
    FT4222PROG,
    FT900,
    FT930,
    FTUMFTPD3A,
    FT2233HP,
    FT4233HP,
    FT2232HP,
    FT4232HP,
    FT233HP,
    FT232HP,
    FT2232HA,
    FT4232HA,
    Unknown(FT_DEVICE),
}

impl From<FT_DEVICE> for DeviceType {
    fn from(value: FT_DEVICE) -> Self {
        match value {
            FT_DEVICE_BM => DeviceType::FTBM,
            FT_DEVICE_AM => DeviceType::FTAM,
            FT_DEVICE_100AX => DeviceType::FT100AX,
            FT_DEVICE_2232C => DeviceType::FT2232C,
            FT_DEVICE_232R => DeviceType::FT232R,
            FT_DEVICE_2232H => DeviceType::FT2232H,
            FT_DEVICE_4232H => DeviceType::FT4232H,
            FT_DEVICE_232H => DeviceType::FT232H,
            FT_DEVICE_X_SERIES => DeviceType::FTX,
            FT_DEVICE_4222H_0 => DeviceType::FT4222H0,
            FT_DEVICE_4222H_1_2 => DeviceType::FT4222H12,
            FT_DEVICE_4222H_3 => DeviceType::FT4222H3,
            FT_DEVICE_4222_PROG => DeviceType::FT4222PROG,
            FT_DEVICE_900 => DeviceType::FT900,
            FT_DEVICE_930 => DeviceType::FT930,
            FT_DEVICE_UMFTPD3A => DeviceType::FTUMFTPD3A,
            FT_DEVICE_2233HP => DeviceType::FT2233HP,
            FT_DEVICE_4233HP => DeviceType::FT4233HP,
            FT_DEVICE_2232HP => DeviceType::FT2232HP,
            FT_DEVICE_4232HP => DeviceType::FT4232HP,
            FT_DEVICE_233HP => DeviceType::FT233HP,
            FT_DEVICE_232HP => DeviceType::FT232HP,
            FT_DEVICE_2232HA => DeviceType::FT2232HA,
            FT_DEVICE_4232HA => DeviceType::FT4232HA,
            value => DeviceType::Unknown(value),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum StringError {
    #[error("wrong length (expected {expected}, got {got})")]
    Length { expected: usize, got: usize },
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SerialNumber(pub [i8; 16]);

impl TryFrom<String> for SerialNumber {
    type Error = StringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes();
        if bytes.len() != 16 {
            return Err(StringError::Length {
                expected: 16,
                got: bytes.len(),
            });
        }
        let mut result = Self([0; 16]);
        for (input, output) in bytes.iter().zip(result.0.iter_mut()) {
            // unsafe: *const u8 and *const i8 have the same size
            *output = unsafe { *(input as *const u8 as *const i8) };
        }
        Ok(result)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Description(pub [i8; 64]);

impl TryFrom<String> for Description {
    type Error = StringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes();
        if bytes.len() != 64 {
            return Err(StringError::Length {
                expected: 16,
                got: bytes.len(),
            });
        }
        let mut result = Self([0; 64]);
        for (input, output) in bytes.iter().zip(result.0.iter_mut()) {
            // unsafe: *const u8 and *const i8 have the same size
            *output = unsafe { *(input as *const u8 as *const i8) };
        }
        Ok(result)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location(pub u32);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeviceInfo {
    pub port_open: bool,
    pub speed: Speed,
    pub device_type: DeviceType,
    pub vendor_id: u16,
    pub product_id: u16,
    pub serial_number: SerialNumber,
    pub description: Description,
    pub index: usize,
    pub location: Location,
}

pub fn list_devices() -> Result<Vec<DeviceInfo>, Error> {
    let devices_length = {
        let mut devices_length: u32 = 0;
        check(unsafe { FT_CreateDeviceInfoList(&mut devices_length) })?;
        devices_length as usize
    };
    if devices_length == 0 {
        return Ok(Vec::new());
    }
    let mut info_nodes = vec![
        FT_DEVICE_LIST_INFO_NODE {
            Flags: 0,
            Type: 0,
            ID: 0,
            LocId: 0,
            SerialNumber: [0; 16],
            Description: [0; 64],
            ftHandle: std::ptr::null_mut(),
        };
        devices_length
    ];
    {
        let mut devices_length = devices_length as u32;
        check(unsafe {
            FT_GetDeviceInfoList(
                info_nodes.as_mut_ptr() as *mut FT_DEVICE_LIST_INFO_NODE,
                &mut devices_length,
            )
        })?;
    }
    let mut devices: Vec<_> = info_nodes
        .iter()
        .enumerate()
        .map(|(index, info_node)| DeviceInfo {
            index,
            location: Location(info_node.LocId),
            port_open: (info_node.Flags & 0b1) > 0,
            speed: if ((info_node.Flags & 0b10) >> 1) > 0 {
                Speed::HighSpeed
            } else {
                Speed::FullSpeed
            },
            device_type: info_node.Type.into(),
            product_id: ((info_node.ID >> 16) & 0xFFFF) as u16,
            vendor_id: (info_node.ID & 0xFFFF) as u16,
            serial_number: SerialNumber(info_node.SerialNumber),
            description: Description(info_node.Description),
        })
        .collect();
    devices.sort_unstable();
    Ok(devices)
}

#[derive(Debug)]
pub struct Handle(*mut ::std::os::raw::c_void);

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { FT4222_UnInitialize(self.0) };
        unsafe { FT_Close(self.0) };
    }
}

#[derive(Debug)]
#[repr(u32)]
pub enum ClockDivider {
    Two = FT4222_SPIClock_CLK_DIV_2,
    Four = FT4222_SPIClock_CLK_DIV_4,
    Eight = FT4222_SPIClock_CLK_DIV_8,
    Sixteen = FT4222_SPIClock_CLK_DIV_16,
    ThirtyTwo = FT4222_SPIClock_CLK_DIV_32,
    SixtyFour = FT4222_SPIClock_CLK_DIV_64,
    OneHundredTwentyEight = FT4222_SPIClock_CLK_DIV_128,
    TwoHundredFiftySix = FT4222_SPIClock_CLK_DIV_256,
    FiveHundredTwelve = FT4222_SPIClock_CLK_DIV_512,
}

#[derive(Debug)]
#[repr(u32)]
pub enum Active {
    High = FT4222_SPICPOL_CLK_IDLE_LOW,
    Low = FT4222_SPICPOL_CLK_IDLE_HIGH,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ClockPhase {
    LeadingEdge = FT4222_SPICPHA_CLK_LEADING,
    TrailingEdge = FT4222_SPICPHA_CLK_TRAILING,
}

#[derive(Debug)]
pub struct OutputMap {
    pub enable_0: bool,
    pub enable_1: bool,
    pub enable_2: bool,
    pub enable_3: bool,
}

impl OutputMap {
    fn serialize(&self) -> u8 {
        (self.enable_0 as u8)
            | ((self.enable_1 as u8) << 1)
            | ((self.enable_2 as u8) << 2)
            | ((self.enable_3 as u8) << 3)
    }
}

#[derive(Debug)]
#[repr(u32)]
pub enum ClockRate {
    TwentyFourMHz = FT4222_ClockRate_SYS_CLK_24,
    FourtyHeightMHz = FT4222_ClockRate_SYS_CLK_48,
    SixtyMHZ = FT4222_ClockRate_SYS_CLK_60,
    EightyMHz = FT4222_ClockRate_SYS_CLK_80,
}

impl DeviceInfo {
    pub fn open_spi(
        &self,
        clock_rate: ClockRate,
        clock_divider: ClockDivider,
        active: Active,
        clock_phase: ClockPhase,
        output_map: OutputMap,
        read_timeout: std::time::Duration,
        write_timeout: std::time::Duration,
    ) -> Result<Handle, Error> {
        let mut raw_handle = std::mem::MaybeUninit::uninit();
        check(unsafe { FT_Open(self.index as ::std::os::raw::c_int, raw_handle.as_mut_ptr()) })?;
        let raw_handle = unsafe { raw_handle.assume_init() };
        check(unsafe { FT_ResetDevice(raw_handle) })?;
        check(unsafe { FT4222_SetClock(raw_handle, clock_rate as u32) })?;
        check(unsafe {
            FT_SetTimeouts(
                raw_handle,
                read_timeout.as_millis() as u32,
                write_timeout.as_millis() as u32,
            )
        })?;
        check(unsafe {
            FT4222_SPIMaster_Init(
                raw_handle,
                FT4222_SPIMode_SPI_IO_SINGLE,
                clock_divider as u32,
                active as u32,
                clock_phase as u32,
                output_map.serialize(),
            )
        })?;
        Ok(Handle(raw_handle))
    }
}

impl Handle {
    pub fn read_write(
        &mut self,
        write_buffer: &mut [u8],
        read_buffer: &mut [u8],
    ) -> Result<u16, Error> {
        let mut bytes_transferred: u16 = 0;
        check(unsafe {
            FT4222_SPIMaster_SingleReadWrite(
                self.0,
                read_buffer.as_mut_ptr(),
                write_buffer.as_mut_ptr(),
                write_buffer.len() as u16,
                &mut bytes_transferred as *mut u16,
                1,
            )
        })?;
        Ok(bytes_transferred)
    }
}
