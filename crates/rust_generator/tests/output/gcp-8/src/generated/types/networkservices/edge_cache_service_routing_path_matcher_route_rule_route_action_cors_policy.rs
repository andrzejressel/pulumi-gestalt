#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCorsPolicy {
    /// In response to a preflight request, setting this to true indicates that the actual request can include user credentials.
    /// This translates to the Access-Control-Allow-Credentials response header.
    #[builder(into)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Option<bool>,
    /// Specifies the content for the Access-Control-Allow-Headers response header.
    #[builder(into)]
    #[serde(rename = "allowHeaders")]
    pub r#allow_headers: Option<Vec<String>>,
    /// Specifies the content for the Access-Control-Allow-Methods response header.
    #[builder(into)]
    #[serde(rename = "allowMethods")]
    pub r#allow_methods: Option<Vec<String>>,
    /// Specifies the list of origins that will be allowed to do CORS requests.
    /// This translates to the Access-Control-Allow-Origin response header.
    #[builder(into)]
    #[serde(rename = "allowOrigins")]
    pub r#allow_origins: Option<Vec<String>>,
    /// If true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
    /// Specifies the content for the Access-Control-Allow-Headers response header.
    #[builder(into)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Option<Vec<String>>,
    /// Specifies how long results of a preflight request can be cached by a client in seconds. Note that many browser clients enforce a maximum TTL of 600s (10 minutes).
    /// - Setting the value to -1 forces a pre-flight check for all requests (not recommended)
    /// - A maximum TTL of 86400s can be set, but note that (as above) some clients may force pre-flight checks at a more regular interval.
    /// - This translates to the Access-Control-Max-Age header.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCorsPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "allow_credentials".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_credentials,
                )
                .await,
            );
            map.insert(
                "allow_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_headers,
                )
                .await,
            );
            map.insert(
                "allow_methods".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_methods,
                )
                .await,
            );
            map.insert(
                "allow_origins".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_origins,
                )
                .await,
            );
            map.insert(
                "disabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disabled,
                )
                .await,
            );
            map.insert(
                "expose_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expose_headers,
                )
                .await,
            );
            map.insert(
                "max_age".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_age,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCorsPolicy {
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
                        let field_value = match fields_map.get("allow_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_headers: {
                        let field_value = match fields_map.get("allow_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_methods: {
                        let field_value = match fields_map.get("allow_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_origins: {
                        let field_value = match fields_map.get("allow_origins") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_origins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("expose_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'expose_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_age: {
                        let field_value = match fields_map.get("max_age") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_age' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
