#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GremlinGraphIndexPolicy {
    /// Indicates if the indexing policy is automatic. Defaults to `true`.
    #[builder(into)]
    pub r#automatic: Option<bool>,
    /// One or more `composite_index` blocks as defined below.
    #[builder(into)]
    pub r#composite_indices: Option<Vec<super::super::types::cosmosdb::GremlinGraphIndexPolicyCompositeIndex>>,
    /// List of paths to exclude from indexing. Required if `indexing_mode` is `Consistent` or `Lazy`.
    #[builder(into)]
    pub r#excluded_paths: Option<Vec<String>>,
    /// List of paths to include in the indexing. Required if `indexing_mode` is `Consistent` or `Lazy`.
    #[builder(into)]
    pub r#included_paths: Option<Vec<String>>,
    /// Indicates the indexing mode. Possible values include: `Consistent`, `Lazy`, `None`.
    #[builder(into)]
    pub r#indexing_mode: String,
    /// One or more `spatial_index` blocks as defined below.
    #[builder(into)]
    pub r#spatial_indices: Option<Vec<super::super::types::cosmosdb::GremlinGraphIndexPolicySpatialIndex>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GremlinGraphIndexPolicy {
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
                    "automatic",
                    &self.r#automatic,
                ),
                to_pulumi_object_field(
                    "compositeIndices",
                    &self.r#composite_indices,
                ),
                to_pulumi_object_field(
                    "excludedPaths",
                    &self.r#excluded_paths,
                ),
                to_pulumi_object_field(
                    "includedPaths",
                    &self.r#included_paths,
                ),
                to_pulumi_object_field(
                    "indexingMode",
                    &self.r#indexing_mode,
                ),
                to_pulumi_object_field(
                    "spatialIndices",
                    &self.r#spatial_indices,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GremlinGraphIndexPolicy {
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
                    r#automatic: {
                        let field_value = match fields_map.get("automatic") {
                            Some(value) => value,
                            None => bail!("Missing field 'automatic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#composite_indices: {
                        let field_value = match fields_map.get("compositeIndices") {
                            Some(value) => value,
                            None => bail!("Missing field 'compositeIndices' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_paths: {
                        let field_value = match fields_map.get("excludedPaths") {
                            Some(value) => value,
                            None => bail!("Missing field 'excludedPaths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_paths: {
                        let field_value = match fields_map.get("includedPaths") {
                            Some(value) => value,
                            None => bail!("Missing field 'includedPaths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#indexing_mode: {
                        let field_value = match fields_map.get("indexingMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'indexingMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spatial_indices: {
                        let field_value = match fields_map.get("spatialIndices") {
                            Some(value) => value,
                            None => bail!("Missing field 'spatialIndices' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
