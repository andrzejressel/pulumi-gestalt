#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApiCorsConfiguration {
    /// Whether credentials are included in the CORS request.
    #[builder(into)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Option<bool>,
    /// Set of allowed HTTP headers.
    #[builder(into)]
    #[serde(rename = "allowHeaders")]
    pub r#allow_headers: Option<Vec<String>>,
    /// Set of allowed HTTP methods.
    #[builder(into)]
    #[serde(rename = "allowMethods")]
    pub r#allow_methods: Option<Vec<String>>,
    /// Set of allowed origins.
    #[builder(into)]
    #[serde(rename = "allowOrigins")]
    pub r#allow_origins: Option<Vec<String>>,
    /// Set of exposed HTTP headers.
    #[builder(into)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Option<Vec<String>>,
    /// Number of seconds that the browser should cache preflight request results.
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApiCorsConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("allow_credentials".to_string(), self.r#allow_credentials.to_pulumi_value().await);
            map.insert("allow_headers".to_string(), self.r#allow_headers.to_pulumi_value().await);
            map.insert("allow_methods".to_string(), self.r#allow_methods.to_pulumi_value().await);
            map.insert("allow_origins".to_string(), self.r#allow_origins.to_pulumi_value().await);
            map.insert("expose_headers".to_string(), self.r#expose_headers.to_pulumi_value().await);
            map.insert("max_age".to_string(), self.r#max_age.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApiCorsConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#allow_credentials: {
                        let field_value = match fields_map.get("allow_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#allow_headers: {
                        let field_value = match fields_map.get("allow_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#allow_methods: {
                        let field_value = match fields_map.get("allow_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#allow_origins: {
                        let field_value = match fields_map.get("allow_origins") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_origins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#expose_headers: {
                        let field_value = match fields_map.get("expose_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'expose_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_age: {
                        let field_value = match fields_map.get("max_age") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_age' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
