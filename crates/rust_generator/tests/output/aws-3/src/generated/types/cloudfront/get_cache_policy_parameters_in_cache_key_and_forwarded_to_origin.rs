#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCachePolicyParametersInCacheKeyAndForwardedToOrigin {
    /// Object that determines whether any cookies in viewer requests (and if so, which cookies) are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
    #[builder(into)]
    #[serde(rename = "cookiesConfigs")]
    pub r#cookies_configs: Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig>,
    /// A flag that can affect whether the Accept-Encoding HTTP header is included in the cache key and included in requests that CloudFront sends to the origin.
    #[builder(into)]
    #[serde(rename = "enableAcceptEncodingBrotli")]
    pub r#enable_accept_encoding_brotli: bool,
    /// A flag that can affect whether the Accept-Encoding HTTP header is included in the cache key and included in requests that CloudFront sends to the origin.
    #[builder(into)]
    #[serde(rename = "enableAcceptEncodingGzip")]
    pub r#enable_accept_encoding_gzip: bool,
    /// Object that determines whether any HTTP headers (and if so, which headers) are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
    #[builder(into)]
    #[serde(rename = "headersConfigs")]
    pub r#headers_configs: Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig>,
    /// Object that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Query String Config for more information.
    #[builder(into)]
    #[serde(rename = "queryStringsConfigs")]
    pub r#query_strings_configs: Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCachePolicyParametersInCacheKeyAndForwardedToOrigin {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("cookies_configs".to_string(), self.r#cookies_configs.to_pulumi_value().await);
            map.insert("enable_accept_encoding_brotli".to_string(), self.r#enable_accept_encoding_brotli.to_pulumi_value().await);
            map.insert("enable_accept_encoding_gzip".to_string(), self.r#enable_accept_encoding_gzip.to_pulumi_value().await);
            map.insert("headers_configs".to_string(), self.r#headers_configs.to_pulumi_value().await);
            map.insert("query_strings_configs".to_string(), self.r#query_strings_configs.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCachePolicyParametersInCacheKeyAndForwardedToOrigin {
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
                    r#cookies_configs: {
                        let field_value = match fields_map.get("cookies_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookies_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enable_accept_encoding_brotli: {
                        let field_value = match fields_map.get("enable_accept_encoding_brotli") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_accept_encoding_brotli' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enable_accept_encoding_gzip: {
                        let field_value = match fields_map.get("enable_accept_encoding_gzip") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_accept_encoding_gzip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#headers_configs: {
                        let field_value = match fields_map.get("headers_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#query_strings_configs: {
                        let field_value = match fields_map.get("query_strings_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_strings_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
