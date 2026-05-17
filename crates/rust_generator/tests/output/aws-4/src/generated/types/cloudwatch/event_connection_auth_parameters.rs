#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventConnectionAuthParameters {
    /// Parameters used for API_KEY authorization. An API key to include in the header for each authentication request. A maximum of 1 are allowed. Conflicts with `basic` and `oauth`. Documented below.
    #[builder(into)]
    #[serde(rename = "apiKey")]
    pub r#api_key: Option<Box<super::super::types::cloudwatch::EventConnectionAuthParametersApiKey>>,
    /// Parameters used for BASIC authorization. A maximum of 1 are allowed. Conflicts with `api_key` and `oauth`. Documented below.
    #[builder(into)]
    #[serde(rename = "basic")]
    pub r#basic: Option<Box<super::super::types::cloudwatch::EventConnectionAuthParametersBasic>>,
    /// Invocation Http Parameters are additional credentials used to sign each Invocation of the ApiDestination created from this Connection. If the ApiDestination Rule Target has additional HttpParameters, the values will be merged together, with the Connection Invocation Http Parameters taking precedence. Secret values are stored and managed by AWS Secrets Manager. A maximum of 1 are allowed. Documented below.
    #[builder(into)]
    #[serde(rename = "invocationHttpParameters")]
    pub r#invocation_http_parameters: Option<Box<super::super::types::cloudwatch::EventConnectionAuthParametersInvocationHttpParameters>>,
    /// Parameters used for OAUTH_CLIENT_CREDENTIALS authorization. A maximum of 1 are allowed. Conflicts with `basic` and `api_key`. Documented below.
    #[builder(into)]
    #[serde(rename = "oauth")]
    pub r#oauth: Option<Box<super::super::types::cloudwatch::EventConnectionAuthParametersOauth>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EventConnectionAuthParameters {
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
                    "api_key",
                    &self.r#api_key,
                ),
                to_pulumi_object_field(
                    "basic",
                    &self.r#basic,
                ),
                to_pulumi_object_field(
                    "invocation_http_parameters",
                    &self.r#invocation_http_parameters,
                ),
                to_pulumi_object_field(
                    "oauth",
                    &self.r#oauth,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EventConnectionAuthParameters {
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
                    r#api_key: {
                        let field_value = match fields_map.get("api_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#basic: {
                        let field_value = match fields_map.get("basic") {
                            Some(value) => value,
                            None => bail!("Missing field 'basic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#invocation_http_parameters: {
                        let field_value = match fields_map.get("invocation_http_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'invocation_http_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth: {
                        let field_value = match fields_map.get("oauth") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
