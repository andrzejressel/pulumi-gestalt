#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountCorsRule {
    /// A list of headers that are allowed to be a part of the cross-origin request.
    #[builder(into)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Vec<String>,
    /// A list of HTTP headers that are allowed to be executed by the origin. Valid options are `DELETE`, `GET`, `HEAD`, `MERGE`, `POST`, `OPTIONS`, `PUT` or `PATCH`.
    #[builder(into)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Vec<String>,
    /// A list of origin domains that will be allowed by CORS.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Vec<String>,
    /// A list of response headers that are exposed to CORS clients.
    #[builder(into)]
    #[serde(rename = "exposedHeaders")]
    pub r#exposed_headers: Vec<String>,
    /// The number of seconds the client should cache a preflight response. Possible values are between `1` and `2147483647`.
    #[builder(into)]
    #[serde(rename = "maxAgeInSeconds")]
    pub r#max_age_in_seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccountCorsRule {
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
                    "allowed_origins",
                    &self.r#allowed_origins,
                ),
                to_pulumi_object_field(
                    "exposed_headers",
                    &self.r#exposed_headers,
                ),
                to_pulumi_object_field(
                    "max_age_in_seconds",
                    &self.r#max_age_in_seconds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccountCorsRule {
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
                    r#allowed_origins: {
                        let field_value = match fields_map.get("allowed_origins") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_origins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#max_age_in_seconds: {
                        let field_value = match fields_map.get("max_age_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_age_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
