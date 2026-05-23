#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWindowsWebAppAuthSettingsV2CustomOidcV2 {
    /// The endpoint to make the Authorisation Request as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    pub r#authorisation_endpoint: String,
    /// The endpoint that provides the keys necessary to validate the token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    pub r#certification_uri: String,
    /// The Client Credential Method used.
    #[builder(into)]
    pub r#client_credential_method: String,
    /// The OAuth 2.0 client ID used by the app for authentication.
    #[builder(into)]
    pub r#client_id: String,
    /// The app setting name containing the OAuth 2.0 client secret used by the app for authentication.
    #[builder(into)]
    pub r#client_secret_setting_name: String,
    /// The endpoint that issued the Token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    pub r#issuer_endpoint: String,
    /// The name of this Windows Web App.
    #[builder(into)]
    pub r#name: String,
    /// The name of the claim that contains the users name.
    #[builder(into)]
    pub r#name_claim_type: String,
    /// The endpoint used for OpenID Connect Discovery. For example `https://example.com/.well-known/openid-configuration`.
    #[builder(into)]
    pub r#openid_configuration_endpoint: String,
    /// The list of the scopes that are requested while authenticating.
    #[builder(into)]
    pub r#scopes: Vec<String>,
    /// The endpoint used to request a Token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    pub r#token_endpoint: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetWindowsWebAppAuthSettingsV2CustomOidcV2 {
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
                    "authorisationEndpoint",
                    &self.r#authorisation_endpoint,
                ),
                to_pulumi_object_field(
                    "certificationUri",
                    &self.r#certification_uri,
                ),
                to_pulumi_object_field(
                    "clientCredentialMethod",
                    &self.r#client_credential_method,
                ),
                to_pulumi_object_field(
                    "clientId",
                    &self.r#client_id,
                ),
                to_pulumi_object_field(
                    "clientSecretSettingName",
                    &self.r#client_secret_setting_name,
                ),
                to_pulumi_object_field(
                    "issuerEndpoint",
                    &self.r#issuer_endpoint,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "nameClaimType",
                    &self.r#name_claim_type,
                ),
                to_pulumi_object_field(
                    "openidConfigurationEndpoint",
                    &self.r#openid_configuration_endpoint,
                ),
                to_pulumi_object_field(
                    "scopes",
                    &self.r#scopes,
                ),
                to_pulumi_object_field(
                    "tokenEndpoint",
                    &self.r#token_endpoint,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetWindowsWebAppAuthSettingsV2CustomOidcV2 {
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
                    r#authorisation_endpoint: {
                        let field_value = match fields_map.get("authorisationEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorisationEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certification_uri: {
                        let field_value = match fields_map.get("certificationUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificationUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_credential_method: {
                        let field_value = match fields_map.get("clientCredentialMethod") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientCredentialMethod' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#client_secret_setting_name: {
                        let field_value = match fields_map.get("clientSecretSettingName") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientSecretSettingName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#issuer_endpoint: {
                        let field_value = match fields_map.get("issuerEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuerEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name_claim_type: {
                        let field_value = match fields_map.get("nameClaimType") {
                            Some(value) => value,
                            None => bail!("Missing field 'nameClaimType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#openid_configuration_endpoint: {
                        let field_value = match fields_map.get("openidConfigurationEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'openidConfigurationEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scopes: {
                        let field_value = match fields_map.get("scopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'scopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_endpoint: {
                        let field_value = match fields_map.get("tokenEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'tokenEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
