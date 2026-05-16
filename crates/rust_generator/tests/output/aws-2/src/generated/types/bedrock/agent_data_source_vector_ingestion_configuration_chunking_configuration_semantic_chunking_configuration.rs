#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentDataSourceVectorIngestionConfigurationChunkingConfigurationSemanticChunkingConfiguration {
    /// The dissimilarity threshold for splitting chunks.
    #[builder(into)]
    #[serde(rename = "breakpointPercentileThreshold")]
    pub r#breakpoint_percentile_threshold: f64,
    /// The buffer size.
    #[builder(into)]
    #[serde(rename = "bufferSize")]
    pub r#buffer_size: f64,
    /// The maximum number of tokens a chunk can contain.
    #[builder(into)]
    #[serde(rename = "maxToken")]
    pub r#max_token: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentDataSourceVectorIngestionConfigurationChunkingConfigurationSemanticChunkingConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("breakpoint_percentile_threshold".to_string(), self.r#breakpoint_percentile_threshold.to_pulumi_value().await);
            map.insert("buffer_size".to_string(), self.r#buffer_size.to_pulumi_value().await);
            map.insert("max_token".to_string(), self.r#max_token.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentDataSourceVectorIngestionConfigurationChunkingConfigurationSemanticChunkingConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#breakpoint_percentile_threshold: {
                        let field_value = match fields_map.get("breakpoint_percentile_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'breakpoint_percentile_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <f64 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#buffer_size: {
                        let field_value = match fields_map.get("buffer_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'buffer_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <f64 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_token: {
                        let field_value = match fields_map.get("max_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <f64 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
