/// A struct representing the identifiers for an teams API user.
///
/// # Fields
///
/// * `protocol_version` - A static string slice representing the version of the protocol.
/// * `manufacturer` - A static string slice representing the manufacturer of the device.
/// * `device` - A static string slice representing the device name.
/// * `app` - A static string slice representing the application name.
/// * `app_version` - A static string slice representing the version of the application.
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct AppIdentifiers {
    pub protocol_version: &'static str,
    pub manufacturer: &'static str,
    pub device: &'static str,
    pub app: &'static str,
    pub app_version: &'static str,
}
