#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorRouteCache {
    /// Is content compression enabled? Possible values are `true` or `false`. Defaults to `false`.
    /// 
    /// > **NOTE:** Content won't be compressed when the requested content is smaller than `1 KB` or larger than `8 MB`(inclusive).
    #[builder(into)]
    #[serde(rename = "compressionEnabled")]
    pub r#compression_enabled: Option<bool>,
    /// A list of one or more `Content types` (formerly known as `MIME types`) to compress. Possible values include `application/eot`, `application/font`, `application/font-sfnt`, `application/javascript`, `application/json`, `application/opentype`, `application/otf`, `application/pkcs7-mime`, `application/truetype`, `application/ttf`, `application/vnd.ms-fontobject`, `application/xhtml+xml`, `application/xml`, `application/xml+rss`, `application/x-font-opentype`, `application/x-font-truetype`, `application/x-font-ttf`, `application/x-httpd-cgi`, `application/x-mpegurl`, `application/x-opentype`, `application/x-otf`, `application/x-perl`, `application/x-ttf`, `application/x-javascript`, `font/eot`, `font/ttf`, `font/otf`, `font/opentype`, `image/svg+xml`, `text/css`, `text/csv`, `text/html`, `text/javascript`, `text/js`, `text/plain`, `text/richtext`, `text/tab-separated-values`, `text/xml`, `text/x-script`, `text/x-component` or `text/x-java-source`.
    #[builder(into)]
    #[serde(rename = "contentTypesToCompresses")]
    pub r#content_types_to_compresses: Option<Vec<String>>,
    /// Defines how the Front Door Route will cache requests that include query strings. Possible values include `IgnoreQueryString`, `IgnoreSpecifiedQueryStrings`, `IncludeSpecifiedQueryStrings` or `UseQueryString`. Defaults to `IgnoreQueryString`.
    /// 
    /// > **NOTE:** The value of the `query_string_caching_behavior` determines if the `query_strings` field will be used as an include list or an ignore list.
    #[builder(into)]
    #[serde(rename = "queryStringCachingBehavior")]
    pub r#query_string_caching_behavior: Option<String>,
    /// Query strings to include or ignore.
    #[builder(into)]
    #[serde(rename = "queryStrings")]
    pub r#query_strings: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorRouteCache {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "compression_enabled",
                    &self.r#compression_enabled,
                ),
                to_pulumi_object_field(
                    "content_types_to_compresses",
                    &self.r#content_types_to_compresses,
                ),
                to_pulumi_object_field(
                    "query_string_caching_behavior",
                    &self.r#query_string_caching_behavior,
                ),
                to_pulumi_object_field(
                    "query_strings",
                    &self.r#query_strings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorRouteCache {
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
                    r#compression_enabled: {
                        let field_value = match fields_map.get("compression_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'compression_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content_types_to_compresses: {
                        let field_value = match fields_map.get("content_types_to_compresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_types_to_compresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_string_caching_behavior: {
                        let field_value = match fields_map.get("query_string_caching_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_string_caching_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_strings: {
                        let field_value = match fields_map.get("query_strings") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_strings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
