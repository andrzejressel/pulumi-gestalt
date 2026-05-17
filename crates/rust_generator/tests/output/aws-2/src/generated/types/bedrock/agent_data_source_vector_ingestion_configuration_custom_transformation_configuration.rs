#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentDataSourceVectorIngestionConfigurationCustomTransformationConfiguration {
    /// The intermediate storage for custom transformation.
    #[builder(into)]
    #[serde(rename = "intermediateStorage")]
    pub r#intermediate_storage: Option<Box<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationIntermediateStorage>>,
    /// A custom processing step for documents moving through the data source ingestion pipeline.
    #[builder(into)]
    #[serde(rename = "transformation")]
    pub r#transformation: Option<Box<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationTransformation>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentDataSourceVectorIngestionConfigurationCustomTransformationConfiguration {
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
                "intermediate_storage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#intermediate_storage,
                )
                .await,
            );
            map.insert(
                "transformation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transformation,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentDataSourceVectorIngestionConfigurationCustomTransformationConfiguration {
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
                    r#intermediate_storage: {
                        let field_value = match fields_map.get("intermediate_storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'intermediate_storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transformation: {
                        let field_value = match fields_map.get("transformation") {
                            Some(value) => value,
                            None => bail!("Missing field 'transformation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
