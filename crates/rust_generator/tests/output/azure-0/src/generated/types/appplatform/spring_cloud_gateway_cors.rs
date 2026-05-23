#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudGatewayCors {
    /// Allowed headers in cross-site requests. The special value `*` allows actual requests to send any header.
    #[builder(into)]
    pub r#allowed_headers: Option<Vec<String>>,
    /// Allowed HTTP methods on cross-site requests. The special value `*` allows all methods. If not set, `GET` and `HEAD` are allowed by default. Possible values are `DELETE`, `GET`, `HEAD`, `MERGE`, `POST`, `OPTIONS` and `PUT`.
    #[builder(into)]
    pub r#allowed_methods: Option<Vec<String>>,
    /// Allowed origin patterns to make cross-site requests.
    #[builder(into)]
    pub r#allowed_origin_patterns: Option<Vec<String>>,
    /// Allowed origins to make cross-site requests. The special value `*` allows all domains.
    #[builder(into)]
    pub r#allowed_origins: Option<Vec<String>>,
    /// is user credentials are supported on cross-site requests?
    #[builder(into)]
    pub r#credentials_allowed: Option<bool>,
    /// HTTP response headers to expose for cross-site requests.
    #[builder(into)]
    pub r#exposed_headers: Option<Vec<String>>,
    /// How long, in seconds, the response from a pre-flight request can be cached by clients.
    #[builder(into)]
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
                    "allowedHeaders",
                    &self.r#allowed_headers,
                ),
                to_pulumi_object_field(
                    "allowedMethods",
                    &self.r#allowed_methods,
                ),
                to_pulumi_object_field(
                    "allowedOriginPatterns",
                    &self.r#allowed_origin_patterns,
                ),
                to_pulumi_object_field(
                    "allowedOrigins",
                    &self.r#allowed_origins,
                ),
                to_pulumi_object_field(
                    "credentialsAllowed",
                    &self.r#credentials_allowed,
                ),
                to_pulumi_object_field(
                    "exposedHeaders",
                    &self.r#exposed_headers,
                ),
                to_pulumi_object_field(
                    "maxAgeSeconds",
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
                        let field_value = match fields_map.get("allowedHeaders") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedHeaders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_methods: {
                        let field_value = match fields_map.get("allowedMethods") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedMethods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_origin_patterns: {
                        let field_value = match fields_map.get("allowedOriginPatterns") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedOriginPatterns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_origins: {
                        let field_value = match fields_map.get("allowedOrigins") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedOrigins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#credentials_allowed: {
                        let field_value = match fields_map.get("credentialsAllowed") {
                            Some(value) => value,
                            None => bail!("Missing field 'credentialsAllowed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exposed_headers: {
                        let field_value = match fields_map.get("exposedHeaders") {
                            Some(value) => value,
                            None => bail!("Missing field 'exposedHeaders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_age_seconds: {
                        let field_value = match fields_map.get("maxAgeSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxAgeSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
