#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAiIndexMetadataConfigAlgorithmConfig {
    /// Configuration options for using brute force search, which simply implements the
    /// standard linear search in the database for each query.
    #[builder(into)]
    #[serde(rename = "bruteForceConfigs")]
    pub r#brute_force_configs: Vec<super::super::types::vertex::GetAiIndexMetadataConfigAlgorithmConfigBruteForceConfig>,
    /// Configuration options for using the tree-AH algorithm (Shallow tree + Asymmetric Hashing).
    /// Please refer to this paper for more details: https://arxiv.org/abs/1908.10396
    #[builder(into)]
    #[serde(rename = "treeAhConfigs")]
    pub r#tree_ah_configs: Vec<super::super::types::vertex::GetAiIndexMetadataConfigAlgorithmConfigTreeAhConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAiIndexMetadataConfigAlgorithmConfig {
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
                "brute_force_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#brute_force_configs,
                )
                .await,
            );
            map.insert(
                "tree_ah_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tree_ah_configs,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAiIndexMetadataConfigAlgorithmConfig {
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
                    r#brute_force_configs: {
                        let field_value = match fields_map.get("brute_force_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'brute_force_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tree_ah_configs: {
                        let field_value = match fields_map.get("tree_ah_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'tree_ah_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
