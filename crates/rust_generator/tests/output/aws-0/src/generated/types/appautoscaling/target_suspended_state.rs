#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TargetSuspendedState {
    /// Whether scale in by a target tracking scaling policy or a step scaling policy is suspended. Default is `false`.
    #[builder(into)]
    #[serde(rename = "dynamicScalingInSuspended")]
    pub r#dynamic_scaling_in_suspended: Option<bool>,
    /// Whether scale out by a target tracking scaling policy or a step scaling policy is suspended. Default is `false`.
    #[builder(into)]
    #[serde(rename = "dynamicScalingOutSuspended")]
    pub r#dynamic_scaling_out_suspended: Option<bool>,
    /// Whether scheduled scaling is suspended. Default is `false`.
    #[builder(into)]
    #[serde(rename = "scheduledScalingSuspended")]
    pub r#scheduled_scaling_suspended: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TargetSuspendedState {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("dynamic_scaling_in_suspended".to_string(), self.r#dynamic_scaling_in_suspended.to_pulumi_value().await);
            map.insert("dynamic_scaling_out_suspended".to_string(), self.r#dynamic_scaling_out_suspended.to_pulumi_value().await);
            map.insert("scheduled_scaling_suspended".to_string(), self.r#scheduled_scaling_suspended.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TargetSuspendedState {
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
                    r#dynamic_scaling_in_suspended: {
                        let field_value = match fields_map.get("dynamic_scaling_in_suspended") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamic_scaling_in_suspended' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dynamic_scaling_out_suspended: {
                        let field_value = match fields_map.get("dynamic_scaling_out_suspended") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamic_scaling_out_suspended' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#scheduled_scaling_suspended: {
                        let field_value = match fields_map.get("scheduled_scaling_suspended") {
                            Some(value) => value,
                            None => bail!("Missing field 'scheduled_scaling_suspended' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
