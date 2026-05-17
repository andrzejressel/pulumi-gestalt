#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBackendBucketCdnPolicyCacheKeyPolicy {
    /// Allows HTTP request headers (by name) to be used in the
    /// cache key.
    #[builder(into)]
    #[serde(rename = "includeHttpHeaders")]
    pub r#include_http_headers: Vec<String>,
    /// Names of query string parameters to include in cache keys.
    /// Default parameters are always included. '&' and '=' will
    /// be percent encoded and not treated as delimiters.
    #[builder(into)]
    #[serde(rename = "queryStringWhitelists")]
    pub r#query_string_whitelists: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetBackendBucketCdnPolicyCacheKeyPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "include_http_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_http_headers,
                )
                .await,
            );
            map.insert(
                "query_string_whitelists".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_string_whitelists,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetBackendBucketCdnPolicyCacheKeyPolicy {
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
                    r#include_http_headers: {
                        let field_value = match fields_map.get("include_http_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_http_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
