#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionBackendServiceCdnPolicyCacheKeyPolicy {
    /// If true requests to different hosts will be cached separately.
    #[builder(into)]
    #[serde(rename = "includeHost")]
    pub r#include_host: Option<bool>,
    /// Names of cookies to include in cache keys.
    #[builder(into)]
    #[serde(rename = "includeNamedCookies")]
    pub r#include_named_cookies: Option<Vec<String>>,
    /// If true, http and https requests will be cached separately.
    #[builder(into)]
    #[serde(rename = "includeProtocol")]
    pub r#include_protocol: Option<bool>,
    /// If true, include query string parameters in the cache key
    /// according to query_string_whitelist and
    /// query_string_blacklist. If neither is set, the entire query
    /// string will be included.
    /// If false, the query string will be excluded from the cache
    /// key entirely.
    #[builder(into)]
    #[serde(rename = "includeQueryString")]
    pub r#include_query_string: Option<bool>,
    /// Names of query string parameters to exclude in cache keys.
    /// All other parameters will be included. Either specify
    /// query_string_whitelist or query_string_blacklist, not both.
    /// '&' and '=' will be percent encoded and not treated as
    /// delimiters.
    #[builder(into)]
    #[serde(rename = "queryStringBlacklists")]
    pub r#query_string_blacklists: Option<Vec<String>>,
    /// Names of query string parameters to include in cache keys.
    /// All other parameters will be excluded. Either specify
    /// query_string_whitelist or query_string_blacklist, not both.
    /// '&' and '=' will be percent encoded and not treated as
    /// delimiters.
    #[builder(into)]
    #[serde(rename = "queryStringWhitelists")]
    pub r#query_string_whitelists: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionBackendServiceCdnPolicyCacheKeyPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "include_host",
                    &self.r#include_host,
                ),
                to_pulumi_object_field(
                    "include_named_cookies",
                    &self.r#include_named_cookies,
                ),
                to_pulumi_object_field(
                    "include_protocol",
                    &self.r#include_protocol,
                ),
                to_pulumi_object_field(
                    "include_query_string",
                    &self.r#include_query_string,
                ),
                to_pulumi_object_field(
                    "query_string_blacklists",
                    &self.r#query_string_blacklists,
                ),
                to_pulumi_object_field(
                    "query_string_whitelists",
                    &self.r#query_string_whitelists,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionBackendServiceCdnPolicyCacheKeyPolicy {
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
                    r#include_host: {
                        let field_value = match fields_map.get("include_host") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_host' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_named_cookies: {
                        let field_value = match fields_map.get("include_named_cookies") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_named_cookies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_protocol: {
                        let field_value = match fields_map.get("include_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_query_string: {
                        let field_value = match fields_map.get("include_query_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_query_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_string_blacklists: {
                        let field_value = match fields_map.get("query_string_blacklists") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_string_blacklists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_string_whitelists: {
                        let field_value = match fields_map.get("query_string_whitelists") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_string_whitelists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
