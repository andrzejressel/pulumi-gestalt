#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobHttpTarget {
    /// HTTP request body.
    /// A request body is allowed only if the HTTP method is POST, PUT, or PATCH.
    /// It is an error to set body on a job with an incompatible HttpMethod.
    /// A base64-encoded string.
    #[builder(into)]
    #[serde(rename = "body")]
    pub r#body: Option<String>,
    /// This map contains the header field names and values.
    /// Repeated headers are not supported, but a header value can contain commas.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<std::collections::HashMap<String, String>>,
    /// Which HTTP method to use for the request.
    #[builder(into)]
    #[serde(rename = "httpMethod")]
    pub r#http_method: Option<String>,
    /// Contains information needed for generating an OAuth token.
    /// This type of authorization should be used when sending requests to a GCP endpoint.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oauthToken")]
    pub r#oauth_token: Option<Box<super::super::types::cloudscheduler::JobHttpTargetOauthToken>>,
    /// Contains information needed for generating an OpenID Connect token.
    /// This type of authorization should be used when sending requests to third party endpoints or Cloud Run.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oidcToken")]
    pub r#oidc_token: Option<Box<super::super::types::cloudscheduler::JobHttpTargetOidcToken>>,
    /// The full URI path that the request will be sent to.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobHttpTarget {
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
                    "body",
                    &self.r#body,
                ),
                to_pulumi_object_field(
                    "headers",
                    &self.r#headers,
                ),
                to_pulumi_object_field(
                    "http_method",
                    &self.r#http_method,
                ),
                to_pulumi_object_field(
                    "oauth_token",
                    &self.r#oauth_token,
                ),
                to_pulumi_object_field(
                    "oidc_token",
                    &self.r#oidc_token,
                ),
                to_pulumi_object_field(
                    "uri",
                    &self.r#uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobHttpTarget {
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
                    r#body: {
                        let field_value = match fields_map.get("body") {
                            Some(value) => value,
                            None => bail!("Missing field 'body' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#oauth_token: {
                        let field_value = match fields_map.get("oauth_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oidc_token: {
                        let field_value = match fields_map.get("oidc_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'oidc_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uri: {
                        let field_value = match fields_map.get("uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
