#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudGatewayCors {
    /// Allowed headers in cross-site requests. The special value `*` allows actual requests to send any header.
    #[builder(into)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Option<Vec<String>>,
    /// Allowed HTTP methods on cross-site requests. The special value `*` allows all methods. If not set, `GET` and `HEAD` are allowed by default. Possible values are `DELETE`, `GET`, `HEAD`, `MERGE`, `POST`, `OPTIONS` and `PUT`.
    #[builder(into)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Option<Vec<String>>,
    /// Allowed origin patterns to make cross-site requests.
    #[builder(into)]
    #[serde(rename = "allowedOriginPatterns")]
    pub r#allowed_origin_patterns: Option<Vec<String>>,
    /// Allowed origins to make cross-site requests. The special value `*` allows all domains.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Option<Vec<String>>,
    /// is user credentials are supported on cross-site requests?
    #[builder(into)]
    #[serde(rename = "credentialsAllowed")]
    pub r#credentials_allowed: Option<bool>,
    /// HTTP response headers to expose for cross-site requests.
    #[builder(into)]
    #[serde(rename = "exposedHeaders")]
    pub r#exposed_headers: Option<Vec<String>>,
    /// How long, in seconds, the response from a pre-flight request can be cached by clients.
    #[builder(into)]
    #[serde(rename = "maxAgeSeconds")]
    pub r#max_age_seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpringCloudGatewayCors {
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
                    "allowed_headers",
                    &self.r#allowed_headers,
                ),
                to_pulumi_object_field(
                    "allowed_methods",
                    &self.r#allowed_methods,
                ),
                to_pulumi_object_field(
                    "allowed_origin_patterns",
                    &self.r#allowed_origin_patterns,
                ),
                to_pulumi_object_field(
                    "allowed_origins",
                    &self.r#allowed_origins,
                ),
                to_pulumi_object_field(
                    "credentials_allowed",
                    &self.r#credentials_allowed,
                ),
                to_pulumi_object_field(
                    "exposed_headers",
                    &self.r#exposed_headers,
                ),
                to_pulumi_object_field(
                    "max_age_seconds",
                    &self.r#max_age_seconds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpringCloudGatewayCors {
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
                    r#allowed_headers: {
                        let field_value = match fields_map.get("allowed_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_methods: {
                        let field_value = match fields_map.get("allowed_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_origin_patterns: {
                        let field_value = match fields_map.get("allowed_origin_patterns") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_origin_patterns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_origins: {
                        let field_value = match fields_map.get("allowed_origins") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_origins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#credentials_allowed: {
                        let field_value = match fields_map.get("credentials_allowed") {
                            Some(value) => value,
                            None => bail!("Missing field 'credentials_allowed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exposed_headers: {
                        let field_value = match fields_map.get("exposed_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'exposed_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_age_seconds: {
                        let field_value = match fields_map.get("max_age_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_age_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
