#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkforcePoolProviderOidc {
    /// The client ID. Must match the audience claim of the JWT issued by the identity provider.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The optional client secret. Required to enable Authorization Code flow for web sign-in.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<Box<super::super::types::iam::WorkforcePoolProviderOidcClientSecret>>,
    /// The OIDC issuer URI. Must be a valid URI using the 'https' scheme.
    #[builder(into)]
    #[serde(rename = "issuerUri")]
    pub r#issuer_uri: String,
    /// OIDC JWKs in JSON String format. For details on definition of a
    /// JWK, see https:tools.ietf.org/html/rfc7517. If not set, then we
    /// use the `jwks_uri` from the discovery document fetched from the
    /// .well-known path for the `issuer_uri`. Currently, RSA and EC asymmetric
    /// keys are supported. The JWK must use following format and include only
    /// the following fields:
    /// ```sh
    /// {
    /// "keys": [
    /// {
    /// "kty": "RSA/EC",
    /// "alg": "<algorithm>",
    /// "use": "sig",
    /// "kid": "<key-id>",
    /// "n": "",
    /// "e": "",
    /// "x": "",
    /// "y": "",
    /// "crv": ""
    /// }
    /// ]
    /// }
    /// ```
    #[builder(into)]
    #[serde(rename = "jwksJson")]
    pub r#jwks_json: Option<String>,
    /// Configuration for web single sign-on for the OIDC provider. Here, web sign-in refers to console sign-in and gcloud sign-in through the browser.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "webSsoConfig")]
    pub r#web_sso_config: Option<Box<super::super::types::iam::WorkforcePoolProviderOidcWebSsoConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkforcePoolProviderOidc {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "client_id",
                    &self.r#client_id,
                ),
                to_pulumi_object_field(
                    "client_secret",
                    &self.r#client_secret,
                ),
                to_pulumi_object_field(
                    "issuer_uri",
                    &self.r#issuer_uri,
                ),
                to_pulumi_object_field(
                    "jwks_json",
                    &self.r#jwks_json,
                ),
                to_pulumi_object_field(
                    "web_sso_config",
                    &self.r#web_sso_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkforcePoolProviderOidc {
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
                        let field_value = match fields_map.get("client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_secret: {
                        let field_value = match fields_map.get("client_secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#issuer_uri: {
                        let field_value = match fields_map.get("issuer_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuer_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jwks_json: {
                        let field_value = match fields_map.get("jwks_json") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwks_json' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_sso_config: {
                        let field_value = match fields_map.get("web_sso_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'web_sso_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
