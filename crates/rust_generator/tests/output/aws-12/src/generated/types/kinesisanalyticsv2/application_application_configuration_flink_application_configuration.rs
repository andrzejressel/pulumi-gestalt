#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationFlinkApplicationConfiguration {
    /// Describes an application's checkpointing configuration.
    #[builder(into)]
    #[serde(rename = "checkpointConfiguration")]
    pub r#checkpoint_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationFlinkApplicationConfigurationCheckpointConfiguration>>,
    /// Describes configuration parameters for CloudWatch logging for an application.
    #[builder(into)]
    #[serde(rename = "monitoringConfiguration")]
    pub r#monitoring_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationFlinkApplicationConfigurationMonitoringConfiguration>>,
    /// Describes parameters for how an application executes multiple tasks simultaneously.
    #[builder(into)]
    #[serde(rename = "parallelismConfiguration")]
    pub r#parallelism_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationFlinkApplicationConfigurationParallelismConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationApplicationConfigurationFlinkApplicationConfiguration {
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
                "checkpoint_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#checkpoint_configuration,
                )
                .await,
            );
            map.insert(
                "monitoring_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#monitoring_configuration,
                )
                .await,
            );
            map.insert(
                "parallelism_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parallelism_configuration,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationApplicationConfigurationFlinkApplicationConfiguration {
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
                    r#checkpoint_configuration: {
                        let field_value = match fields_map.get("checkpoint_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'checkpoint_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monitoring_configuration: {
                        let field_value = match fields_map.get("monitoring_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'monitoring_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parallelism_configuration: {
                        let field_value = match fields_map.get("parallelism_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'parallelism_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
