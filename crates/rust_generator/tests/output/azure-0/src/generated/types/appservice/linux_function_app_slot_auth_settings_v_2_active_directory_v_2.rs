#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxFunctionAppSlotAuthSettingsV2ActiveDirectoryV2 {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinuxFunctionAppSlotAuthSettingsV2ActiveDirectoryV2 {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "allowed_applications".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_applications,
                )
                .await,
            );
            map.insert(
                "allowed_audiences".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_audiences,
                )
                .await,
            );
            map.insert(
                "allowed_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_groups,
                )
                .await,
            );
            map.insert(
                "allowed_identities".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_identities,
                )
                .await,
            );
            map.insert(
                "client_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_id,
                )
                .await,
            );
            map.insert(
                "client_secret_certificate_thumbprint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_secret_certificate_thumbprint,
                )
                .await,
            );
            map.insert(
                "client_secret_setting_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_secret_setting_name,
                )
                .await,
            );
            map.insert(
                "jwt_allowed_client_applications".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#jwt_allowed_client_applications,
                )
                .await,
            );
            map.insert(
                "jwt_allowed_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#jwt_allowed_groups,
                )
                .await,
            );
            map.insert(
                "login_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#login_parameters,
                )
                .await,
            );
            map.insert(
                "tenant_auth_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tenant_auth_endpoint,
                )
                .await,
            );
            map.insert(
                "www_authentication_disabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#www_authentication_disabled,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinuxFunctionAppSlotAuthSettingsV2ActiveDirectoryV2 {
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
                        let field_value = match fields_map.get("allowed_applications") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_applications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_audiences: {
                        let field_value = match fields_map.get("allowed_audiences") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_audiences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_groups: {
                        let field_value = match fields_map.get("allowed_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_identities: {
                        let field_value = match fields_map.get("allowed_identities") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_identities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_id: {
                        let field_value = match fields_map.get("client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_secret_certificate_thumbprint: {
                        let field_value = match fields_map.get("client_secret_certificate_thumbprint") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_secret_certificate_thumbprint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_secret_setting_name: {
                        let field_value = match fields_map.get("client_secret_setting_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_secret_setting_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jwt_allowed_client_applications: {
                        let field_value = match fields_map.get("jwt_allowed_client_applications") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwt_allowed_client_applications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jwt_allowed_groups: {
                        let field_value = match fields_map.get("jwt_allowed_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwt_allowed_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#login_parameters: {
                        let field_value = match fields_map.get("login_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'login_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tenant_auth_endpoint: {
                        let field_value = match fields_map.get("tenant_auth_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'tenant_auth_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#www_authentication_disabled: {
                        let field_value = match fields_map.get("www_authentication_disabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'www_authentication_disabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
