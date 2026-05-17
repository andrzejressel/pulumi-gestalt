#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeliveryPipelineSerialPipelineStage {
    /// Optional. The deploy parameters to use for the target in this stage.
    #[builder(into)]
    #[serde(rename = "deployParameters")]
    pub r#deploy_parameters: Option<Vec<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageDeployParameter>>,
    /// Skaffold profiles to use when rendering the manifest for this stage's `Target`.
    #[builder(into)]
    #[serde(rename = "profiles")]
    pub r#profiles: Option<Vec<String>>,
    /// Optional. The strategy to use for a `Rollout` to this stage.
    #[builder(into)]
    #[serde(rename = "strategy")]
    pub r#strategy: Option<Box<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategy>>,
    /// The target_id to which this stage points. This field refers exclusively to the last segment of a target name. For example, this field would just be `my-target` (rather than `projects/project/locations/location/targets/my-target`). The location of the `Target` is inferred to be the same as the location of the `DeliveryPipeline` that contains this `Stage`.
    #[builder(into)]
    #[serde(rename = "targetId")]
    pub r#target_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeliveryPipelineSerialPipelineStage {
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
                "deploy_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#deploy_parameters,
                )
                .await,
            );
            map.insert(
                "profiles".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#profiles,
                )
                .await,
            );
            map.insert(
                "strategy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#strategy,
                )
                .await,
            );
            map.insert(
                "target_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeliveryPipelineSerialPipelineStage {
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
                    r#deploy_parameters: {
                        let field_value = match fields_map.get("deploy_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'deploy_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#profiles: {
                        let field_value = match fields_map.get("profiles") {
                            Some(value) => value,
                            None => bail!("Missing field 'profiles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#strategy: {
                        let field_value = match fields_map.get("strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_id: {
                        let field_value = match fields_map.get("target_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
