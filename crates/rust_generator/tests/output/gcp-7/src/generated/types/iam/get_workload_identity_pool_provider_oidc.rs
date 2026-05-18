#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWorkloadIdentityPoolProviderOidc {
    /// Acceptable values for the 'aud' field (audience) in the OIDC token. Token exchange
    /// requests are rejected if the token audience does not match one of the configured
    /// values. Each audience may be at most 256 characters. A maximum of 10 audiences may
    /// be configured.
    /// 
    /// If this list is empty, the OIDC token audience must be equal to the full canonical
    /// resource name of the WorkloadIdentityPoolProvider, with or without the HTTPS prefix.
    /// For example:
    /// '''
    /// //iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>
    /// https://iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>
    /// '''
    #[builder(into)]
    #[serde(rename = "allowedAudiences")]
    pub r#allowed_audiences: Vec<String>,
    /// The OIDC issuer URL.
    #[builder(into)]
    #[serde(rename = "issuerUri")]
    pub r#issuer_uri: String,
    /// OIDC JWKs in JSON String format. For details on definition of a
    /// JWK, see https:tools.ietf.org/html/rfc7517. If not set, then we
    /// use the 'jwks_uri' from the discovery document fetched from the
    /// .well-known path for the 'issuer_uri'. Currently, RSA and EC asymmetric
    /// keys are supported. The JWK must use following format and include only
    /// the following fields:
    /// '''
    /// {
    ///   "keys": [
    ///     {
    ///           "kty": "RSA/EC",
    ///           "alg": "<algorithm>",
    ///           "use": "sig",
    ///           "kid": "<key-id>",
    ///           "n": "",
    ///           "e": "",
    ///           "x": "",
    ///           "y": "",
    ///           "crv": ""
    ///     }
    ///   ]
    /// }
    /// '''
    #[builder(into)]
    #[serde(rename = "jwksJson")]
    pub r#jwks_json: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetWorkloadIdentityPoolProviderOidc {
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
                    "allowed_audiences",
                    &self.r#allowed_audiences,
                ),
                to_pulumi_object_field(
                    "issuer_uri",
                    &self.r#issuer_uri,
                ),
                to_pulumi_object_field(
                    "jwks_json",
                    &self.r#jwks_json,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetWorkloadIdentityPoolProviderOidc {
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
                    r#allowed_audiences: {
                        let field_value = match fields_map.get("allowed_audiences") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_audiences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
