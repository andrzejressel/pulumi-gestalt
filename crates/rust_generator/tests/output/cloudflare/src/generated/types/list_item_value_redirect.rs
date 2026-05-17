#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ListItemValueRedirect {
    /// Whether the redirect also matches subdomains of the source url. Available values: `disabled`, `enabled`.
    #[builder(into)]
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Option<String>,
    /// Whether to preserve the path suffix when doing subpath matching. Available values: `disabled`, `enabled`.
    #[builder(into)]
    #[serde(rename = "preservePathSuffix")]
    pub r#preserve_path_suffix: Option<String>,
    /// Whether the redirect target url should keep the query string of the request's url. Available values: `disabled`, `enabled`.
    #[builder(into)]
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Option<String>,
    /// The source url of the redirect.
    #[builder(into)]
    #[serde(rename = "sourceUrl")]
    pub r#source_url: String,
    /// The status code to be used when redirecting a request.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
    /// Whether the redirect also matches subpaths of the source url. Available values: `disabled`, `enabled`.
    #[builder(into)]
    #[serde(rename = "subpathMatching")]
    pub r#subpath_matching: Option<String>,
    /// The target url of the redirect.
    #[builder(into)]
    #[serde(rename = "targetUrl")]
    pub r#target_url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ListItemValueRedirect {
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
                "include_subdomains".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_subdomains,
                )
                .await,
            );
            map.insert(
                "preserve_path_suffix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preserve_path_suffix,
                )
                .await,
            );
            map.insert(
                "preserve_query_string".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preserve_query_string,
                )
                .await,
            );
            map.insert(
                "source_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_url,
                )
                .await,
            );
            map.insert(
                "status_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status_code,
                )
                .await,
            );
            map.insert(
                "subpath_matching".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subpath_matching,
                )
                .await,
            );
            map.insert(
                "target_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_url,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ListItemValueRedirect {
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
                    r#include_subdomains: {
                        let field_value = match fields_map.get("include_subdomains") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_subdomains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preserve_path_suffix: {
                        let field_value = match fields_map.get("preserve_path_suffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'preserve_path_suffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preserve_query_string: {
                        let field_value = match fields_map.get("preserve_query_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'preserve_query_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_url: {
                        let field_value = match fields_map.get("source_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status_code: {
                        let field_value = match fields_map.get("status_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'status_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subpath_matching: {
                        let field_value = match fields_map.get("subpath_matching") {
                            Some(value) => value,
                            None => bail!("Missing field 'subpath_matching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_url: {
                        let field_value = match fields_map.get("target_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
