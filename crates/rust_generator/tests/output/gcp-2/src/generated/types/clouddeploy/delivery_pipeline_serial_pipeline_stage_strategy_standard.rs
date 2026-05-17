#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeliveryPipelineSerialPipelineStageStrategyStandard {
    /// Optional. Configuration for the postdeploy job. If this is not configured, postdeploy job will not be present.
    #[builder(into)]
    #[serde(rename = "postdeploy")]
    pub r#postdeploy: Option<Box<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyStandardPostdeploy>>,
    /// Optional. Configuration for the predeploy job. If this is not configured, predeploy job will not be present.
    #[builder(into)]
    #[serde(rename = "predeploy")]
    pub r#predeploy: Option<Box<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyStandardPredeploy>>,
    /// Whether to verify a deployment.
    #[builder(into)]
    #[serde(rename = "verify")]
    pub r#verify: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeliveryPipelineSerialPipelineStageStrategyStandard {
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
                "postdeploy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#postdeploy,
                )
                .await,
            );
            map.insert(
                "predeploy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#predeploy,
                )
                .await,
            );
            map.insert(
                "verify".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#verify,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeliveryPipelineSerialPipelineStageStrategyStandard {
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
                    r#postdeploy: {
                        let field_value = match fields_map.get("postdeploy") {
                            Some(value) => value,
                            None => bail!("Missing field 'postdeploy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#predeploy: {
                        let field_value = match fields_map.get("predeploy") {
                            Some(value) => value,
                            None => bail!("Missing field 'predeploy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verify: {
                        let field_value = match fields_map.get("verify") {
                            Some(value) => value,
                            None => bail!("Missing field 'verify' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
