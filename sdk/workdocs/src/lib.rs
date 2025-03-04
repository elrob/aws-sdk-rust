#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>The WorkDocs API is designed for the following use cases:</p>
//! <ul>
//! <li>
//! <p>File Migration: File migration applications are supported for users who
//! want to migrate their files from an on-premises or off-premises file system or
//! service. Users can insert files into a user directory structure, as well as
//! allow for basic metadata changes, such as modifications to the permissions of
//! files.</p>
//! </li>
//! <li>
//! <p>Security: Support security applications are supported for users who have
//! additional security needs, such as antivirus or data loss prevention. The API
//! actions, along with AWS CloudTrail, allow these applications to detect when
//! changes occur in Amazon WorkDocs. Then, the application can take the necessary
//! actions and replace the target file. If the target file violates the policy, the
//! application can also choose to email the user.</p>
//! </li>
//! <li>
//! <p>eDiscovery/Analytics: General administrative applications are supported,
//! such as eDiscovery and analytics. These applications can choose to mimic or
//! record the actions in an Amazon WorkDocs site, along with AWS CloudTrail, to
//! replicate data for eDiscovery, backup, or analytical applications.</p>
//! </li>
//! </ul>
//! <p>All Amazon WorkDocs API actions are Amazon authenticated and certificate-signed.
//! They not only require the use of the AWS SDK, but also allow for the exclusive use of
//! IAM users and roles to help facilitate access, trust, and permission policies. By
//! creating a role and allowing an IAM user to access the Amazon WorkDocs site, the IAM
//! user gains full administrative visibility into the entire Amazon WorkDocs site (or as
//! set in the IAM policy). This includes, but is not limited to, the ability to modify file
//! permissions and upload any file to any user. This allows developers to perform the three
//! use cases above, as well as give users the ability to grant access on a selective basis
//! using the IAM model.</p>
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
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::DateTime;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("workdocs", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
