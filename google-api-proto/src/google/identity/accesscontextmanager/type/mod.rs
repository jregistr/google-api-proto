/// The encryption state of the device.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceEncryptionStatus {
    /// The encryption status of the device is not specified or not known.
    EncryptionUnspecified = 0,
    /// The device does not support encryption.
    EncryptionUnsupported = 1,
    /// The device supports encryption, but is currently unencrypted.
    Unencrypted = 2,
    /// The device is encrypted.
    Encrypted = 3,
}
impl DeviceEncryptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeviceEncryptionStatus::EncryptionUnspecified => "ENCRYPTION_UNSPECIFIED",
            DeviceEncryptionStatus::EncryptionUnsupported => "ENCRYPTION_UNSUPPORTED",
            DeviceEncryptionStatus::Unencrypted => "UNENCRYPTED",
            DeviceEncryptionStatus::Encrypted => "ENCRYPTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENCRYPTION_UNSPECIFIED" => Some(Self::EncryptionUnspecified),
            "ENCRYPTION_UNSUPPORTED" => Some(Self::EncryptionUnsupported),
            "UNENCRYPTED" => Some(Self::Unencrypted),
            "ENCRYPTED" => Some(Self::Encrypted),
            _ => None,
        }
    }
}
/// The operating system type of the device.
/// Next id: 7
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OsType {
    /// The operating system of the device is not specified or not known.
    OsUnspecified = 0,
    /// A desktop Mac operating system.
    DesktopMac = 1,
    /// A desktop Windows operating system.
    DesktopWindows = 2,
    /// A desktop Linux operating system.
    DesktopLinux = 3,
    /// A desktop ChromeOS operating system.
    DesktopChromeOs = 6,
    /// An Android operating system.
    Android = 4,
    /// An iOS operating system.
    Ios = 5,
}
impl OsType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OsType::OsUnspecified => "OS_UNSPECIFIED",
            OsType::DesktopMac => "DESKTOP_MAC",
            OsType::DesktopWindows => "DESKTOP_WINDOWS",
            OsType::DesktopLinux => "DESKTOP_LINUX",
            OsType::DesktopChromeOs => "DESKTOP_CHROME_OS",
            OsType::Android => "ANDROID",
            OsType::Ios => "IOS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OS_UNSPECIFIED" => Some(Self::OsUnspecified),
            "DESKTOP_MAC" => Some(Self::DesktopMac),
            "DESKTOP_WINDOWS" => Some(Self::DesktopWindows),
            "DESKTOP_LINUX" => Some(Self::DesktopLinux),
            "DESKTOP_CHROME_OS" => Some(Self::DesktopChromeOs),
            "ANDROID" => Some(Self::Android),
            "IOS" => Some(Self::Ios),
            _ => None,
        }
    }
}
/// The degree to which the device is managed by the Cloud organization.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceManagementLevel {
    /// The device's management level is not specified or not known.
    ManagementUnspecified = 0,
    /// The device is not managed.
    None = 1,
    /// Basic management is enabled, which is generally limited to monitoring and
    /// wiping the corporate account.
    Basic = 2,
    /// Complete device management. This includes more thorough monitoring and the
    /// ability to directly manage the device (such as remote wiping). This can be
    /// enabled through the Android Enterprise Platform.
    Complete = 3,
}
impl DeviceManagementLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeviceManagementLevel::ManagementUnspecified => "MANAGEMENT_UNSPECIFIED",
            DeviceManagementLevel::None => "NONE",
            DeviceManagementLevel::Basic => "BASIC",
            DeviceManagementLevel::Complete => "COMPLETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MANAGEMENT_UNSPECIFIED" => Some(Self::ManagementUnspecified),
            "NONE" => Some(Self::None),
            "BASIC" => Some(Self::Basic),
            "COMPLETE" => Some(Self::Complete),
            _ => None,
        }
    }
}
