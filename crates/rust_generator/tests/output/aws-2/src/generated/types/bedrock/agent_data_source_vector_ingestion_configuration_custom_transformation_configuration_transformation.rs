#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationTransformation {
    /// When the service applies the transformation. Currently only `POST_CHUNKING` is supported.
    #[builder(into)]
    #[serde(rename = "stepToApply")]
    pub r#step_to_apply: String,
    /// The lambda function that processes documents.
    #[builder(into)]
    #[serde(rename = "transformationFunction")]
    pub r#transformation_function: Option<Box<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationTransformationTransformationFunction>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationTransformation {
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
                "step_to_apply".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#step_to_apply,
                )
                .await,
            );
            map.insert(
                "transformation_function".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transformation_function,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationTransformation {
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
                    r#step_to_apply: {
                        let field_value = match fields_map.get("step_to_apply") {
                            Some(value) => value,
                            None => bail!("Missing field 'step_to_apply' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transformation_function: {
                        let field_value = match fields_map.get("transformation_function") {
                            Some(value) => value,
                            None => bail!("Missing field 'transformation_function' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
