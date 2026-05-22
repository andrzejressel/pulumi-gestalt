#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsWebAppSlotAuthSettingsV2ActiveDirectoryV2 {
    /// The list of allowed Applications for the Default Authorisation Policy.
    #[builder(into)]
    #[serde(rename = "allowedApplications")]
    pub r#allowed_applications: Option<Vec<String>>,
    /// Specifies a list of Allowed audience values to consider when validating JWTs issued by Azure Active Directory.
    /// 
    /// > **NOTE:** This is configured on the Authentication Provider side and is Read Only here.
    #[builder(into)]
    #[serde(rename = "allowedAudiences")]
    pub r#allowed_audiences: Option<Vec<String>>,
    /// The list of allowed Group Names for the Default Authorisation Policy.
    #[builder(into)]
    #[serde(rename = "allowedGroups")]
    pub r#allowed_groups: Option<Vec<String>>,
    /// The list of allowed Identities for the Default Authorisation Policy.
    #[builder(into)]
    #[serde(rename = "allowedIdentities")]
    pub r#allowed_identities: Option<Vec<String>>,
    /// The ID of the Client to use to authenticate with Azure Active Directory.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The thumbprint of the certificate used for signing purposes.
    #[builder(into)]
    #[serde(rename = "clientSecretCertificateThumbprint")]
    pub r#client_secret_certificate_thumbprint: Option<String>,
    /// The App Setting name that contains the client secret of the Client.
    /// 
    /// !> **NOTE:** A setting with this name must exist in `app_settings` to function correctly.
    #[builder(into)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Option<String>,
    /// A list of Allowed Client Applications in the JWT Claim.
    #[builder(into)]
    #[serde(rename = "jwtAllowedClientApplications")]
    pub r#jwt_allowed_client_applications: Option<Vec<String>>,
    /// A list of Allowed Groups in the JWT Claim.
    #[builder(into)]
    #[serde(rename = "jwtAllowedGroups")]
    pub r#jwt_allowed_groups: Option<Vec<String>>,
    /// A map of key-value pairs to send to the Authorisation Endpoint when a user logs in.
    #[builder(into)]
    #[serde(rename = "loginParameters")]
    pub r#login_parameters: Option<std::collections::HashMap<String, String>>,
    /// The Azure Tenant Endpoint for the Authenticating Tenant. e.g. `https://login.microsoftonline.com/{tenant-guid}/v2.0/`
    /// 
    /// > **NOTE:** [Here](https://learn.microsoft.com/en-us/entra/identity-platform/authentication-national-cloud#microsoft-entra-authentication-endpoints) is a list of possible authentication endpoints based on the cloud environment. [Here](https://learn.microsoft.com/en-us/azure/app-service/configure-authentication-provider-aad?tabs=workforce-tenant) is more information to better understand how to configure authentication for Azure App Service or Azure Functions.
    #[builder(into)]
    #[serde(rename = "tenantAuthEndpoint")]
    pub r#tenant_auth_endpoint: String,
    /// Should the www-authenticate provider should be omitted from the request? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "wwwAuthenticationDisabled")]
    pub r#www_authentication_disabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsWebAppSlotAuthSettingsV2ActiveDirectoryV2 {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "allowedApplications",
                    &self.r#allowed_applications,
                ),
                to_pulumi_object_field(
                    "allowedAudiences",
                    &self.r#allowed_audiences,
                ),
                to_pulumi_object_field(
                    "allowedGroups",
                    &self.r#allowed_groups,
                ),
                to_pulumi_object_field(
                    "allowedIdentities",
                    &self.r#allowed_identities,
                ),
                to_pulumi_object_field(
                    "clientId",
                    &self.r#client_id,
                ),
                to_pulumi_object_field(
                    "clientSecretCertificateThumbprint",
                    &self.r#client_secret_certificate_thumbprint,
                ),
                to_pulumi_object_field(
                    "clientSecretSettingName",
                    &self.r#client_secret_setting_name,
                ),
                to_pulumi_object_field(
                    "jwtAllowedClientApplications",
                    &self.r#jwt_allowed_client_applications,
                ),
                to_pulumi_object_field(
                    "jwtAllowedGroups",
                    &self.r#jwt_allowed_groups,
                ),
                to_pulumi_object_field(
                    "loginParameters",
                    &self.r#login_parameters,
                ),
                to_pulumi_object_field(
                    "tenantAuthEndpoint",
                    &self.r#tenant_auth_endpoint,
                ),
                to_pulumi_object_field(
                    "wwwAuthenticationDisabled",
                    &self.r#www_authentication_disabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsWebAppSlotAuthSettingsV2ActiveDirectoryV2 {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#allowed_applications: {
                        let field_value = match fields_map.get("allowedApplications") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedApplications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_audiences: {
                        let field_value = match fields_map.get("allowedAudiences") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedAudiences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_groups: {
                        let field_value = match fields_map.get("allowedGroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedGroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_identities: {
                        let field_value = match fields_map.get("allowedIdentities") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedIdentities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_id: {
                        let field_value = match fields_map.get("clientId") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_secret_certificate_thumbprint: {
                        let field_value = match fields_map.get("clientSecretCertificateThumbprint") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientSecretCertificateThumbprint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_secret_setting_name: {
                        let field_value = match fields_map.get("clientSecretSettingName") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientSecretSettingName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jwt_allowed_client_applications: {
                        let field_value = match fields_map.get("jwtAllowedClientApplications") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwtAllowedClientApplications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jwt_allowed_groups: {
                        let field_value = match fields_map.get("jwtAllowedGroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwtAllowedGroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#login_parameters: {
                        let field_value = match fields_map.get("loginParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'loginParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tenant_auth_endpoint: {
                        let field_value = match fields_map.get("tenantAuthEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'tenantAuthEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#www_authentication_disabled: {
                        let field_value = match fields_map.get("wwwAuthenticationDisabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'wwwAuthenticationDisabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
