#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CachePolicyParametersInCacheKeyAndForwardedToOrigin {
    /// Whether any cookies in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
    #[builder(into)]
    #[serde(rename = "cookiesConfig")]
    pub r#cookies_config: Box<super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig>,
    /// Flag determines whether the Accept-Encoding HTTP header is included in the cache key and in requests that CloudFront sends to the origin.
    #[builder(into)]
    #[serde(rename = "enableAcceptEncodingBrotli")]
    pub r#enable_accept_encoding_brotli: Option<bool>,
    /// Whether the Accept-Encoding HTTP header is included in the cache key and in requests sent to the origin by CloudFront.
    #[builder(into)]
    #[serde(rename = "enableAcceptEncodingGzip")]
    pub r#enable_accept_encoding_gzip: Option<bool>,
    /// Whether any HTTP headers are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
    #[builder(into)]
    #[serde(rename = "headersConfig")]
    pub r#headers_config: Box<super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig>,
    /// Whether any URL query strings in viewer requests are included in the cache key. It also automatically includes these query strings in requests that CloudFront sends to the origin. Please refer to the Query String Config for more information.
    #[builder(into)]
    #[serde(rename = "queryStringsConfig")]
    pub r#query_strings_config: Box<super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CachePolicyParametersInCacheKeyAndForwardedToOrigin {
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
                "cookies_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cookies_config,
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
                "headers_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#headers_config,
                )
                .await,
            );
            map.insert(
                "query_strings_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_strings_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CachePolicyParametersInCacheKeyAndForwardedToOrigin {
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
                    r#cookies_config: {
                        let field_value = match fields_map.get("cookies_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookies_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#headers_config: {
                        let field_value = match fields_map.get("headers_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_strings_config: {
                        let field_value = match fields_map.get("query_strings_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_strings_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
