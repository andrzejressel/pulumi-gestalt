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
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "cookies_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cookies_configs,
                )
                .await,
            );
            map.insert(
                "enable_accept_encoding_brotli".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_accept_encoding_brotli,
                )
                .await,
            );
            map.insert(
                "enable_accept_encoding_gzip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_accept_encoding_gzip,
                )
                .await,
            );
            map.insert(
                "headers_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#headers_configs,
                )
                .await,
            );
            map.insert(
                "query_strings_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_strings_configs,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCachePolicyParametersInCacheKeyAndForwardedToOrigin {
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
                    r#cookies_configs: {
                        let field_value = match fields_map.get("cookies_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookies_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_accept_encoding_brotli: {
                        let field_value = match fields_map.get("enable_accept_encoding_brotli") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_accept_encoding_brotli' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_accept_encoding_gzip: {
                        let field_value = match fields_map.get("enable_accept_encoding_gzip") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_accept_encoding_gzip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#headers_configs: {
                        let field_value = match fields_map.get("headers_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_strings_configs: {
                        let field_value = match fields_map.get("query_strings_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_strings_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
