#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p> Amazon S3 Glacier (Glacier) is a storage solution for "cold data."</p>
//!
//! <p>Glacier is an extremely low-cost storage service that provides secure,
//! durable, and easy-to-use storage for data backup and archival. With Glacier,
//! customers can store their data cost effectively for months, years, or decades.
//! Glacier also enables customers to offload the administrative burdens of operating and
//! scaling storage to AWS, so they don't have to worry about capacity planning, hardware
//! provisioning, data replication, hardware failure and recovery, or time-consuming hardware
//! migrations.</p>
//!
//! <p>Glacier is a great storage choice when low storage cost is paramount and your
//! data is rarely retrieved. If your
//! application requires fast or frequent access to your data, consider using Amazon S3. For
//! more information, see <a href="http://aws.amazon.com/s3/">Amazon Simple Storage Service
//! (Amazon S3)</a>.</p>
//!
//! <p>You can store any kind of data in any format. There is no maximum limit on the total
//! amount of data you can store in Glacier.</p>
//!
//! <p>If you are a first-time user of Glacier, we recommend that you begin by
//! reading the following sections in the <i>Amazon S3 Glacier Developer
//! Guide</i>:</p>
//! <ul>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/introduction.html">What is
//! Amazon S3 Glacier</a> - This section of the Developer Guide describes the
//! underlying data model, the operations it supports, and the AWS SDKs that you can use
//! to interact with the service.</p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/amazon-glacier-getting-started.html">Getting Started
//! with Amazon S3 Glacier</a> - The Getting Started section walks you through the
//! process of creating a vault, uploading archives, creating jobs to download archives,
//! retrieving the job output, and deleting archives.</p>
//! </li>
//! </ul>
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
mod glacier_checksums;
mod http_serde;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Generated accessors for nested fields
mod lens;
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
/// Paginators for the service
pub mod paginator;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::byte_stream::AggregatedBytes;
    pub use aws_smithy_http::byte_stream::ByteStream;
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::Blob;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("glacier", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
