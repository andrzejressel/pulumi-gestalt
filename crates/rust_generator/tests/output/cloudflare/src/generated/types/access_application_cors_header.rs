#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessApplicationCorsHeader {
    /// Value to determine whether all HTTP headers are exposed.
    #[builder(into)]
    #[serde(rename = "allowAllHeaders")]
    pub r#allow_all_headers: Option<bool>,
    /// Value to determine whether all methods are exposed.
    #[builder(into)]
    #[serde(rename = "allowAllMethods")]
    pub r#allow_all_methods: Option<bool>,
    /// Value to determine whether all origins are permitted to make CORS requests.
    #[builder(into)]
    #[serde(rename = "allowAllOrigins")]
    pub r#allow_all_origins: Option<bool>,
    /// Value to determine if credentials (cookies, authorization headers, or TLS client certificates) are included with requests.
    #[builder(into)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Option<bool>,
    /// List of HTTP headers to expose via CORS.
    #[builder(into)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Option<Vec<String>>,
    /// List of methods to expose via CORS.
    #[builder(into)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Option<Vec<String>>,
    /// List of origins permitted to make CORS requests.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Option<Vec<String>>,
    /// The maximum time a preflight request will be cached.
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccessApplicationCorsHeader {
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
                "allow_all_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_all_headers,
                )
                .await,
            );
            map.insert(
                "allow_all_methods".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_all_methods,
                )
                .await,
            );
            map.insert(
                "allow_all_origins".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_all_origins,
                )
                .await,
            );
            map.insert(
                "allow_credentials".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_credentials,
                )
                .await,
            );
            map.insert(
                "allowed_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_headers,
                )
                .await,
            );
            map.insert(
                "allowed_methods".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_methods,
                )
                .await,
            );
            map.insert(
                "allowed_origins".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_origins,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccessApplicationCorsHeader {
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
                    r#allow_all_headers: {
                        let field_value = match fields_map.get("allow_all_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_all_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_all_methods: {
                        let field_value = match fields_map.get("allow_all_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_all_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_all_origins: {
                        let field_value = match fields_map.get("allow_all_origins") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_all_origins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_credentials: {
                        let field_value = match fields_map.get("allow_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
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
