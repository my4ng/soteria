//! # D-Bus interface proxy for: `org.freedesktop.PolicyKit1.Authority`
//!
//! This code was generated by `zbus-xmlgen` `4.1.0` from D-Bus introspection data.
//! Source: `Interface '/org/freedesktop/PolicyKit1/Authority' from service 'org.freedesktop.PolicyKit1' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//! This type implements the [D-Bus standard interfaces], (`org.freedesktop.DBus.*`) for which the
//! following zbus API can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//!
//! Consequently `zbus-xmlgen` did not generate code for the above interfaces.
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,

#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zbus::{
    proxy,
    zvariant::{Type, Value},
    DBusError,
};

pub type Result<T> = std::result::Result<T, PolkitError>;

/// Errors from https://www.freedesktop.org/software/polkit/docs/latest/eggdbus-interface-org.freedesktop.PolicyKit1.Authority.html#eggdbus-errordomain-org.freedesktop.PolicyKit1.Error.
#[derive(Debug, DBusError)]
#[zbus(prefix = "org.freedesktop.PolicyKit1.Error")]
pub enum PolkitError {
    #[zbus(error)]
    ZBus(zbus::Error),
    Failed(String),
    Cancelled(String),
    NotSupported(String),
    NotAuthorized(String),
    CancellationIdNotUnique(String),
}

impl From<std::io::Error> for PolkitError {
    fn from(value: std::io::Error) -> Self {
        Self::Failed(value.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct Identity<'a> {
    kind: String,
    #[serde(borrow)]
    details: HashMap<String, Value<'a>>,
}

impl<'a> Identity<'a> {
    pub fn new(kind: String, details: HashMap<String, Value<'a>>) -> Self {
        Identity { kind, details }
    }

    pub fn get_kind(&self) -> &str {
        &self.kind
    }

    pub fn get_details(&self) -> &HashMap<String, Value<'a>> {
        &self.details
    }
}

impl std::fmt::Display for Identity<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Kind: {}; Details: {:#?}", self.kind, self.details)
    }
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct Subject<'a> {
    kind: String,
    #[serde(borrow)]
    details: HashMap<String, Value<'a>>,
}

impl<'a> Subject<'a> {
    pub fn new(kind: String, details: HashMap<String, Value<'a>>) -> Self {
        Self { kind, details }
    }

    pub fn get_kind(&self) -> &str {
        &self.kind
    }

    pub fn get_details(&self) -> &HashMap<String, Value<'a>> {
        &self.details
    }
}

#[proxy(
    interface = "org.freedesktop.PolicyKit1.Authority",
    default_service = "org.freedesktop.PolicyKit1",
    default_path = "/org/freedesktop/PolicyKit1/Authority"
)]
pub trait Authority {
    /// AuthenticationAgentResponse method
    fn authentication_agent_response(&self, cookie: &str, identity: &Identity<'_>) -> Result<()>;

    /// AuthenticationAgentResponse2 method
    fn authentication_agent_response2(
        &self,
        uid: u32,
        cookie: &str,
        identity: &Identity<'_>,
    ) -> Result<()>;

    /// CancelCheckAuthorization method
    fn cancel_check_authorization(&self, cancellation_id: &str) -> Result<()>;

    /// CheckAuthorization method
    fn check_authorization(
        &self,
        subject: &Subject<'_>,
        action_id: &str,
        details: std::collections::HashMap<&str, &str>,
        flags: u32,
        cancellation_id: &str,
    ) -> Result<(bool, bool, std::collections::HashMap<String, String>)>;

    /// EnumerateActions method
    #[allow(clippy::type_complexity)]
    fn enumerate_actions(
        &self,
        locale: &str,
    ) -> Result<
        Vec<(
            String,
            String,
            String,
            String,
            String,
            String,
            u32,
            u32,
            u32,
            std::collections::HashMap<String, String>,
        )>,
    >;

    /// EnumerateTemporaryAuthorizations method
    #[allow(clippy::type_complexity)]
    fn enumerate_temporary_authorizations(
        &self,
        subject: &Subject<'_>,
    ) -> Result<
        Vec<(
            String,
            String,
            (
                String,
                std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
            ),
            u64,
            u64,
        )>,
    >;

    /// RegisterAuthenticationAgent method
    fn register_authentication_agent(
        &self,
        subject: &Subject<'_>,
        locale: &str,
        object_path: &str,
    ) -> Result<()>;

    /// RegisterAuthenticationAgentWithOptions method
    fn register_authentication_agent_with_options(
        &self,
        subject: &Subject<'_>,
        locale: &str,
        object_path: &str,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> Result<()>;

    /// RevokeTemporaryAuthorizationById method
    fn revoke_temporary_authorization_by_id(&self, id: &str) -> Result<()>;

    /// RevokeTemporaryAuthorizations method
    fn revoke_temporary_authorizations(&self, subject: &Subject<'_>) -> Result<()>;

    /// UnregisterAuthenticationAgent method
    fn unregister_authentication_agent(
        &self,
        subject: &Subject<'_>,
        object_path: &str,
    ) -> Result<()>;

    /// Changed signal
    #[zbus(signal)]
    fn changed(&self) -> Result<()>;

    /// BackendFeatures property
    #[zbus(property)]
    fn backend_features(&self) -> Result<u32>;

    /// BackendName property
    #[zbus(property)]
    fn backend_name(&self) -> Result<String>;

    /// BackendVersion property
    #[zbus(property)]
    fn backend_version(&self) -> Result<String>;
}
