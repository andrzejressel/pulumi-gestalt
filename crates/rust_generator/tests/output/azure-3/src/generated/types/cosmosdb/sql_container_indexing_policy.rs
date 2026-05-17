#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SqlContainerIndexingPolicy {
    /// One or more `composite_index` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "compositeIndices")]
    pub r#composite_indices: Option<Vec<super::super::types::cosmosdb::SqlContainerIndexingPolicyCompositeIndex>>,
    /// One or more `excluded_path` blocks as defined below. Either `included_path` or `excluded_path` must contain the `path` `/*`
    #[builder(into)]
    #[serde(rename = "excludedPaths")]
    pub r#excluded_paths: Option<Vec<super::super::types::cosmosdb::SqlContainerIndexingPolicyExcludedPath>>,
    /// One or more `included_path` blocks as defined below. Either `included_path` or `excluded_path` must contain the `path` `/*`
    #[builder(into)]
    #[serde(rename = "includedPaths")]
    pub r#included_paths: Option<Vec<super::super::types::cosmosdb::SqlContainerIndexingPolicyIncludedPath>>,
    /// Indicates the indexing mode. Possible values include: `consistent` and `none`. Defaults to `consistent`.
    #[builder(into)]
    #[serde(rename = "indexingMode")]
    pub r#indexing_mode: Option<String>,
    /// One or more `spatial_index` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "spatialIndices")]
    pub r#spatial_indices: Option<Vec<super::super::types::cosmosdb::SqlContainerIndexingPolicySpatialIndex>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SqlContainerIndexingPolicy {
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
                "composite_indices".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#composite_indices,
                )
                .await,
            );
            map.insert(
                "excluded_paths".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#excluded_paths,
                )
                .await,
            );
            map.insert(
                "included_paths".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#included_paths,
                )
                .await,
            );
            map.insert(
                "indexing_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#indexing_mode,
                )
                .await,
            );
            map.insert(
                "spatial_indices".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spatial_indices,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SqlContainerIndexingPolicy {
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
                    r#composite_indices: {
                        let field_value = match fields_map.get("composite_indices") {
                            Some(value) => value,
                            None => bail!("Missing field 'composite_indices' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_paths: {
                        let field_value = match fields_map.get("excluded_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_paths: {
                        let field_value = match fields_map.get("included_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#indexing_mode: {
                        let field_value = match fields_map.get("indexing_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'indexing_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spatial_indices: {
                        let field_value = match fields_map.get("spatial_indices") {
                            Some(value) => value,
                            None => bail!("Missing field 'spatial_indices' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
