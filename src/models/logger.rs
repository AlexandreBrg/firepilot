/*
 * Firecracker API
 *
 * RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying JSON modeled data. The transport medium is a Unix Domain Socket.
 *
 * The version of the OpenAPI document: 1.3.0
 * Contact: compute-capsule@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// Logger : Describes the configuration option for the logging capability.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Logger {
    /// Set the level. The possible values are case-insensitive.
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    /// Path to the named pipe or file for the human readable log output.
    #[serde(rename = "log_path")]
    pub log_path: String,
    /// Whether or not to output the level in the logs.
    #[serde(rename = "show_level", skip_serializing_if = "Option::is_none")]
    pub show_level: Option<bool>,
    /// Whether or not to include the file path and line number of the log's origin.
    #[serde(rename = "show_log_origin", skip_serializing_if = "Option::is_none")]
    pub show_log_origin: Option<bool>,
}

impl Logger {
    /// Describes the configuration option for the logging capability.
    pub fn new(log_path: String) -> Logger {
        Logger {
            level: None,
            log_path,
            show_level: None,
            show_log_origin: None,
        }
    }
}

/// Set the level. The possible values are case-insensitive.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "Error")]
    Error,
    #[serde(rename = "Warning")]
    Warning,
    #[serde(rename = "Info")]
    Info,
    #[serde(rename = "Debug")]
    Debug,
}

impl Default for Level {
    fn default() -> Level {
        Self::Error
    }
}
