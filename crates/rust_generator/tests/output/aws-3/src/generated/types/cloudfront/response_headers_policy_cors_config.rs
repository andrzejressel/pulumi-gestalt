#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResponseHeadersPolicyCorsConfig {
    /// A Boolean value that CloudFront uses as the value for the `Access-Control-Allow-Credentials` HTTP response header.
    #[builder(into)]
    pub r#access_control_allow_credentials: bool,
    /// Object that contains an attribute `items` that contains a list of HTTP header names that CloudFront includes as values for the `Access-Control-Allow-Headers` HTTP response header.
    #[builder(into)]
    pub r#access_control_allow_headers: Box<super::super::types::cloudfront::ResponseHeadersPolicyCorsConfigAccessControlAllowHeaders>,
    /// Object that contains an attribute `items` that contains a list of HTTP methods that CloudFront includes as values for the `Access-Control-Allow-Methods` HTTP response header. Valid values: `GET` | `POST` | `OPTIONS` | `PUT` | `DELETE` | `HEAD` | `ALL`
    #[builder(into)]
    pub r#access_control_allow_methods: Box<super::super::types::cloudfront::ResponseHeadersPolicyCorsConfigAccessControlAllowMethods>,
    /// Object that contains an attribute `items` that contains a list of origins that CloudFront can use as the value for the `Access-Control-Allow-Origin` HTTP response header.
    #[builder(into)]
    pub r#access_control_allow_origins: Box<super::super::types::cloudfront::ResponseHeadersPolicyCorsConfigAccessControlAllowOrigins>,
    /// Object that contains an attribute `items` that contains a list of HTTP headers that CloudFront includes as values for the `Access-Control-Expose-Headers` HTTP response header.
    #[builder(into)]
    pub r#access_control_expose_headers: Option<Box<super::super::types::cloudfront::ResponseHeadersPolicyCorsConfigAccessControlExposeHeaders>>,
    /// A number that CloudFront uses as the value for the `Access-Control-Max-Age` HTTP response header.
    #[builder(into)]
    pub r#access_control_max_age_sec: Option<i32>,
    /// A Boolean value that determines how CloudFront behaves for the HTTP response header.
    #[builder(into)]
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
                    "accessControlAllowCredentials",
                    &self.r#access_control_allow_credentials,
                ),
                to_pulumi_object_field(
                    "accessControlAllowHeaders",
                    &self.r#access_control_allow_headers,
                ),
                to_pulumi_object_field(
                    "accessControlAllowMethods",
                    &self.r#access_control_allow_methods,
                ),
                to_pulumi_object_field(
                    "accessControlAllowOrigins",
                    &self.r#access_control_allow_origins,
                ),
                to_pulumi_object_field(
                    "accessControlExposeHeaders",
                    &self.r#access_control_expose_headers,
                ),
                to_pulumi_object_field(
                    "accessControlMaxAgeSec",
                    &self.r#access_control_max_age_sec,
                ),
                to_pulumi_object_field(
                    "originOverride",
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
                        let field_value = match fields_map.get("accessControlAllowCredentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessControlAllowCredentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#access_control_allow_headers: {
                        let field_value = match fields_map.get("accessControlAllowHeaders") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessControlAllowHeaders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#access_control_allow_methods: {
                        let field_value = match fields_map.get("accessControlAllowMethods") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessControlAllowMethods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#access_control_allow_origins: {
                        let field_value = match fields_map.get("accessControlAllowOrigins") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessControlAllowOrigins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#access_control_expose_headers: {
                        let field_value = match fields_map.get("accessControlExposeHeaders") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessControlExposeHeaders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#access_control_max_age_sec: {
                        let field_value = match fields_map.get("accessControlMaxAgeSec") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessControlMaxAgeSec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_override: {
                        let field_value = match fields_map.get("originOverride") {
                            Some(value) => value,
                            None => bail!("Missing field 'originOverride' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
