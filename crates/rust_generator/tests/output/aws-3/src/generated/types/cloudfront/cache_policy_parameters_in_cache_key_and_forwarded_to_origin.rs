#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CachePolicyParametersInCacheKeyAndForwardedToOrigin {
    /// Whether any cookies in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
    #[builder(into)]
    pub r#cookies_config: Box<super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig>,
    /// Flag determines whether the Accept-Encoding HTTP header is included in the cache key and in requests that CloudFront sends to the origin.
    #[builder(into)]
    pub r#enable_accept_encoding_brotli: Option<bool>,
    /// Whether the Accept-Encoding HTTP header is included in the cache key and in requests sent to the origin by CloudFront.
    #[builder(into)]
    pub r#enable_accept_encoding_gzip: Option<bool>,
    /// Whether any HTTP headers are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
    #[builder(into)]
    pub r#headers_config: Box<super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig>,
    /// Whether any URL query strings in viewer requests are included in the cache key. It also automatically includes these query strings in requests that CloudFront sends to the origin. Please refer to the Query String Config for more information.
    #[builder(into)]
    pub r#query_strings_config: Box<super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CachePolicyParametersInCacheKeyAndForwardedToOrigin {
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
                    "cookiesConfig",
                    &self.r#cookies_config,
                ),
                to_pulumi_object_field(
                    "enableAcceptEncodingBrotli",
                    &self.r#enable_accept_encoding_brotli,
                ),
                to_pulumi_object_field(
                    "enableAcceptEncodingGzip",
                    &self.r#enable_accept_encoding_gzip,
                ),
                to_pulumi_object_field(
                    "headersConfig",
                    &self.r#headers_config,
                ),
                to_pulumi_object_field(
                    "queryStringsConfig",
                    &self.r#query_strings_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("cookiesConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookiesConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_accept_encoding_brotli: {
                        let field_value = match fields_map.get("enableAcceptEncodingBrotli") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableAcceptEncodingBrotli' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_accept_encoding_gzip: {
                        let field_value = match fields_map.get("enableAcceptEncodingGzip") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableAcceptEncodingGzip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#headers_config: {
                        let field_value = match fields_map.get("headersConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'headersConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_strings_config: {
                        let field_value = match fields_map.get("queryStringsConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'queryStringsConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
