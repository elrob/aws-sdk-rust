// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) client: aws_smithy_client::Client<
        aws_smithy_client::erase::DynConnector,
        aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
    >,
    pub(crate) conf: crate::Config,
}

/// Client for AWSMarketplace Metering
///
/// Client for invoking operations on AWSMarketplace Metering. Each operation on AWSMarketplace Metering is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_marketplacemetering::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::RetryConfig;
/// # async fn docs() {
///     let shared_config = aws_config::load_from_env().await;
///     let config = aws_sdk_marketplacemetering::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_marketplacemetering::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client {
    handle: std::sync::Arc<Handle>,
}

impl std::clone::Clone for Client {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl
    From<
        aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    > for Client
{
    fn from(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    ) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl Client {
    /// Creates a client with the given service configuration.
    pub fn with_config(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
        conf: crate::Config,
    ) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    /// Constructs a fluent builder for the [`BatchMeterUsage`](crate::client::fluent_builders::BatchMeterUsage) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`usage_records(Vec<UsageRecord>)`](crate::client::fluent_builders::BatchMeterUsage::usage_records) / [`set_usage_records(Option<Vec<UsageRecord>>)`](crate::client::fluent_builders::BatchMeterUsage::set_usage_records): <p>The set of <code>UsageRecords</code> to submit. <code>BatchMeterUsage</code> accepts up to 25 <code>UsageRecords</code> at a time.</p>
    ///   - [`product_code(impl Into<String>)`](crate::client::fluent_builders::BatchMeterUsage::product_code) / [`set_product_code(Option<String>)`](crate::client::fluent_builders::BatchMeterUsage::set_product_code): <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
    /// - On success, responds with [`BatchMeterUsageOutput`](crate::output::BatchMeterUsageOutput) with field(s):
    ///   - [`results(Option<Vec<UsageRecordResult>>)`](crate::output::BatchMeterUsageOutput::results): <p>Contains all <code>UsageRecords</code> processed by <code>BatchMeterUsage</code>. These records were either honored by AWS Marketplace Metering Service or were invalid. Invalid records should be fixed before being resubmitted.</p>
    ///   - [`unprocessed_records(Option<Vec<UsageRecord>>)`](crate::output::BatchMeterUsageOutput::unprocessed_records): <p>Contains all <code>UsageRecords</code> that were not processed by <code>BatchMeterUsage</code>. This is a list of <code>UsageRecords</code>. You can retry the failed request by making another <code>BatchMeterUsage</code> call with this list as input in the <code>BatchMeterUsageRequest</code>.</p>
    /// - On failure, responds with [`SdkError<BatchMeterUsageError>`](crate::error::BatchMeterUsageError)
    pub fn batch_meter_usage(&self) -> fluent_builders::BatchMeterUsage {
        fluent_builders::BatchMeterUsage::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`MeterUsage`](crate::client::fluent_builders::MeterUsage) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`product_code(impl Into<String>)`](crate::client::fluent_builders::MeterUsage::product_code) / [`set_product_code(Option<String>)`](crate::client::fluent_builders::MeterUsage::set_product_code): <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
    ///   - [`timestamp(DateTime)`](crate::client::fluent_builders::MeterUsage::timestamp) / [`set_timestamp(Option<DateTime>)`](crate::client::fluent_builders::MeterUsage::set_timestamp): <p>Timestamp, in UTC, for which the usage is being reported. Your application can meter usage for up to one hour in the past. Make sure the <code>timestamp</code> value is not before the start of the software usage.</p>
    ///   - [`usage_dimension(impl Into<String>)`](crate::client::fluent_builders::MeterUsage::usage_dimension) / [`set_usage_dimension(Option<String>)`](crate::client::fluent_builders::MeterUsage::set_usage_dimension): <p>It will be one of the fcp dimension name provided during the publishing of the product.</p>
    ///   - [`usage_quantity(i32)`](crate::client::fluent_builders::MeterUsage::usage_quantity) / [`set_usage_quantity(Option<i32>)`](crate::client::fluent_builders::MeterUsage::set_usage_quantity): <p>Consumption value for the hour. Defaults to <code>0</code> if not specified.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::MeterUsage::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::MeterUsage::set_dry_run): <p>Checks whether you have the permissions required for the action, but does not make the request. If you have the permissions, the request returns <code>DryRunOperation</code>; otherwise, it returns <code>UnauthorizedException</code>. Defaults to <code>false</code> if not specified.</p>
    ///   - [`usage_allocations(Vec<UsageAllocation>)`](crate::client::fluent_builders::MeterUsage::usage_allocations) / [`set_usage_allocations(Option<Vec<UsageAllocation>>)`](crate::client::fluent_builders::MeterUsage::set_usage_allocations): <p>The set of <code>UsageAllocations</code> to submit.</p>  <p>The sum of all <code>UsageAllocation</code> quantities must equal the <code>UsageQuantity</code> of the <code>MeterUsage</code> request, and each <code>UsageAllocation</code> must have a unique set of tags (include no tags).</p>
    /// - On success, responds with [`MeterUsageOutput`](crate::output::MeterUsageOutput) with field(s):
    ///   - [`metering_record_id(Option<String>)`](crate::output::MeterUsageOutput::metering_record_id): <p>Metering record id.</p>
    /// - On failure, responds with [`SdkError<MeterUsageError>`](crate::error::MeterUsageError)
    pub fn meter_usage(&self) -> fluent_builders::MeterUsage {
        fluent_builders::MeterUsage::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`RegisterUsage`](crate::client::fluent_builders::RegisterUsage) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`product_code(impl Into<String>)`](crate::client::fluent_builders::RegisterUsage::product_code) / [`set_product_code(Option<String>)`](crate::client::fluent_builders::RegisterUsage::set_product_code): <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
    ///   - [`public_key_version(i32)`](crate::client::fluent_builders::RegisterUsage::public_key_version) / [`set_public_key_version(Option<i32>)`](crate::client::fluent_builders::RegisterUsage::set_public_key_version): <p>Public Key Version provided by AWS Marketplace</p>
    ///   - [`nonce(impl Into<String>)`](crate::client::fluent_builders::RegisterUsage::nonce) / [`set_nonce(Option<String>)`](crate::client::fluent_builders::RegisterUsage::set_nonce): <p>(Optional) To scope down the registration to a specific running software instance and guard against replay attacks.</p>
    /// - On success, responds with [`RegisterUsageOutput`](crate::output::RegisterUsageOutput) with field(s):
    ///   - [`public_key_rotation_timestamp(Option<DateTime>)`](crate::output::RegisterUsageOutput::public_key_rotation_timestamp): <p>(Optional) Only included when public key version has expired</p>
    ///   - [`signature(Option<String>)`](crate::output::RegisterUsageOutput::signature): <p>JWT Token</p>
    /// - On failure, responds with [`SdkError<RegisterUsageError>`](crate::error::RegisterUsageError)
    pub fn register_usage(&self) -> fluent_builders::RegisterUsage {
        fluent_builders::RegisterUsage::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`ResolveCustomer`](crate::client::fluent_builders::ResolveCustomer) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`registration_token(impl Into<String>)`](crate::client::fluent_builders::ResolveCustomer::registration_token) / [`set_registration_token(Option<String>)`](crate::client::fluent_builders::ResolveCustomer::set_registration_token): <p>When a buyer visits your website during the registration process, the buyer submits a registration token through the browser. The registration token is resolved to obtain a <code>CustomerIdentifier</code> along with the <code>CustomerAWSAccountId</code> and <code>ProductCode</code>.</p>
    /// - On success, responds with [`ResolveCustomerOutput`](crate::output::ResolveCustomerOutput) with field(s):
    ///   - [`customer_identifier(Option<String>)`](crate::output::ResolveCustomerOutput::customer_identifier): <p>The <code>CustomerIdentifier</code> is used to identify an individual customer in your application. Calls to <code>BatchMeterUsage</code> require <code>CustomerIdentifiers</code> for each <code>UsageRecord</code>.</p>
    ///   - [`product_code(Option<String>)`](crate::output::ResolveCustomerOutput::product_code): <p>The product code is returned to confirm that the buyer is registering for your product. Subsequent <code>BatchMeterUsage</code> calls should be made using this product code.</p>
    ///   - [`customer_aws_account_id(Option<String>)`](crate::output::ResolveCustomerOutput::customer_aws_account_id): <p>The <code>CustomerAWSAccountId</code> provides the AWS account ID associated with the <code>CustomerIdentifier</code> for the individual customer.</p>
    /// - On failure, responds with [`SdkError<ResolveCustomerError>`](crate::error::ResolveCustomerError)
    pub fn resolve_customer(&self) -> fluent_builders::ResolveCustomer {
        fluent_builders::ResolveCustomer::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `BatchMeterUsage`.
    ///
    /// <p> <code>BatchMeterUsage</code> is called from a SaaS application listed on AWS Marketplace to post metering records for a set of customers.</p>
    /// <p>For identical requests, the API is idempotent; requests can be retried with the same records or a subset of the input records.</p>
    /// <p>Every request to <code>BatchMeterUsage</code> is for one product. If you need to meter usage for multiple products, you must make multiple calls to <code>BatchMeterUsage</code>.</p>
    /// <p>Usage records are expected to be submitted as quickly as possible after the event that is being recorded, and are not accepted more than 6 hours after the event.</p>
    /// <p> <code>BatchMeterUsage</code> can process up to 25 <code>UsageRecords</code> at a time.</p>
    /// <p>A <code>UsageRecord</code> can optionally include multiple usage allocations, to provide customers with usage data split into buckets by tags that you define (or allow the customer to define).</p>
    /// <p> <code>BatchMeterUsage</code> returns a list of <code>UsageRecordResult</code> objects, showing the result for each <code>UsageRecord</code>, as well as a list of <code>UnprocessedRecords</code>, indicating errors in the service side that you should retry.</p>
    /// <p> <code>BatchMeterUsage</code> requests must be less than 1MB in size.</p> <note>
    /// <p>For an example of using <code>BatchMeterUsage</code>, see <a href="https://docs.aws.amazon.com/marketplace/latest/userguide/saas-code-examples.html#saas-batchmeterusage-example"> BatchMeterUsage code example</a> in the <i>AWS Marketplace Seller Guide</i>.</p>
    /// </note>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct BatchMeterUsage {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::batch_meter_usage_input::Builder,
    }
    impl BatchMeterUsage {
        /// Creates a new `BatchMeterUsage`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::BatchMeterUsageOutput,
            aws_smithy_http::result::SdkError<crate::error::BatchMeterUsageError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// Appends an item to `UsageRecords`.
        ///
        /// To override the contents of this collection use [`set_usage_records`](Self::set_usage_records).
        ///
        /// <p>The set of <code>UsageRecords</code> to submit. <code>BatchMeterUsage</code> accepts up to 25 <code>UsageRecords</code> at a time.</p>
        pub fn usage_records(mut self, input: crate::model::UsageRecord) -> Self {
            self.inner = self.inner.usage_records(input);
            self
        }
        /// <p>The set of <code>UsageRecords</code> to submit. <code>BatchMeterUsage</code> accepts up to 25 <code>UsageRecords</code> at a time.</p>
        pub fn set_usage_records(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::UsageRecord>>,
        ) -> Self {
            self.inner = self.inner.set_usage_records(input);
            self
        }
        /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
        pub fn product_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.product_code(input.into());
            self
        }
        /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
        pub fn set_product_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_product_code(input);
            self
        }
    }
    /// Fluent builder constructing a request to `MeterUsage`.
    ///
    /// <p>API to emit metering records. For identical requests, the API is idempotent. It simply returns the metering record ID.</p>
    /// <p> <code>MeterUsage</code> is authenticated on the buyer's AWS account using credentials from the EC2 instance, ECS task, or EKS pod.</p>
    /// <p> <code>MeterUsage</code> can optionally include multiple usage allocations, to provide customers with usage data split into buckets by tags that you define (or allow the customer to define).</p>
    /// <p>Usage records are expected to be submitted as quickly as possible after the event that is being recorded, and are not accepted more than 6 hours after the event.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct MeterUsage {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::meter_usage_input::Builder,
    }
    impl MeterUsage {
        /// Creates a new `MeterUsage`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::MeterUsageOutput,
            aws_smithy_http::result::SdkError<crate::error::MeterUsageError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
        pub fn product_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.product_code(input.into());
            self
        }
        /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
        pub fn set_product_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_product_code(input);
            self
        }
        /// <p>Timestamp, in UTC, for which the usage is being reported. Your application can meter usage for up to one hour in the past. Make sure the <code>timestamp</code> value is not before the start of the software usage.</p>
        pub fn timestamp(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.inner = self.inner.timestamp(input);
            self
        }
        /// <p>Timestamp, in UTC, for which the usage is being reported. Your application can meter usage for up to one hour in the past. Make sure the <code>timestamp</code> value is not before the start of the software usage.</p>
        pub fn set_timestamp(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.inner = self.inner.set_timestamp(input);
            self
        }
        /// <p>It will be one of the fcp dimension name provided during the publishing of the product.</p>
        pub fn usage_dimension(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.usage_dimension(input.into());
            self
        }
        /// <p>It will be one of the fcp dimension name provided during the publishing of the product.</p>
        pub fn set_usage_dimension(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_usage_dimension(input);
            self
        }
        /// <p>Consumption value for the hour. Defaults to <code>0</code> if not specified.</p>
        pub fn usage_quantity(mut self, input: i32) -> Self {
            self.inner = self.inner.usage_quantity(input);
            self
        }
        /// <p>Consumption value for the hour. Defaults to <code>0</code> if not specified.</p>
        pub fn set_usage_quantity(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_usage_quantity(input);
            self
        }
        /// <p>Checks whether you have the permissions required for the action, but does not make the request. If you have the permissions, the request returns <code>DryRunOperation</code>; otherwise, it returns <code>UnauthorizedException</code>. Defaults to <code>false</code> if not specified.</p>
        pub fn dry_run(mut self, input: bool) -> Self {
            self.inner = self.inner.dry_run(input);
            self
        }
        /// <p>Checks whether you have the permissions required for the action, but does not make the request. If you have the permissions, the request returns <code>DryRunOperation</code>; otherwise, it returns <code>UnauthorizedException</code>. Defaults to <code>false</code> if not specified.</p>
        pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_dry_run(input);
            self
        }
        /// Appends an item to `UsageAllocations`.
        ///
        /// To override the contents of this collection use [`set_usage_allocations`](Self::set_usage_allocations).
        ///
        /// <p>The set of <code>UsageAllocations</code> to submit.</p>
        /// <p>The sum of all <code>UsageAllocation</code> quantities must equal the <code>UsageQuantity</code> of the <code>MeterUsage</code> request, and each <code>UsageAllocation</code> must have a unique set of tags (include no tags).</p>
        pub fn usage_allocations(mut self, input: crate::model::UsageAllocation) -> Self {
            self.inner = self.inner.usage_allocations(input);
            self
        }
        /// <p>The set of <code>UsageAllocations</code> to submit.</p>
        /// <p>The sum of all <code>UsageAllocation</code> quantities must equal the <code>UsageQuantity</code> of the <code>MeterUsage</code> request, and each <code>UsageAllocation</code> must have a unique set of tags (include no tags).</p>
        pub fn set_usage_allocations(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::UsageAllocation>>,
        ) -> Self {
            self.inner = self.inner.set_usage_allocations(input);
            self
        }
    }
    /// Fluent builder constructing a request to `RegisterUsage`.
    ///
    /// <p>Paid container software products sold through AWS Marketplace must integrate with the AWS Marketplace Metering Service and call the <code>RegisterUsage</code> operation for software entitlement and metering. Free and BYOL products for Amazon ECS or Amazon EKS aren't required to call <code>RegisterUsage</code>, but you may choose to do so if you would like to receive usage data in your seller reports. The sections below explain the behavior of <code>RegisterUsage</code>. <code>RegisterUsage</code> performs two primary functions: metering and entitlement.</p>
    /// <ul>
    /// <li> <p> <i>Entitlement</i>: <code>RegisterUsage</code> allows you to verify that the customer running your paid software is subscribed to your product on AWS Marketplace, enabling you to guard against unauthorized use. Your container image that integrates with <code>RegisterUsage</code> is only required to guard against unauthorized use at container startup, as such a <code>CustomerNotSubscribedException</code> or <code>PlatformNotSupportedException</code> will only be thrown on the initial call to <code>RegisterUsage</code>. Subsequent calls from the same Amazon ECS task instance (e.g. task-id) or Amazon EKS pod will not throw a <code>CustomerNotSubscribedException</code>, even if the customer unsubscribes while the Amazon ECS task or Amazon EKS pod is still running.</p> </li>
    /// <li> <p> <i>Metering</i>: <code>RegisterUsage</code> meters software use per ECS task, per hour, or per pod for Amazon EKS with usage prorated to the second. A minimum of 1 minute of usage applies to tasks that are short lived. For example, if a customer has a 10 node Amazon ECS or Amazon EKS cluster and a service configured as a Daemon Set, then Amazon ECS or Amazon EKS will launch a task on all 10 cluster nodes and the customer will be charged: (10 * hourly_rate). Metering for software use is automatically handled by the AWS Marketplace Metering Control Plane -- your software is not required to perform any metering specific actions, other than call <code>RegisterUsage</code> once for metering of software use to commence. The AWS Marketplace Metering Control Plane will also continue to bill customers for running ECS tasks and Amazon EKS pods, regardless of the customers subscription state, removing the need for your software to perform entitlement checks at runtime.</p> </li>
    /// </ul>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct RegisterUsage {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::register_usage_input::Builder,
    }
    impl RegisterUsage {
        /// Creates a new `RegisterUsage`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::RegisterUsageOutput,
            aws_smithy_http::result::SdkError<crate::error::RegisterUsageError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
        pub fn product_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.product_code(input.into());
            self
        }
        /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
        pub fn set_product_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_product_code(input);
            self
        }
        /// <p>Public Key Version provided by AWS Marketplace</p>
        pub fn public_key_version(mut self, input: i32) -> Self {
            self.inner = self.inner.public_key_version(input);
            self
        }
        /// <p>Public Key Version provided by AWS Marketplace</p>
        pub fn set_public_key_version(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_public_key_version(input);
            self
        }
        /// <p>(Optional) To scope down the registration to a specific running software instance and guard against replay attacks.</p>
        pub fn nonce(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.nonce(input.into());
            self
        }
        /// <p>(Optional) To scope down the registration to a specific running software instance and guard against replay attacks.</p>
        pub fn set_nonce(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_nonce(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ResolveCustomer`.
    ///
    /// <p> <code>ResolveCustomer</code> is called by a SaaS application during the registration process. When a buyer visits your website during the registration process, the buyer submits a registration token through their browser. The registration token is resolved through this API to obtain a <code>CustomerIdentifier</code> along with the <code>CustomerAWSAccountId</code> and <code>ProductCode</code>.</p> <note>
    /// <p>The API needs to called from the seller account id used to publish the SaaS application to successfully resolve the token.</p>
    /// <p>For an example of using <code>ResolveCustomer</code>, see <a href="https://docs.aws.amazon.com/marketplace/latest/userguide/saas-code-examples.html#saas-resolvecustomer-example"> ResolveCustomer code example</a> in the <i>AWS Marketplace Seller Guide</i>.</p>
    /// </note>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ResolveCustomer {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::resolve_customer_input::Builder,
    }
    impl ResolveCustomer {
        /// Creates a new `ResolveCustomer`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ResolveCustomerOutput,
            aws_smithy_http::result::SdkError<crate::error::ResolveCustomerError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>When a buyer visits your website during the registration process, the buyer submits a registration token through the browser. The registration token is resolved to obtain a <code>CustomerIdentifier</code> along with the <code>CustomerAWSAccountId</code> and <code>ProductCode</code>.</p>
        pub fn registration_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.registration_token(input.into());
            self
        }
        /// <p>When a buyer visits your website during the registration process, the buyer submits a registration token through the browser. The registration token is resolved to obtain a <code>CustomerIdentifier</code> along with the <code>CustomerAWSAccountId</code> and <code>ProductCode</code>.</p>
        pub fn set_registration_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_registration_token(input);
            self
        }
    }
}

impl Client {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn<C, E>(conf: crate::Config, conn: C) -> Self
    where
        C: aws_smithy_client::bounds::SmithyConnector<Error = E> + Send + 'static,
        E: Into<aws_smithy_http::result::ConnectorError>,
    {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::new()
            .connector(aws_smithy_client::erase::DynConnector::new(conn))
            .middleware(aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ));
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::dyn_https().middleware(
            aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ),
        );
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        // the builder maintains a try-state. To avoid suppressing the warning when sleep is unset,
        // only set it if we actually have a sleep impl.
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
