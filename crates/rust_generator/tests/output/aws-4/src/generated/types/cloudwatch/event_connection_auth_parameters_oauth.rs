#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventConnectionAuthParametersOauth {
    /// The URL to the authorization endpoint.
    #[builder(into)]
    #[serde(rename = "authorizationEndpoint")]
    pub r#authorization_endpoint: String,
    /// Contains the client parameters for OAuth authorization. Contains the following two parameters.
    #[builder(into)]
    #[serde(rename = "clientParameters")]
    pub r#client_parameters: Option<Box<super::super::types::cloudwatch::EventConnectionAuthParametersOauthClientParameters>>,
    /// A password for the authorization. Created and stored in AWS Secrets Manager.
    #[builder(into)]
    #[serde(rename = "httpMethod")]
    pub r#http_method: String,
    /// OAuth Http Parameters are additional credentials used to sign the request to the authorization endpoint to exchange the OAuth Client information for an access token. Secret values are stored and managed by AWS Secrets Manager. A maximum of 1 are allowed. Documented below.
    #[builder(into)]
    #[serde(rename = "oauthHttpParameters")]
    pub r#oauth_http_parameters: Box<super::super::types::cloudwatch::EventConnectionAuthParametersOauthOauthHttpParameters>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EventConnectionAuthParametersOauth {
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
                "authorization_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authorization_endpoint,
                )
                .await,
            );
            map.insert(
                "client_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_parameters,
                )
                .await,
            );
            map.insert(
                "http_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_method,
                )
                .await,
            );
            map.insert(
                "oauth_http_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oauth_http_parameters,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EventConnectionAuthParametersOauth {
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
                    r#authorization_endpoint: {
                        let field_value = match fields_map.get("authorization_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorization_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_parameters: {
                        let field_value = match fields_map.get("client_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_method: {
                        let field_value = match fields_map.get("http_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_http_parameters: {
                        let field_value = match fields_map.get("oauth_http_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_http_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
