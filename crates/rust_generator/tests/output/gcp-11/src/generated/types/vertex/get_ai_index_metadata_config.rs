#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAiIndexMetadataConfig {
    /// The configuration with regard to the algorithms used for efficient search.
    #[builder(into)]
    #[serde(rename = "algorithmConfigs")]
    pub r#algorithm_configs: Vec<super::super::types::vertex::GetAiIndexMetadataConfigAlgorithmConfig>,
    /// The default number of neighbors to find via approximate search before exact reordering is
    /// performed. Exact reordering is a procedure where results returned by an
    /// approximate search algorithm are reordered via a more expensive distance computation.
    /// Required if tree-AH algorithm is used.
    #[builder(into)]
    #[serde(rename = "approximateNeighborsCount")]
    pub r#approximate_neighbors_count: i32,
    /// The number of dimensions of the input vectors.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: i32,
    /// The distance measure used in nearest neighbor search. The value must be one of the followings:
    /// * SQUARED_L2_DISTANCE: Euclidean (L_2) Distance
    /// * L1_DISTANCE: Manhattan (L_1) Distance
    /// * COSINE_DISTANCE: Cosine Distance. Defined as 1 - cosine similarity.
    /// * DOT_PRODUCT_DISTANCE: Dot Product Distance. Defined as a negative of the dot product
    #[builder(into)]
    #[serde(rename = "distanceMeasureType")]
    pub r#distance_measure_type: String,
    /// Type of normalization to be carried out on each vector. The value must be one of the followings:
    /// * UNIT_L2_NORM: Unit L2 normalization type
    /// * NONE: No normalization type is specified.
    #[builder(into)]
    #[serde(rename = "featureNormType")]
    pub r#feature_norm_type: String,
    /// Index data is split into equal parts to be processed. These are called "shards".
    /// The shard size must be specified when creating an index. The value must be one of the followings:
    /// * SHARD_SIZE_SMALL: Small (2GB)
    /// * SHARD_SIZE_MEDIUM: Medium (20GB)
    /// * SHARD_SIZE_LARGE: Large (50GB)
    #[builder(into)]
    #[serde(rename = "shardSize")]
    pub r#shard_size: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAiIndexMetadataConfig {
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
                "algorithm_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#algorithm_configs,
                )
                .await,
            );
            map.insert(
                "approximate_neighbors_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#approximate_neighbors_count,
                )
                .await,
            );
            map.insert(
                "dimensions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dimensions,
                )
                .await,
            );
            map.insert(
                "distance_measure_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#distance_measure_type,
                )
                .await,
            );
            map.insert(
                "feature_norm_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#feature_norm_type,
                )
                .await,
            );
            map.insert(
                "shard_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#shard_size,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAiIndexMetadataConfig {
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
                    r#algorithm_configs: {
                        let field_value = match fields_map.get("algorithm_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'algorithm_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#approximate_neighbors_count: {
                        let field_value = match fields_map.get("approximate_neighbors_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'approximate_neighbors_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dimensions: {
                        let field_value = match fields_map.get("dimensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#distance_measure_type: {
                        let field_value = match fields_map.get("distance_measure_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'distance_measure_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#feature_norm_type: {
                        let field_value = match fields_map.get("feature_norm_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'feature_norm_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shard_size: {
                        let field_value = match fields_map.get("shard_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'shard_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
