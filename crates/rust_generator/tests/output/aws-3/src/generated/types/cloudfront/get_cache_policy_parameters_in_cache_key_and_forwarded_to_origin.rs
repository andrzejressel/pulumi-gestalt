#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCachePolicyParametersInCacheKeyAndForwardedToOrigin {
    /// Object that determines whether any cookies in viewer requests (and if so, which cookies) are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
    #[builder(into)]
    pub r#cookies_configs: Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig>,
    /// A flag that can affect whether the Accept-Encoding HTTP header is included in the cache key and included in requests that CloudFront sends to the origin.
    #[builder(into)]
    pub r#enable_accept_encoding_brotli: bool,
    /// A flag that can affect whether the Accept-Encoding HTTP header is included in the cache key and included in requests that CloudFront sends to the origin.
    #[builder(into)]
    pub r#enable_accept_encoding_gzip: bool,
    /// Object that determines whether any HTTP headers (and if so, which headers) are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
    #[builder(into)]
    pub r#headers_configs: Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig>,
    /// Object that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Query String Config for more information.
    #[builder(into)]
    pub r#query_strings_configs: Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCachePolicyParametersInCacheKeyAndForwardedToOrigin {
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
                    "cookiesConfigs",
                    &self.r#cookies_configs,
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
                    "headersConfigs",
                    &self.r#headers_configs,
                ),
                to_pulumi_object_field(
                    "queryStringsConfigs",
                    &self.r#query_strings_configs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("cookiesConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookiesConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#headers_configs: {
                        let field_value = match fields_map.get("headersConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'headersConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_strings_configs: {
                        let field_value = match fields_map.get("queryStringsConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'queryStringsConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
