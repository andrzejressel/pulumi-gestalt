#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResponseHeadersPolicyCorsConfig {
    /// A Boolean value that CloudFront uses as the value for the `Access-Control-Allow-Credentials` HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlAllowCredentials")]
    pub r#access_control_allow_credentials: bool,
    /// Object that contains an attribute `items` that contains a list of HTTP header names that CloudFront includes as values for the `Access-Control-Allow-Headers` HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlAllowHeaders")]
    pub r#access_control_allow_headers: Box<super::super::types::cloudfront::ResponseHeadersPolicyCorsConfigAccessControlAllowHeaders>,
    /// Object that contains an attribute `items` that contains a list of HTTP methods that CloudFront includes as values for the `Access-Control-Allow-Methods` HTTP response header. Valid values: `GET` | `POST` | `OPTIONS` | `PUT` | `DELETE` | `HEAD` | `ALL`
    #[builder(into)]
    #[serde(rename = "accessControlAllowMethods")]
    pub r#access_control_allow_methods: Box<super::super::types::cloudfront::ResponseHeadersPolicyCorsConfigAccessControlAllowMethods>,
    /// Object that contains an attribute `items` that contains a list of origins that CloudFront can use as the value for the `Access-Control-Allow-Origin` HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlAllowOrigins")]
    pub r#access_control_allow_origins: Box<super::super::types::cloudfront::ResponseHeadersPolicyCorsConfigAccessControlAllowOrigins>,
    /// Object that contains an attribute `items` that contains a list of HTTP headers that CloudFront includes as values for the `Access-Control-Expose-Headers` HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlExposeHeaders")]
    pub r#access_control_expose_headers: Option<Box<super::super::types::cloudfront::ResponseHeadersPolicyCorsConfigAccessControlExposeHeaders>>,
    /// A number that CloudFront uses as the value for the `Access-Control-Max-Age` HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlMaxAgeSec")]
    pub r#access_control_max_age_sec: Option<i32>,
    /// A Boolean value that determines how CloudFront behaves for the HTTP response header.
    #[builder(into)]
    #[serde(rename = "originOverride")]
    pub r#origin_override: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResponseHeadersPolicyCorsConfig {
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
                    "access_control_allow_credentials",
                    &self.r#access_control_allow_credentials,
                ),
                to_pulumi_object_field(
                    "access_control_allow_headers",
                    &self.r#access_control_allow_headers,
                ),
                to_pulumi_object_field(
                    "access_control_allow_methods",
                    &self.r#access_control_allow_methods,
                ),
                to_pulumi_object_field(
                    "access_control_allow_origins",
                    &self.r#access_control_allow_origins,
                ),
                to_pulumi_object_field(
                    "access_control_expose_headers",
                    &self.r#access_control_expose_headers,
                ),
                to_pulumi_object_field(
                    "access_control_max_age_sec",
                    &self.r#access_control_max_age_sec,
                ),
                to_pulumi_object_field(
                    "origin_override",
                    &self.r#origin_override,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResponseHeadersPolicyCorsConfig {
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
                    r#access_control_allow_credentials: {
                        let field_value = match fields_map.get("access_control_allow_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_allow_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#access_control_allow_headers: {
                        let field_value = match fields_map.get("access_control_allow_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_allow_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#access_control_allow_methods: {
                        let field_value = match fields_map.get("access_control_allow_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_allow_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#access_control_allow_origins: {
                        let field_value = match fields_map.get("access_control_allow_origins") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_allow_origins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#access_control_expose_headers: {
                        let field_value = match fields_map.get("access_control_expose_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_expose_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#access_control_max_age_sec: {
                        let field_value = match fields_map.get("access_control_max_age_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_max_age_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_override: {
                        let field_value = match fields_map.get("origin_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
