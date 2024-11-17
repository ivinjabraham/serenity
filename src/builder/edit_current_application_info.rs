use std::borrow::Cow;
use std::collections::HashMap;

#[cfg(feature = "http")]
use crate::http::Http;
use crate::model::prelude::*;

/// A builder for editing [`CurrentApplicationInfo`] i.e the current Application's information.
///
/// The fields are optional, and only the ones explicitly set will be updated.
#[derive(Clone, Debug, Default, Serialize)]
#[must_use]
pub struct EditCurrentApplicationInfo<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_install_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_connections_verification_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    install_params: Option<CreateInstallParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration_types_config: Option<HashMap<InstallationContext, InstallationContextConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<ApplicationFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cover_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interactions_endpoint_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_webhooks_url: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_webhook_status: Option<EventWebhookStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_webhook_types: Option<Vec<EventWebhookType>>,
}

impl<'a> EditCurrentApplicationInfo<'a> {
    /// Creates a new builder instance with all values set to None.
    pub fn new() -> Self {
        Self::default()
    }

    /// Default custom authorization URL for the app, if enabled.
    pub fn install_url(mut self, url: impl Into<Cow<'a, str>>) -> Self {
        self.custom_install_url = Some(url.into());
        self
    }

    /// Description of the app.
    pub fn description(mut self, description: impl Into<Cow<'a, str>>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Role connection verification URL for the app.
    pub fn verification_url(mut self, url: impl Into<Cow<'a, str>>) -> Self {
        self.role_connections_verification_url = Some(url.into());
        self
    }

    /// Settings for the app's default in-app authorization link, if enabled.
    pub fn install_params(mut self, params: CreateInstallParams) -> Self {
        self.install_params = Some(params);
        self
    }

    /// Default scopes and permissions for each supported installation context.
    /// Value for each key is an integration type configuration object.
    pub fn integration_types_config(
        mut self,
        config: HashMap<InstallationContext, InstallationContextConfig>,
    ) -> Self {
        self.integration_types_config = Some(config);
        self
    }

    /// App's public flags.
    ///
    /// Only limited intent flags (GATEWAY_PRESENCE_LIMITED, GATEWAY_GUILD_MEMBERS_LIMITED,
    /// and GATEWAY_MESSAGE_CONTENT_LIMITED) can be updated via the API.
    pub fn flags(mut self, flags: ApplicationFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    /// Icon for the app
    pub fn icon(mut self, base64_image_data: String) -> Self {
        self.icon = Some(base64_image_data);
        self
    }

    /// Sets the default rich presence invite cover image using base64 image data.
    pub fn cover_image(mut self, base64_image_data: String) -> Self {
        self.cover_image = Some(base64_image_data);
        self
    }

    /// Interactions endpoint URL for the app.
    ///
    /// To update an Interactions endpoint URL via the API, the URL must be valid according
    /// to the [Receiving an Interaction]
    /// (https://discord.com/developers/docs/interactions/receiving-and-responding#receiving-an-interaction) documentation.
    pub fn endpoint_url(mut self, url: impl Into<Cow<'a, str>>) -> Self {
        self.interactions_endpoint_url = Some(url.into());
        self
    }

    /// List of tags describing the content and functionality of the app (max of 20 characters per
    /// tag). Max of 5 tags.
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Event webhooks URL for the app to receive webhook events.
    pub fn webhooks_url(mut self, url: impl Into<Cow<'a, str>>) -> Self {
        self.event_webhooks_url = Some(url.into());
        self
    }

    /// If webhook events are enabled for the app.
    pub fn webhook_status(mut self, status: EventWebhookStatus) -> Self {
        self.event_webhook_status = Some(status);
        self
    }

    /// List of Webhook event types to subscribe to.
    pub fn webhook_types(mut self, types: Vec<EventWebhookType>) -> Self {
        self.event_webhook_types = Some(types);
        self
    }

    /// Executes the builder, sending the configured application data to Discord.
    /// Returns updated [`CurrentApplicationInfo`] on success.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or if the Discord API
    /// rejects the updated information.
    #[cfg(feature = "http")]
    pub async fn execute(self, http: &Http) -> Result<CurrentApplicationInfo> {
        http.edit_current_application_info(&self).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateInstallParams {
    pub scopes: Vec<Scope>,
    pub permissions: Permissions,
}

impl CreateInstallParams {
    #[must_use]
    pub fn new(scopes: Vec<Scope>, permissions: Permissions) -> Self {
        Self {
            scopes,
            permissions,
        }
    }

    #[must_use]
    pub fn scopes(mut self, scopes: Vec<Scope>) -> Self {
        self.scopes = scopes;
        self
    }

    #[must_use]
    pub fn permissions(mut self, permissions: Permissions) -> Self {
        self.permissions = permissions;
        self
    }
}
