#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionCacheBehaviorSettings {
    /// The HTTP methods that are processed and forwarded to the distribution's origin.
    #[builder(into)]
    #[serde(rename = "allowedHttpMethods")]
    pub r#allowed_http_methods: Option<String>,
    /// The HTTP method responses that are cached by your distribution.
    #[builder(into)]
    #[serde(rename = "cachedHttpMethods")]
    pub r#cached_http_methods: Option<String>,
    /// The default amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the content has been updated.
    #[builder(into)]
    #[serde(rename = "defaultTtl")]
    pub r#default_ttl: Option<i32>,
    /// An object that describes the cookies that are forwarded to the origin. Your content is cached based on the cookies that are forwarded. Detailed below
    #[builder(into)]
    #[serde(rename = "forwardedCookies")]
    pub r#forwarded_cookies: Option<Box<super::super::types::lightsail::DistributionCacheBehaviorSettingsForwardedCookies>>,
    /// An object that describes the headers that are forwarded to the origin. Your content is cached based on the headers that are forwarded. Detailed below
    #[builder(into)]
    #[serde(rename = "forwardedHeaders")]
    pub r#forwarded_headers: Option<Box<super::super::types::lightsail::DistributionCacheBehaviorSettingsForwardedHeaders>>,
    /// An object that describes the query strings that are forwarded to the origin. Your content is cached based on the query strings that are forwarded. Detailed below
    #[builder(into)]
    #[serde(rename = "forwardedQueryStrings")]
    pub r#forwarded_query_strings: Option<Box<super::super::types::lightsail::DistributionCacheBehaviorSettingsForwardedQueryStrings>>,
    /// The maximum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated.
    #[builder(into)]
    #[serde(rename = "maximumTtl")]
    pub r#maximum_ttl: Option<i32>,
    /// The minimum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated.
    #[builder(into)]
    #[serde(rename = "minimumTtl")]
    pub r#minimum_ttl: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionCacheBehaviorSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("allowed_http_methods".to_string(), self.r#allowed_http_methods.to_pulumi_value().await);
            map.insert("cached_http_methods".to_string(), self.r#cached_http_methods.to_pulumi_value().await);
            map.insert("default_ttl".to_string(), self.r#default_ttl.to_pulumi_value().await);
            map.insert("forwarded_cookies".to_string(), self.r#forwarded_cookies.to_pulumi_value().await);
            map.insert("forwarded_headers".to_string(), self.r#forwarded_headers.to_pulumi_value().await);
            map.insert("forwarded_query_strings".to_string(), self.r#forwarded_query_strings.to_pulumi_value().await);
            map.insert("maximum_ttl".to_string(), self.r#maximum_ttl.to_pulumi_value().await);
            map.insert("minimum_ttl".to_string(), self.r#minimum_ttl.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionCacheBehaviorSettings {
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
                    r#allowed_http_methods: {
                        let field_value = match fields_map.get("allowed_http_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_http_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cached_http_methods: {
                        let field_value = match fields_map.get("cached_http_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'cached_http_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#default_ttl: {
                        let field_value = match fields_map.get("default_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#forwarded_cookies: {
                        let field_value = match fields_map.get("forwarded_cookies") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwarded_cookies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lightsail::DistributionCacheBehaviorSettingsForwardedCookies>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#forwarded_headers: {
                        let field_value = match fields_map.get("forwarded_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwarded_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lightsail::DistributionCacheBehaviorSettingsForwardedHeaders>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#forwarded_query_strings: {
                        let field_value = match fields_map.get("forwarded_query_strings") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwarded_query_strings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lightsail::DistributionCacheBehaviorSettingsForwardedQueryStrings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#maximum_ttl: {
                        let field_value = match fields_map.get("maximum_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#minimum_ttl: {
                        let field_value = match fields_map.get("minimum_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
