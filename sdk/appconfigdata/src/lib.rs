#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>AppConfig Data provides the data plane APIs your application uses to retrieve configuration data.
//! Here's how it works:</p>
//! <p>Your application retrieves configuration data by first establishing a configuration
//! session using the AppConfig Data <a>StartConfigurationSession</a> API action. Your session's
//! client then makes periodic calls to <a>GetLatestConfiguration</a> to check for
//! and retrieve the latest data available.</p>
//! <p>When calling <code>StartConfigurationSession</code>, your code sends the following
//! information:</p>
//! <ul>
//! <li>
//! <p>Identifiers (ID or name) of an AppConfig application, environment, and
//! configuration profile that the session tracks.</p>
//! </li>
//! <li>
//! <p>(Optional) The minimum amount of time the session's client must wait between calls
//! to <code>GetLatestConfiguration</code>.</p>
//! </li>
//! </ul>
//! <p>In response, AppConfig provides an <code>InitialConfigurationToken</code> to be given to
//! the session's client and used the first time it calls <code>GetLatestConfiguration</code>
//! for that session.</p>
//! <p>When calling <code>GetLatestConfiguration</code>, your client code sends the most recent
//! <code>ConfigurationToken</code> value it has and receives in response:</p>
//! <ul>
//! <li>
//! <p>
//! <code>NextPollConfigurationToken</code>: the <code>ConfigurationToken</code> value
//! to use on the next call to <code>GetLatestConfiguration</code>.</p>
//! </li>
//! <li>
//! <p>
//! <code>NextPollIntervalInSeconds</code>: the duration the client should wait before
//! making its next call to <code>GetLatestConfiguration</code>. This duration may vary
//! over the course of the session, so it should be used instead of the value sent on the
//! <code>StartConfigurationSession</code> call.</p>
//! </li>
//! <li>
//! <p>The configuration: the latest data intended for the session. This may be empty if
//! the client already has the latest version of the configuration.</p>
//! </li>
//! </ul>
//! <p>For more information and to view example CLI commands that show how to retrieve a
//! configuration using the AppConfig Data <code>StartConfigurationSession</code> and
//! <code>GetLatestConfiguration</code> API actions, see <a href="http://docs.aws.amazon.com/appconfig/latest/userguide/appconfig-retrieving-the-configuration">Receiving the
//! configuration</a> in the <i>AppConfig User Guide</i>.</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
mod http_serde;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::Blob;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("appconfigdata", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
