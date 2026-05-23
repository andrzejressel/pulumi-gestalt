#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustAccessIdentityProviderConfig {
    #[builder(into)]
    #[serde(rename = "apiToken")]
    pub r#api_token: Option<String>,
    #[builder(into)]
    #[serde(rename = "appsDomain")]
    pub r#apps_domain: Option<String>,
    #[builder(into)]
    #[serde(rename = "attributes")]
    pub r#attributes: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "authUrl")]
    pub r#auth_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "authorizationServerId")]
    pub r#authorization_server_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "centrifyAccount")]
    pub r#centrify_account: Option<String>,
    #[builder(into)]
    #[serde(rename = "centrifyAppId")]
    pub r#centrify_app_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "certsUrl")]
    pub r#certs_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "claims")]
    pub r#claims: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    #[builder(into)]
    #[serde(rename = "conditionalAccessEnabled")]
    pub r#conditional_access_enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "directoryId")]
    pub r#directory_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "emailAttributeName")]
    pub r#email_attribute_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "emailClaimName")]
    pub r#email_claim_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "idpPublicCert")]
    pub r#idp_public_cert: Option<String>,
    #[builder(into)]
    #[serde(rename = "issuerUrl")]
    pub r#issuer_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "oktaAccount")]
    pub r#okta_account: Option<String>,
    #[builder(into)]
    #[serde(rename = "oneloginAccount")]
    pub r#onelogin_account: Option<String>,
    #[builder(into)]
    #[serde(rename = "pingEnvId")]
    pub r#ping_env_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "pkceEnabled")]
    pub r#pkce_enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "redirectUrl")]
    pub r#redirect_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "signRequest")]
    pub r#sign_request: Option<bool>,
    #[builder(into)]
    #[serde(rename = "ssoTargetUrl")]
    pub r#sso_target_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "supportGroups")]
    pub r#support_groups: Option<bool>,
    #[builder(into)]
    #[serde(rename = "tokenUrl")]
    pub r#token_url: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZeroTrustAccessIdentityProviderConfig {
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
                    "apiToken",
                    &self.r#api_token,
                ),
                to_pulumi_object_field(
                    "appsDomain",
                    &self.r#apps_domain,
                ),
                to_pulumi_object_field(
                    "attributes",
                    &self.r#attributes,
                ),
                to_pulumi_object_field(
                    "authUrl",
                    &self.r#auth_url,
                ),
                to_pulumi_object_field(
                    "authorizationServerId",
                    &self.r#authorization_server_id,
                ),
                to_pulumi_object_field(
                    "centrifyAccount",
                    &self.r#centrify_account,
                ),
                to_pulumi_object_field(
                    "centrifyAppId",
                    &self.r#centrify_app_id,
                ),
                to_pulumi_object_field(
                    "certsUrl",
                    &self.r#certs_url,
                ),
                to_pulumi_object_field(
                    "claims",
                    &self.r#claims,
                ),
                to_pulumi_object_field(
                    "clientId",
                    &self.r#client_id,
                ),
                to_pulumi_object_field(
                    "clientSecret",
                    &self.r#client_secret,
                ),
                to_pulumi_object_field(
                    "conditionalAccessEnabled",
                    &self.r#conditional_access_enabled,
                ),
                to_pulumi_object_field(
                    "directoryId",
                    &self.r#directory_id,
                ),
                to_pulumi_object_field(
                    "emailAttributeName",
                    &self.r#email_attribute_name,
                ),
                to_pulumi_object_field(
                    "emailClaimName",
                    &self.r#email_claim_name,
                ),
                to_pulumi_object_field(
                    "idpPublicCert",
                    &self.r#idp_public_cert,
                ),
                to_pulumi_object_field(
                    "issuerUrl",
                    &self.r#issuer_url,
                ),
                to_pulumi_object_field(
                    "oktaAccount",
                    &self.r#okta_account,
                ),
                to_pulumi_object_field(
                    "oneloginAccount",
                    &self.r#onelogin_account,
                ),
                to_pulumi_object_field(
                    "pingEnvId",
                    &self.r#ping_env_id,
                ),
                to_pulumi_object_field(
                    "pkceEnabled",
                    &self.r#pkce_enabled,
                ),
                to_pulumi_object_field(
                    "redirectUrl",
                    &self.r#redirect_url,
                ),
                to_pulumi_object_field(
                    "scopes",
                    &self.r#scopes,
                ),
                to_pulumi_object_field(
                    "signRequest",
                    &self.r#sign_request,
                ),
                to_pulumi_object_field(
                    "ssoTargetUrl",
                    &self.r#sso_target_url,
                ),
                to_pulumi_object_field(
                    "supportGroups",
                    &self.r#support_groups,
                ),
                to_pulumi_object_field(
                    "tokenUrl",
                    &self.r#token_url,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZeroTrustAccessIdentityProviderConfig {
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
                    r#api_token: {
                        let field_value = match fields_map.get("apiToken") {
                            Some(value) => value,
                            None => bail!("Missing field 'apiToken' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#apps_domain: {
                        let field_value = match fields_map.get("appsDomain") {
                            Some(value) => value,
                            None => bail!("Missing field 'appsDomain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#attributes: {
                        let field_value = match fields_map.get("attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_url: {
                        let field_value = match fields_map.get("authUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'authUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authorization_server_id: {
                        let field_value = match fields_map.get("authorizationServerId") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorizationServerId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#centrify_account: {
                        let field_value = match fields_map.get("centrifyAccount") {
                            Some(value) => value,
                            None => bail!("Missing field 'centrifyAccount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#centrify_app_id: {
                        let field_value = match fields_map.get("centrifyAppId") {
                            Some(value) => value,
                            None => bail!("Missing field 'centrifyAppId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certs_url: {
                        let field_value = match fields_map.get("certsUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'certsUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#claims: {
                        let field_value = match fields_map.get("claims") {
                            Some(value) => value,
                            None => bail!("Missing field 'claims' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#client_secret: {
                        let field_value = match fields_map.get("clientSecret") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientSecret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#conditional_access_enabled: {
                        let field_value = match fields_map.get("conditionalAccessEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'conditionalAccessEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#directory_id: {
                        let field_value = match fields_map.get("directoryId") {
                            Some(value) => value,
                            None => bail!("Missing field 'directoryId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_attribute_name: {
                        let field_value = match fields_map.get("emailAttributeName") {
                            Some(value) => value,
                            None => bail!("Missing field 'emailAttributeName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_claim_name: {
                        let field_value = match fields_map.get("emailClaimName") {
                            Some(value) => value,
                            None => bail!("Missing field 'emailClaimName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idp_public_cert: {
                        let field_value = match fields_map.get("idpPublicCert") {
                            Some(value) => value,
                            None => bail!("Missing field 'idpPublicCert' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#okta_account: {
                        let field_value = match fields_map.get("oktaAccount") {
                            Some(value) => value,
                            None => bail!("Missing field 'oktaAccount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#onelogin_account: {
                        let field_value = match fields_map.get("oneloginAccount") {
                            Some(value) => value,
                            None => bail!("Missing field 'oneloginAccount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ping_env_id: {
                        let field_value = match fields_map.get("pingEnvId") {
                            Some(value) => value,
                            None => bail!("Missing field 'pingEnvId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pkce_enabled: {
                        let field_value = match fields_map.get("pkceEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'pkceEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_url: {
                        let field_value = match fields_map.get("redirectUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirectUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#sign_request: {
                        let field_value = match fields_map.get("signRequest") {
                            Some(value) => value,
                            None => bail!("Missing field 'signRequest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sso_target_url: {
                        let field_value = match fields_map.get("ssoTargetUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssoTargetUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#support_groups: {
                        let field_value = match fields_map.get("supportGroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'supportGroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_url: {
                        let field_value = match fields_map.get("tokenUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'tokenUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
