#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FunctionUrlCors {
    /// Whether to allow cookies or other credentials in requests to the function URL. The default is `false`.
    #[builder(into)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Option<bool>,
    /// The HTTP headers that origins can include in requests to the function URL. For example: `["date", "keep-alive", "x-custom-header"]`.
    #[builder(into)]
    #[serde(rename = "allowHeaders")]
    pub r#allow_headers: Option<Vec<String>>,
    /// The HTTP methods that are allowed when calling the function URL. For example: `["GET", "POST", "DELETE"]`, or the wildcard character (`["*"]`).
    #[builder(into)]
    #[serde(rename = "allowMethods")]
    pub r#allow_methods: Option<Vec<String>>,
    /// The origins that can access the function URL. You can list any number of specific origins (or the wildcard character (`"*"`)), separated by a comma. For example: `["https://www.example.com", "http://localhost:60905"]`.
    #[builder(into)]
    #[serde(rename = "allowOrigins")]
    pub r#allow_origins: Option<Vec<String>>,
    /// The HTTP headers in your function response that you want to expose to origins that call the function URL.
    #[builder(into)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Option<Vec<String>>,
    /// The maximum amount of time, in seconds, that web browsers can cache results of a preflight request. By default, this is set to `0`, which means that the browser doesn't cache results. The maximum value is `86400`.
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FunctionUrlCors {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FunctionUrlCors {
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
