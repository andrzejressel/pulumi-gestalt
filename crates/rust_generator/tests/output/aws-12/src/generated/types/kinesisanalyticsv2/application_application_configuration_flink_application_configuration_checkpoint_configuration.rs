#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationFlinkApplicationConfigurationCheckpointConfiguration {
    /// Describes the interval in milliseconds between checkpoint operations.
    #[builder(into)]
    #[serde(rename = "checkpointInterval")]
    pub r#checkpoint_interval: Option<i32>,
    /// Describes whether checkpointing is enabled for a Flink-based Kinesis Data Analytics application.
    #[builder(into)]
    #[serde(rename = "checkpointingEnabled")]
    pub r#checkpointing_enabled: Option<bool>,
    /// Describes whether the application uses Kinesis Data Analytics' default checkpointing behavior. Valid values: `CUSTOM`, `DEFAULT`. Set this attribute to `CUSTOM` in order for any specified `checkpointing_enabled`, `checkpoint_interval`, or `min_pause_between_checkpoints` attribute values to be effective. If this attribute is set to `DEFAULT`, the application will always use the following values:
    /// * `checkpointing_enabled = true`
    /// * `checkpoint_interval = 60000`
    /// * `min_pause_between_checkpoints = 5000`
    #[builder(into)]
    #[serde(rename = "configurationType")]
    pub r#configuration_type: String,
    /// Describes the minimum time in milliseconds after a checkpoint operation completes that a new checkpoint operation can start.
    #[builder(into)]
    #[serde(rename = "minPauseBetweenCheckpoints")]
    pub r#min_pause_between_checkpoints: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationApplicationConfigurationFlinkApplicationConfigurationCheckpointConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "checkpoint_interval",
                    &self.r#checkpoint_interval,
                ),
                to_pulumi_object_field(
                    "checkpointing_enabled",
                    &self.r#checkpointing_enabled,
                ),
                to_pulumi_object_field(
                    "configuration_type",
                    &self.r#configuration_type,
                ),
                to_pulumi_object_field(
                    "min_pause_between_checkpoints",
                    &self.r#min_pause_between_checkpoints,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationApplicationConfigurationFlinkApplicationConfigurationCheckpointConfiguration {
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
                    r#checkpoint_interval: {
                        let field_value = match fields_map.get("checkpoint_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'checkpoint_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#checkpointing_enabled: {
                        let field_value = match fields_map.get("checkpointing_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'checkpointing_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#configuration_type: {
                        let field_value = match fields_map.get("configuration_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'configuration_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_pause_between_checkpoints: {
                        let field_value = match fields_map.get("min_pause_between_checkpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_pause_between_checkpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
