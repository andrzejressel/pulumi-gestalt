#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IdentityProviderConfigOidc {
    /// Client ID for the OpenID Connect identity provider.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The JWT claim that the provider will use to return groups.
    #[builder(into)]
    #[serde(rename = "groupsClaim")]
    pub r#groups_claim: Option<String>,
    /// A prefix that is prepended to group claims e.g., `oidc:`.
    #[builder(into)]
    #[serde(rename = "groupsPrefix")]
    pub r#groups_prefix: Option<String>,
    /// The name of the identity provider config.
    #[builder(into)]
    #[serde(rename = "identityProviderConfigName")]
    pub r#identity_provider_config_name: String,
    /// Issuer URL for the OpenID Connect identity provider.
    #[builder(into)]
    #[serde(rename = "issuerUrl")]
    pub r#issuer_url: String,
    /// The key value pairs that describe required claims in the identity token.
    #[builder(into)]
    #[serde(rename = "requiredClaims")]
    pub r#required_claims: Option<std::collections::HashMap<String, String>>,
    /// The JWT claim that the provider will use as the username.
    #[builder(into)]
    #[serde(rename = "usernameClaim")]
    pub r#username_claim: Option<String>,
    /// A prefix that is prepended to username claims.
    #[builder(into)]
    #[serde(rename = "usernamePrefix")]
    pub r#username_prefix: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IdentityProviderConfigOidc {
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
                    "clientId",
                    &self.r#client_id,
                ),
                to_pulumi_object_field(
                    "groupsClaim",
                    &self.r#groups_claim,
                ),
                to_pulumi_object_field(
                    "groupsPrefix",
                    &self.r#groups_prefix,
                ),
                to_pulumi_object_field(
                    "identityProviderConfigName",
                    &self.r#identity_provider_config_name,
                ),
                to_pulumi_object_field(
                    "issuerUrl",
                    &self.r#issuer_url,
                ),
                to_pulumi_object_field(
                    "requiredClaims",
                    &self.r#required_claims,
                ),
                to_pulumi_object_field(
                    "usernameClaim",
                    &self.r#username_claim,
                ),
                to_pulumi_object_field(
                    "usernamePrefix",
                    &self.r#username_prefix,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IdentityProviderConfigOidc {
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
                    r#client_id: {
                        let field_value = match fields_map.get("clientId") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#groups_claim: {
                        let field_value = match fields_map.get("groupsClaim") {
                            Some(value) => value,
                            None => bail!("Missing field 'groupsClaim' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#groups_prefix: {
                        let field_value = match fields_map.get("groupsPrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'groupsPrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_provider_config_name: {
                        let field_value = match fields_map.get("identityProviderConfigName") {
                            Some(value) => value,
                            None => bail!("Missing field 'identityProviderConfigName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#issuer_url: {
                        let field_value = match fields_map.get("issuerUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuerUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#required_claims: {
                        let field_value = match fields_map.get("requiredClaims") {
                            Some(value) => value,
                            None => bail!("Missing field 'requiredClaims' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#username_claim: {
                        let field_value = match fields_map.get("usernameClaim") {
                            Some(value) => value,
                            None => bail!("Missing field 'usernameClaim' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#username_prefix: {
                        let field_value = match fields_map.get("usernamePrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'usernamePrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
