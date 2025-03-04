#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>AWS OpsWorks</fullname>
//! <p>Welcome to the <i>AWS OpsWorks Stacks API Reference</i>. This guide provides descriptions, syntax, and
//! usage examples for AWS OpsWorks Stacks actions and data types, including common parameters and error
//! codes. </p>
//! <p>AWS OpsWorks Stacks is an application management service that provides an integrated experience for
//! overseeing the complete application lifecycle. For information about this product, go to the
//! <a href="http://aws.amazon.com/opsworks/">AWS OpsWorks</a> details page. </p>
//!
//! <p>
//! <b>SDKs and CLI</b>
//! </p>
//! <p>The most common way to use the AWS OpsWorks Stacks API is by using the AWS Command Line Interface (CLI) or by using one of the AWS SDKs to implement applications in your preferred language. For more information, see:</p>
//! <ul>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-welcome.html">AWS CLI</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/AWSJavaSDK/latest/javadoc/com/amazonaws/services/opsworks/AWSOpsWorksClient.html">AWS SDK for Java</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/sdkfornet/latest/apidocs/html/N_Amazon_OpsWorks.htm">AWS SDK for
//! .NET</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/aws-sdk-php-2/latest/class-Aws.OpsWorks.OpsWorksClient.html">AWS
//! SDK for PHP 2</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <a href="http://docs.aws.amazon.com/sdkforruby/api/">AWS SDK for Ruby</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <a href="http://aws.amazon.com/documentation/sdkforjavascript/">AWS SDK for Node.js</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <a href="http://docs.pythonboto.org/en/latest/ref/opsworks.html">AWS SDK for
//! Python(Boto)</a>
//! </p>
//! </li>
//! </ul>
//!
//! <p>
//! <b>Endpoints</b>
//! </p>
//! <p>AWS OpsWorks Stacks supports the following endpoints, all HTTPS. You must connect to one of the following endpoints. Stacks
//! can only be accessed or managed within the endpoint in which they are created.</p>
//! <ul>
//! <li>
//! <p>opsworks.us-east-1.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks.us-east-2.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks.us-west-1.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks.us-west-2.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks.ca-central-1.amazonaws.com (API only; not available in the AWS console)</p>
//! </li>
//! <li>
//! <p>opsworks.eu-west-1.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks.eu-west-2.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks.eu-west-3.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks.eu-central-1.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks.ap-northeast-1.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks.ap-northeast-2.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks.ap-south-1.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks.ap-southeast-1.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks.ap-southeast-2.amazonaws.com</p>
//! </li>
//! <li>
//! <p>opsworks.sa-east-1.amazonaws.com</p>
//! </li>
//! </ul>
//! <p>
//! <b>Chef Versions</b>
//! </p>
//! <p>When you call <a>CreateStack</a>, <a>CloneStack</a>, or <a>UpdateStack</a> we recommend you
//! use the <code>ConfigurationManager</code> parameter to specify the Chef version.
//! The recommended and default value for Linux stacks is currently 12. Windows stacks use Chef 12.2. For more information,
//! see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook-chef11.html">Chef Versions</a>.</p>
//! <note>
//! <p>You can specify Chef 12, 11.10, or 11.4 for your Linux stack. We recommend migrating your existing Linux stacks to Chef 12 as soon as possible.</p>
//! </note>
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
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("opsworks", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
