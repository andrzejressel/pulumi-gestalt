#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutoscalerAutoscalingPolicyScaleDownControl {
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "maxScaledDownReplicas")]
    pub r#max_scaled_down_replicas: Option<Box<super::super::types::compute::AutoscalerAutoscalingPolicyScaleDownControlMaxScaledDownReplicas>>,
    /// How long back autoscaling should look when computing recommendations
    /// to include directives regarding slower scale down, as described above.
    #[builder(into)]
    #[serde(rename = "timeWindowSec")]
    pub r#time_window_sec: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutoscalerAutoscalingPolicyScaleDownControl {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "max_scaled_down_replicas".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_scaled_down_replicas,
                )
                .await,
            );
            map.insert(
                "time_window_sec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_window_sec,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AutoscalerAutoscalingPolicyScaleDownControl {
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
                    r#max_scaled_down_replicas: {
                        let field_value = match fields_map.get("max_scaled_down_replicas") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_scaled_down_replicas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_window_sec: {
                        let field_value = match fields_map.get("time_window_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_window_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
