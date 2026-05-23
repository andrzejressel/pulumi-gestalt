#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiFeatureOnlineStoreFeatureviewVectorSearchConfig {
    /// Configuration options for using brute force search, which simply implements the standard linear search in the database for each query. It is primarily meant for benchmarking and to generate the ground truth for approximate search.
    #[builder(into)]
    pub r#brute_force_config: Option<Box<super::super::types::vertex::AiFeatureOnlineStoreFeatureviewVectorSearchConfigBruteForceConfig>>,
    /// Column of crowding. This column contains crowding attribute which is a constraint on a neighbor list produced by nearest neighbor search requiring that no more than some value k' of the k neighbors returned have the same value of crowdingAttribute.
    #[builder(into)]
    pub r#crowding_column: Option<String>,
    /// The distance measure used in nearest neighbor search.
    /// For details on allowed values, see the [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.featureOnlineStores.featureViews#DistanceMeasureType).
    /// Possible values are: `SQUARED_L2_DISTANCE`, `COSINE_DISTANCE`, `DOT_PRODUCT_DISTANCE`.
    #[builder(into)]
    pub r#distance_measure_type: Option<String>,
    /// Column of embedding. This column contains the source data to create index for vector search.
    #[builder(into)]
    pub r#embedding_column: String,
    /// The number of dimensions of the input embedding.
    #[builder(into)]
    pub r#embedding_dimension: Option<i32>,
    /// Columns of features that are used to filter vector search results.
    #[builder(into)]
    pub r#filter_columns: Option<Vec<String>>,
    /// Configuration options for the tree-AH algorithm (Shallow tree + Asymmetric Hashing). Please refer to this paper for more details: https://arxiv.org/abs/1908.10396
    /// Structure is documented below.
    #[builder(into)]
    pub r#tree_ah_config: Option<Box<super::super::types::vertex::AiFeatureOnlineStoreFeatureviewVectorSearchConfigTreeAhConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AiFeatureOnlineStoreFeatureviewVectorSearchConfig {
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
                    "bruteForceConfig",
                    &self.r#brute_force_config,
                ),
                to_pulumi_object_field(
                    "crowdingColumn",
                    &self.r#crowding_column,
                ),
                to_pulumi_object_field(
                    "distanceMeasureType",
                    &self.r#distance_measure_type,
                ),
                to_pulumi_object_field(
                    "embeddingColumn",
                    &self.r#embedding_column,
                ),
                to_pulumi_object_field(
                    "embeddingDimension",
                    &self.r#embedding_dimension,
                ),
                to_pulumi_object_field(
                    "filterColumns",
                    &self.r#filter_columns,
                ),
                to_pulumi_object_field(
                    "treeAhConfig",
                    &self.r#tree_ah_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AiFeatureOnlineStoreFeatureviewVectorSearchConfig {
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
                    r#brute_force_config: {
                        let field_value = match fields_map.get("bruteForceConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'bruteForceConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#crowding_column: {
                        let field_value = match fields_map.get("crowdingColumn") {
                            Some(value) => value,
                            None => bail!("Missing field 'crowdingColumn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#distance_measure_type: {
                        let field_value = match fields_map.get("distanceMeasureType") {
                            Some(value) => value,
                            None => bail!("Missing field 'distanceMeasureType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#embedding_column: {
                        let field_value = match fields_map.get("embeddingColumn") {
                            Some(value) => value,
                            None => bail!("Missing field 'embeddingColumn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#embedding_dimension: {
                        let field_value = match fields_map.get("embeddingDimension") {
                            Some(value) => value,
                            None => bail!("Missing field 'embeddingDimension' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_columns: {
                        let field_value = match fields_map.get("filterColumns") {
                            Some(value) => value,
                            None => bail!("Missing field 'filterColumns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tree_ah_config: {
                        let field_value = match fields_map.get("treeAhConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'treeAhConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
