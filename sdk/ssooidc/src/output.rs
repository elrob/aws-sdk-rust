// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StartDeviceAuthorizationOutput {
    /// <p>The short-lived code that is used by the device when polling for a session token.</p>
    pub device_code: std::option::Option<std::string::String>,
    /// <p>A one-time user verification code. This is needed to authorize an in-use device.</p>
    pub user_code: std::option::Option<std::string::String>,
    /// <p>The URI of the verification page that takes the <code>userCode</code> to authorize the device.</p>
    pub verification_uri: std::option::Option<std::string::String>,
    /// <p>An alternate URL that the client can use to automatically launch a browser. This process skips the manual step in which the user visits the verification page and enters their code.</p>
    pub verification_uri_complete: std::option::Option<std::string::String>,
    /// <p>Indicates the number of seconds in which the verification code will become invalid.</p>
    pub expires_in: i32,
    /// <p>Indicates the number of seconds the client must wait between attempts when polling for a session.</p>
    pub interval: i32,
}
impl StartDeviceAuthorizationOutput {
    /// <p>The short-lived code that is used by the device when polling for a session token.</p>
    pub fn device_code(&self) -> std::option::Option<&str> {
        self.device_code.as_deref()
    }
    /// <p>A one-time user verification code. This is needed to authorize an in-use device.</p>
    pub fn user_code(&self) -> std::option::Option<&str> {
        self.user_code.as_deref()
    }
    /// <p>The URI of the verification page that takes the <code>userCode</code> to authorize the device.</p>
    pub fn verification_uri(&self) -> std::option::Option<&str> {
        self.verification_uri.as_deref()
    }
    /// <p>An alternate URL that the client can use to automatically launch a browser. This process skips the manual step in which the user visits the verification page and enters their code.</p>
    pub fn verification_uri_complete(&self) -> std::option::Option<&str> {
        self.verification_uri_complete.as_deref()
    }
    /// <p>Indicates the number of seconds in which the verification code will become invalid.</p>
    pub fn expires_in(&self) -> i32 {
        self.expires_in
    }
    /// <p>Indicates the number of seconds the client must wait between attempts when polling for a session.</p>
    pub fn interval(&self) -> i32 {
        self.interval
    }
}
impl std::fmt::Debug for StartDeviceAuthorizationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StartDeviceAuthorizationOutput");
        formatter.field("device_code", &self.device_code);
        formatter.field("user_code", &self.user_code);
        formatter.field("verification_uri", &self.verification_uri);
        formatter.field("verification_uri_complete", &self.verification_uri_complete);
        formatter.field("expires_in", &self.expires_in);
        formatter.field("interval", &self.interval);
        formatter.finish()
    }
}
/// See [`StartDeviceAuthorizationOutput`](crate::output::StartDeviceAuthorizationOutput)
pub mod start_device_authorization_output {
    /// A builder for [`StartDeviceAuthorizationOutput`](crate::output::StartDeviceAuthorizationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) device_code: std::option::Option<std::string::String>,
        pub(crate) user_code: std::option::Option<std::string::String>,
        pub(crate) verification_uri: std::option::Option<std::string::String>,
        pub(crate) verification_uri_complete: std::option::Option<std::string::String>,
        pub(crate) expires_in: std::option::Option<i32>,
        pub(crate) interval: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The short-lived code that is used by the device when polling for a session token.</p>
        pub fn device_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.device_code = Some(input.into());
            self
        }
        /// <p>The short-lived code that is used by the device when polling for a session token.</p>
        pub fn set_device_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.device_code = input;
            self
        }
        /// <p>A one-time user verification code. This is needed to authorize an in-use device.</p>
        pub fn user_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.user_code = Some(input.into());
            self
        }
        /// <p>A one-time user verification code. This is needed to authorize an in-use device.</p>
        pub fn set_user_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user_code = input;
            self
        }
        /// <p>The URI of the verification page that takes the <code>userCode</code> to authorize the device.</p>
        pub fn verification_uri(mut self, input: impl Into<std::string::String>) -> Self {
            self.verification_uri = Some(input.into());
            self
        }
        /// <p>The URI of the verification page that takes the <code>userCode</code> to authorize the device.</p>
        pub fn set_verification_uri(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.verification_uri = input;
            self
        }
        /// <p>An alternate URL that the client can use to automatically launch a browser. This process skips the manual step in which the user visits the verification page and enters their code.</p>
        pub fn verification_uri_complete(mut self, input: impl Into<std::string::String>) -> Self {
            self.verification_uri_complete = Some(input.into());
            self
        }
        /// <p>An alternate URL that the client can use to automatically launch a browser. This process skips the manual step in which the user visits the verification page and enters their code.</p>
        pub fn set_verification_uri_complete(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.verification_uri_complete = input;
            self
        }
        /// <p>Indicates the number of seconds in which the verification code will become invalid.</p>
        pub fn expires_in(mut self, input: i32) -> Self {
            self.expires_in = Some(input);
            self
        }
        /// <p>Indicates the number of seconds in which the verification code will become invalid.</p>
        pub fn set_expires_in(mut self, input: std::option::Option<i32>) -> Self {
            self.expires_in = input;
            self
        }
        /// <p>Indicates the number of seconds the client must wait between attempts when polling for a session.</p>
        pub fn interval(mut self, input: i32) -> Self {
            self.interval = Some(input);
            self
        }
        /// <p>Indicates the number of seconds the client must wait between attempts when polling for a session.</p>
        pub fn set_interval(mut self, input: std::option::Option<i32>) -> Self {
            self.interval = input;
            self
        }
        /// Consumes the builder and constructs a [`StartDeviceAuthorizationOutput`](crate::output::StartDeviceAuthorizationOutput)
        pub fn build(self) -> crate::output::StartDeviceAuthorizationOutput {
            crate::output::StartDeviceAuthorizationOutput {
                device_code: self.device_code,
                user_code: self.user_code,
                verification_uri: self.verification_uri,
                verification_uri_complete: self.verification_uri_complete,
                expires_in: self.expires_in.unwrap_or_default(),
                interval: self.interval.unwrap_or_default(),
            }
        }
    }
}
impl StartDeviceAuthorizationOutput {
    /// Creates a new builder-style object to manufacture [`StartDeviceAuthorizationOutput`](crate::output::StartDeviceAuthorizationOutput)
    pub fn builder() -> crate::output::start_device_authorization_output::Builder {
        crate::output::start_device_authorization_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RegisterClientOutput {
    /// <p>The unique identifier string for each client. This client uses this identifier to get authenticated by the service in subsequent calls.</p>
    pub client_id: std::option::Option<std::string::String>,
    /// <p>A secret string generated for the client. The client will use this string to get authenticated by the service in subsequent calls.</p>
    pub client_secret: std::option::Option<std::string::String>,
    /// <p>Indicates the time at which the <code>clientId</code> and <code>clientSecret</code> were issued.</p>
    pub client_id_issued_at: i64,
    /// <p>Indicates the time at which the <code>clientId</code> and <code>clientSecret</code> will become invalid.</p>
    pub client_secret_expires_at: i64,
    /// <p>The endpoint where the client can request authorization.</p>
    pub authorization_endpoint: std::option::Option<std::string::String>,
    /// <p>The endpoint where the client can get an access token.</p>
    pub token_endpoint: std::option::Option<std::string::String>,
}
impl RegisterClientOutput {
    /// <p>The unique identifier string for each client. This client uses this identifier to get authenticated by the service in subsequent calls.</p>
    pub fn client_id(&self) -> std::option::Option<&str> {
        self.client_id.as_deref()
    }
    /// <p>A secret string generated for the client. The client will use this string to get authenticated by the service in subsequent calls.</p>
    pub fn client_secret(&self) -> std::option::Option<&str> {
        self.client_secret.as_deref()
    }
    /// <p>Indicates the time at which the <code>clientId</code> and <code>clientSecret</code> were issued.</p>
    pub fn client_id_issued_at(&self) -> i64 {
        self.client_id_issued_at
    }
    /// <p>Indicates the time at which the <code>clientId</code> and <code>clientSecret</code> will become invalid.</p>
    pub fn client_secret_expires_at(&self) -> i64 {
        self.client_secret_expires_at
    }
    /// <p>The endpoint where the client can request authorization.</p>
    pub fn authorization_endpoint(&self) -> std::option::Option<&str> {
        self.authorization_endpoint.as_deref()
    }
    /// <p>The endpoint where the client can get an access token.</p>
    pub fn token_endpoint(&self) -> std::option::Option<&str> {
        self.token_endpoint.as_deref()
    }
}
impl std::fmt::Debug for RegisterClientOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RegisterClientOutput");
        formatter.field("client_id", &self.client_id);
        formatter.field("client_secret", &self.client_secret);
        formatter.field("client_id_issued_at", &self.client_id_issued_at);
        formatter.field("client_secret_expires_at", &self.client_secret_expires_at);
        formatter.field("authorization_endpoint", &self.authorization_endpoint);
        formatter.field("token_endpoint", &self.token_endpoint);
        formatter.finish()
    }
}
/// See [`RegisterClientOutput`](crate::output::RegisterClientOutput)
pub mod register_client_output {
    /// A builder for [`RegisterClientOutput`](crate::output::RegisterClientOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) client_id: std::option::Option<std::string::String>,
        pub(crate) client_secret: std::option::Option<std::string::String>,
        pub(crate) client_id_issued_at: std::option::Option<i64>,
        pub(crate) client_secret_expires_at: std::option::Option<i64>,
        pub(crate) authorization_endpoint: std::option::Option<std::string::String>,
        pub(crate) token_endpoint: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The unique identifier string for each client. This client uses this identifier to get authenticated by the service in subsequent calls.</p>
        pub fn client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_id = Some(input.into());
            self
        }
        /// <p>The unique identifier string for each client. This client uses this identifier to get authenticated by the service in subsequent calls.</p>
        pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.client_id = input;
            self
        }
        /// <p>A secret string generated for the client. The client will use this string to get authenticated by the service in subsequent calls.</p>
        pub fn client_secret(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_secret = Some(input.into());
            self
        }
        /// <p>A secret string generated for the client. The client will use this string to get authenticated by the service in subsequent calls.</p>
        pub fn set_client_secret(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.client_secret = input;
            self
        }
        /// <p>Indicates the time at which the <code>clientId</code> and <code>clientSecret</code> were issued.</p>
        pub fn client_id_issued_at(mut self, input: i64) -> Self {
            self.client_id_issued_at = Some(input);
            self
        }
        /// <p>Indicates the time at which the <code>clientId</code> and <code>clientSecret</code> were issued.</p>
        pub fn set_client_id_issued_at(mut self, input: std::option::Option<i64>) -> Self {
            self.client_id_issued_at = input;
            self
        }
        /// <p>Indicates the time at which the <code>clientId</code> and <code>clientSecret</code> will become invalid.</p>
        pub fn client_secret_expires_at(mut self, input: i64) -> Self {
            self.client_secret_expires_at = Some(input);
            self
        }
        /// <p>Indicates the time at which the <code>clientId</code> and <code>clientSecret</code> will become invalid.</p>
        pub fn set_client_secret_expires_at(mut self, input: std::option::Option<i64>) -> Self {
            self.client_secret_expires_at = input;
            self
        }
        /// <p>The endpoint where the client can request authorization.</p>
        pub fn authorization_endpoint(mut self, input: impl Into<std::string::String>) -> Self {
            self.authorization_endpoint = Some(input.into());
            self
        }
        /// <p>The endpoint where the client can request authorization.</p>
        pub fn set_authorization_endpoint(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.authorization_endpoint = input;
            self
        }
        /// <p>The endpoint where the client can get an access token.</p>
        pub fn token_endpoint(mut self, input: impl Into<std::string::String>) -> Self {
            self.token_endpoint = Some(input.into());
            self
        }
        /// <p>The endpoint where the client can get an access token.</p>
        pub fn set_token_endpoint(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.token_endpoint = input;
            self
        }
        /// Consumes the builder and constructs a [`RegisterClientOutput`](crate::output::RegisterClientOutput)
        pub fn build(self) -> crate::output::RegisterClientOutput {
            crate::output::RegisterClientOutput {
                client_id: self.client_id,
                client_secret: self.client_secret,
                client_id_issued_at: self.client_id_issued_at.unwrap_or_default(),
                client_secret_expires_at: self.client_secret_expires_at.unwrap_or_default(),
                authorization_endpoint: self.authorization_endpoint,
                token_endpoint: self.token_endpoint,
            }
        }
    }
}
impl RegisterClientOutput {
    /// Creates a new builder-style object to manufacture [`RegisterClientOutput`](crate::output::RegisterClientOutput)
    pub fn builder() -> crate::output::register_client_output::Builder {
        crate::output::register_client_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateTokenOutput {
    /// <p>An opaque token to access AWS SSO resources assigned to a user.</p>
    pub access_token: std::option::Option<std::string::String>,
    /// <p>Used to notify the client that the returned token is an access token. The supported type is <code>BearerToken</code>.</p>
    pub token_type: std::option::Option<std::string::String>,
    /// <p>Indicates the time in seconds when an access token will expire.</p>
    pub expires_in: i32,
    /// <p>A token that, if present, can be used to refresh a previously issued access token that might have expired.</p>
    pub refresh_token: std::option::Option<std::string::String>,
    /// <p>The identifier of the user that associated with the access token, if present.</p>
    pub id_token: std::option::Option<std::string::String>,
}
impl CreateTokenOutput {
    /// <p>An opaque token to access AWS SSO resources assigned to a user.</p>
    pub fn access_token(&self) -> std::option::Option<&str> {
        self.access_token.as_deref()
    }
    /// <p>Used to notify the client that the returned token is an access token. The supported type is <code>BearerToken</code>.</p>
    pub fn token_type(&self) -> std::option::Option<&str> {
        self.token_type.as_deref()
    }
    /// <p>Indicates the time in seconds when an access token will expire.</p>
    pub fn expires_in(&self) -> i32 {
        self.expires_in
    }
    /// <p>A token that, if present, can be used to refresh a previously issued access token that might have expired.</p>
    pub fn refresh_token(&self) -> std::option::Option<&str> {
        self.refresh_token.as_deref()
    }
    /// <p>The identifier of the user that associated with the access token, if present.</p>
    pub fn id_token(&self) -> std::option::Option<&str> {
        self.id_token.as_deref()
    }
}
impl std::fmt::Debug for CreateTokenOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateTokenOutput");
        formatter.field("access_token", &self.access_token);
        formatter.field("token_type", &self.token_type);
        formatter.field("expires_in", &self.expires_in);
        formatter.field("refresh_token", &self.refresh_token);
        formatter.field("id_token", &self.id_token);
        formatter.finish()
    }
}
/// See [`CreateTokenOutput`](crate::output::CreateTokenOutput)
pub mod create_token_output {
    /// A builder for [`CreateTokenOutput`](crate::output::CreateTokenOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) access_token: std::option::Option<std::string::String>,
        pub(crate) token_type: std::option::Option<std::string::String>,
        pub(crate) expires_in: std::option::Option<i32>,
        pub(crate) refresh_token: std::option::Option<std::string::String>,
        pub(crate) id_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>An opaque token to access AWS SSO resources assigned to a user.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.access_token = Some(input.into());
            self
        }
        /// <p>An opaque token to access AWS SSO resources assigned to a user.</p>
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.access_token = input;
            self
        }
        /// <p>Used to notify the client that the returned token is an access token. The supported type is <code>BearerToken</code>.</p>
        pub fn token_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.token_type = Some(input.into());
            self
        }
        /// <p>Used to notify the client that the returned token is an access token. The supported type is <code>BearerToken</code>.</p>
        pub fn set_token_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.token_type = input;
            self
        }
        /// <p>Indicates the time in seconds when an access token will expire.</p>
        pub fn expires_in(mut self, input: i32) -> Self {
            self.expires_in = Some(input);
            self
        }
        /// <p>Indicates the time in seconds when an access token will expire.</p>
        pub fn set_expires_in(mut self, input: std::option::Option<i32>) -> Self {
            self.expires_in = input;
            self
        }
        /// <p>A token that, if present, can be used to refresh a previously issued access token that might have expired.</p>
        pub fn refresh_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.refresh_token = Some(input.into());
            self
        }
        /// <p>A token that, if present, can be used to refresh a previously issued access token that might have expired.</p>
        pub fn set_refresh_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.refresh_token = input;
            self
        }
        /// <p>The identifier of the user that associated with the access token, if present.</p>
        pub fn id_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.id_token = Some(input.into());
            self
        }
        /// <p>The identifier of the user that associated with the access token, if present.</p>
        pub fn set_id_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id_token = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateTokenOutput`](crate::output::CreateTokenOutput)
        pub fn build(self) -> crate::output::CreateTokenOutput {
            crate::output::CreateTokenOutput {
                access_token: self.access_token,
                token_type: self.token_type,
                expires_in: self.expires_in.unwrap_or_default(),
                refresh_token: self.refresh_token,
                id_token: self.id_token,
            }
        }
    }
}
impl CreateTokenOutput {
    /// Creates a new builder-style object to manufacture [`CreateTokenOutput`](crate::output::CreateTokenOutput)
    pub fn builder() -> crate::output::create_token_output::Builder {
        crate::output::create_token_output::Builder::default()
    }
}
