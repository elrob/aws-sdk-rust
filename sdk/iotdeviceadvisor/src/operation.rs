// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateSuiteDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_suite_definition`](crate::client::Client::create_suite_definition).
///
/// See [`crate::client::fluent_builders::CreateSuiteDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateSuiteDefinition {
    _private: (),
}
impl CreateSuiteDefinition {
    /// Creates a new builder-style object to manufacture [`CreateSuiteDefinitionInput`](crate::input::CreateSuiteDefinitionInput)
    pub fn builder() -> crate::input::create_suite_definition_input::Builder {
        crate::input::create_suite_definition_input::Builder::default()
    }
    /// Creates a new `CreateSuiteDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateSuiteDefinition {
    type Output = std::result::Result<
        crate::output::CreateSuiteDefinitionOutput,
        crate::error::CreateSuiteDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_suite_definition_error(response)
        } else {
            crate::operation_deser::parse_create_suite_definition_response(response)
        }
    }
}

/// Operation shape for `DeleteSuiteDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_suite_definition`](crate::client::Client::delete_suite_definition).
///
/// See [`crate::client::fluent_builders::DeleteSuiteDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSuiteDefinition {
    _private: (),
}
impl DeleteSuiteDefinition {
    /// Creates a new builder-style object to manufacture [`DeleteSuiteDefinitionInput`](crate::input::DeleteSuiteDefinitionInput)
    pub fn builder() -> crate::input::delete_suite_definition_input::Builder {
        crate::input::delete_suite_definition_input::Builder::default()
    }
    /// Creates a new `DeleteSuiteDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSuiteDefinition {
    type Output = std::result::Result<
        crate::output::DeleteSuiteDefinitionOutput,
        crate::error::DeleteSuiteDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_suite_definition_error(response)
        } else {
            crate::operation_deser::parse_delete_suite_definition_response(response)
        }
    }
}

/// Operation shape for `GetEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_endpoint`](crate::client::Client::get_endpoint).
///
/// See [`crate::client::fluent_builders::GetEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetEndpoint {
    _private: (),
}
impl GetEndpoint {
    /// Creates a new builder-style object to manufacture [`GetEndpointInput`](crate::input::GetEndpointInput)
    pub fn builder() -> crate::input::get_endpoint_input::Builder {
        crate::input::get_endpoint_input::Builder::default()
    }
    /// Creates a new `GetEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEndpoint {
    type Output =
        std::result::Result<crate::output::GetEndpointOutput, crate::error::GetEndpointError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_endpoint_error(response)
        } else {
            crate::operation_deser::parse_get_endpoint_response(response)
        }
    }
}

/// Operation shape for `GetSuiteDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_suite_definition`](crate::client::Client::get_suite_definition).
///
/// See [`crate::client::fluent_builders::GetSuiteDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetSuiteDefinition {
    _private: (),
}
impl GetSuiteDefinition {
    /// Creates a new builder-style object to manufacture [`GetSuiteDefinitionInput`](crate::input::GetSuiteDefinitionInput)
    pub fn builder() -> crate::input::get_suite_definition_input::Builder {
        crate::input::get_suite_definition_input::Builder::default()
    }
    /// Creates a new `GetSuiteDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSuiteDefinition {
    type Output = std::result::Result<
        crate::output::GetSuiteDefinitionOutput,
        crate::error::GetSuiteDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_suite_definition_error(response)
        } else {
            crate::operation_deser::parse_get_suite_definition_response(response)
        }
    }
}

/// Operation shape for `GetSuiteRun`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_suite_run`](crate::client::Client::get_suite_run).
///
/// See [`crate::client::fluent_builders::GetSuiteRun`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetSuiteRun {
    _private: (),
}
impl GetSuiteRun {
    /// Creates a new builder-style object to manufacture [`GetSuiteRunInput`](crate::input::GetSuiteRunInput)
    pub fn builder() -> crate::input::get_suite_run_input::Builder {
        crate::input::get_suite_run_input::Builder::default()
    }
    /// Creates a new `GetSuiteRun` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSuiteRun {
    type Output =
        std::result::Result<crate::output::GetSuiteRunOutput, crate::error::GetSuiteRunError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_suite_run_error(response)
        } else {
            crate::operation_deser::parse_get_suite_run_response(response)
        }
    }
}

/// Operation shape for `GetSuiteRunReport`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_suite_run_report`](crate::client::Client::get_suite_run_report).
///
/// See [`crate::client::fluent_builders::GetSuiteRunReport`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetSuiteRunReport {
    _private: (),
}
impl GetSuiteRunReport {
    /// Creates a new builder-style object to manufacture [`GetSuiteRunReportInput`](crate::input::GetSuiteRunReportInput)
    pub fn builder() -> crate::input::get_suite_run_report_input::Builder {
        crate::input::get_suite_run_report_input::Builder::default()
    }
    /// Creates a new `GetSuiteRunReport` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSuiteRunReport {
    type Output = std::result::Result<
        crate::output::GetSuiteRunReportOutput,
        crate::error::GetSuiteRunReportError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_suite_run_report_error(response)
        } else {
            crate::operation_deser::parse_get_suite_run_report_response(response)
        }
    }
}

/// Operation shape for `ListSuiteDefinitions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_suite_definitions`](crate::client::Client::list_suite_definitions).
///
/// See [`crate::client::fluent_builders::ListSuiteDefinitions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListSuiteDefinitions {
    _private: (),
}
impl ListSuiteDefinitions {
    /// Creates a new builder-style object to manufacture [`ListSuiteDefinitionsInput`](crate::input::ListSuiteDefinitionsInput)
    pub fn builder() -> crate::input::list_suite_definitions_input::Builder {
        crate::input::list_suite_definitions_input::Builder::default()
    }
    /// Creates a new `ListSuiteDefinitions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSuiteDefinitions {
    type Output = std::result::Result<
        crate::output::ListSuiteDefinitionsOutput,
        crate::error::ListSuiteDefinitionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_suite_definitions_error(response)
        } else {
            crate::operation_deser::parse_list_suite_definitions_response(response)
        }
    }
}

/// Operation shape for `ListSuiteRuns`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_suite_runs`](crate::client::Client::list_suite_runs).
///
/// See [`crate::client::fluent_builders::ListSuiteRuns`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListSuiteRuns {
    _private: (),
}
impl ListSuiteRuns {
    /// Creates a new builder-style object to manufacture [`ListSuiteRunsInput`](crate::input::ListSuiteRunsInput)
    pub fn builder() -> crate::input::list_suite_runs_input::Builder {
        crate::input::list_suite_runs_input::Builder::default()
    }
    /// Creates a new `ListSuiteRuns` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSuiteRuns {
    type Output =
        std::result::Result<crate::output::ListSuiteRunsOutput, crate::error::ListSuiteRunsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_suite_runs_error(response)
        } else {
            crate::operation_deser::parse_list_suite_runs_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `StartSuiteRun`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_suite_run`](crate::client::Client::start_suite_run).
///
/// See [`crate::client::fluent_builders::StartSuiteRun`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartSuiteRun {
    _private: (),
}
impl StartSuiteRun {
    /// Creates a new builder-style object to manufacture [`StartSuiteRunInput`](crate::input::StartSuiteRunInput)
    pub fn builder() -> crate::input::start_suite_run_input::Builder {
        crate::input::start_suite_run_input::Builder::default()
    }
    /// Creates a new `StartSuiteRun` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartSuiteRun {
    type Output =
        std::result::Result<crate::output::StartSuiteRunOutput, crate::error::StartSuiteRunError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_suite_run_error(response)
        } else {
            crate::operation_deser::parse_start_suite_run_response(response)
        }
    }
}

/// Operation shape for `StopSuiteRun`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`stop_suite_run`](crate::client::Client::stop_suite_run).
///
/// See [`crate::client::fluent_builders::StopSuiteRun`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopSuiteRun {
    _private: (),
}
impl StopSuiteRun {
    /// Creates a new builder-style object to manufacture [`StopSuiteRunInput`](crate::input::StopSuiteRunInput)
    pub fn builder() -> crate::input::stop_suite_run_input::Builder {
        crate::input::stop_suite_run_input::Builder::default()
    }
    /// Creates a new `StopSuiteRun` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopSuiteRun {
    type Output =
        std::result::Result<crate::output::StopSuiteRunOutput, crate::error::StopSuiteRunError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_suite_run_error(response)
        } else {
            crate::operation_deser::parse_stop_suite_run_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateSuiteDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_suite_definition`](crate::client::Client::update_suite_definition).
///
/// See [`crate::client::fluent_builders::UpdateSuiteDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateSuiteDefinition {
    _private: (),
}
impl UpdateSuiteDefinition {
    /// Creates a new builder-style object to manufacture [`UpdateSuiteDefinitionInput`](crate::input::UpdateSuiteDefinitionInput)
    pub fn builder() -> crate::input::update_suite_definition_input::Builder {
        crate::input::update_suite_definition_input::Builder::default()
    }
    /// Creates a new `UpdateSuiteDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateSuiteDefinition {
    type Output = std::result::Result<
        crate::output::UpdateSuiteDefinitionOutput,
        crate::error::UpdateSuiteDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_suite_definition_error(response)
        } else {
            crate::operation_deser::parse_update_suite_definition_response(response)
        }
    }
}
