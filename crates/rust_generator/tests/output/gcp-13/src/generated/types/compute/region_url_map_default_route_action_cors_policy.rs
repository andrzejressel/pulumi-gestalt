#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionUrlMapDefaultRouteActionCorsPolicy {
    /// In response to a preflight request, setting this to true indicates that the actual request can include user credentials. This field translates to the Access-Control-Allow-Credentials header.
    /// Default is false.
    #[builder(into)]
    pub r#allow_credentials: Option<bool>,
    /// Specifies the content for the Access-Control-Allow-Headers header.
    #[builder(into)]
    pub r#allow_headers: Option<Vec<String>>,
    /// Specifies the content for the Access-Control-Allow-Methods header.
    #[builder(into)]
    pub r#allow_methods: Option<Vec<String>>,
    /// Specifies the regualar expression patterns that match allowed origins. For regular expression grammar
    /// please see en.cppreference.com/w/cpp/regex/ecmascript
    /// An origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes.
    #[builder(into)]
    pub r#allow_origin_regexes: Option<Vec<String>>,
    /// Specifies the list of origins that will be allowed to do CORS requests.
    /// An origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes.
    #[builder(into)]
    pub r#allow_origins: Option<Vec<String>>,
    /// If true, the setting specifies the CORS policy is disabled. The default value of false, which indicates that the CORS policy is in effect.
    #[builder(into)]
    pub r#disabled: Option<bool>,
    /// Specifies the content for the Access-Control-Expose-Headers header.
    #[builder(into)]
    pub r#expose_headers: Option<Vec<String>>,
    /// Specifies how long results of a preflight request can be cached in seconds.
    /// This translates to the Access-Control-Max-Age header.
    #[builder(into)]
    pub r#max_age: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionUrlMapDefaultRouteActionCorsPolicy {
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
                    "allowCredentials",
                    &self.r#allow_credentials,
                ),
                to_pulumi_object_field(
                    "allowHeaders",
                    &self.r#allow_headers,
                ),
                to_pulumi_object_field(
                    "allowMethods",
                    &self.r#allow_methods,
                ),
                to_pulumi_object_field(
                    "allowOriginRegexes",
                    &self.r#allow_origin_regexes,
                ),
                to_pulumi_object_field(
                    "allowOrigins",
                    &self.r#allow_origins,
                ),
                to_pulumi_object_field(
                    "disabled",
                    &self.r#disabled,
                ),
                to_pulumi_object_field(
                    "exposeHeaders",
                    &self.r#expose_headers,
                ),
                to_pulumi_object_field(
                    "maxAge",
                    &self.r#max_age,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionUrlMapDefaultRouteActionCorsPolicy {
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
                    r#allow_credentials: {
                        let field_value = match fields_map.get("allowCredentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowCredentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_headers: {
                        let field_value = match fields_map.get("allowHeaders") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowHeaders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_methods: {
                        let field_value = match fields_map.get("allowMethods") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowMethods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_origin_regexes: {
                        let field_value = match fields_map.get("allowOriginRegexes") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowOriginRegexes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_origins: {
                        let field_value = match fields_map.get("allowOrigins") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowOrigins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disabled: {
                        let field_value = match fields_map.get("disabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expose_headers: {
                        let field_value = match fields_map.get("exposeHeaders") {
                            Some(value) => value,
                            None => bail!("Missing field 'exposeHeaders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_age: {
                        let field_value = match fields_map.get("maxAge") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxAge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
