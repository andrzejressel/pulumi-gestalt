#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupInstanceRefreshPreferences {
    /// Alarm Specification for Instance Refresh.
    #[builder(into)]
    #[serde(rename = "alarmSpecification")]
    pub r#alarm_specification: Option<Box<super::super::types::autoscaling::GroupInstanceRefreshPreferencesAlarmSpecification>>,
    /// Automatically rollback if instance refresh fails. Defaults to `false`. This option may only be set to `true` when specifying a `launch_template` or `mixed_instances_policy`.
    #[builder(into)]
    #[serde(rename = "autoRollback")]
    pub r#auto_rollback: Option<bool>,
    /// Number of seconds to wait after a checkpoint. Defaults to `3600`.
    #[builder(into)]
    #[serde(rename = "checkpointDelay")]
    pub r#checkpoint_delay: Option<String>,
    /// List of percentages for each checkpoint. Values must be unique and in ascending order. To replace all instances, the final number must be `100`.
    #[builder(into)]
    #[serde(rename = "checkpointPercentages")]
    pub r#checkpoint_percentages: Option<Vec<i32>>,
    /// Number of seconds until a newly launched instance is configured and ready to use. Default behavior is to use the Auto Scaling Group's health check grace period.
    #[builder(into)]
    #[serde(rename = "instanceWarmup")]
    pub r#instance_warmup: Option<String>,
    /// Amount of capacity in the Auto Scaling group that can be in service and healthy, or pending, to support your workload when an instance refresh is in place, as a percentage of the desired capacity of the Auto Scaling group. Values must be between `100` and `200`, defaults to `100`.
    #[builder(into)]
    #[serde(rename = "maxHealthyPercentage")]
    pub r#max_healthy_percentage: Option<i32>,
    /// Amount of capacity in the Auto Scaling group that must remain healthy during an instance refresh to allow the operation to continue, as a percentage of the desired capacity of the Auto Scaling group. Defaults to `90`.
    #[builder(into)]
    #[serde(rename = "minHealthyPercentage")]
    pub r#min_healthy_percentage: Option<i32>,
    /// Behavior when encountering instances protected from scale in are found. Available behaviors are `Refresh`, `Ignore`, and `Wait`. Default is `Ignore`.
    #[builder(into)]
    #[serde(rename = "scaleInProtectedInstances")]
    pub r#scale_in_protected_instances: Option<String>,
    /// Replace instances that already have your desired configuration. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "skipMatching")]
    pub r#skip_matching: Option<bool>,
    /// Behavior when encountering instances in the `Standby` state in are found. Available behaviors are `Terminate`, `Ignore`, and `Wait`. Default is `Ignore`.
    #[builder(into)]
    #[serde(rename = "standbyInstances")]
    pub r#standby_instances: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GroupInstanceRefreshPreferences {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("alarm_specification".to_string(), self.r#alarm_specification.to_pulumi_value().await);
            map.insert("auto_rollback".to_string(), self.r#auto_rollback.to_pulumi_value().await);
            map.insert("checkpoint_delay".to_string(), self.r#checkpoint_delay.to_pulumi_value().await);
            map.insert("checkpoint_percentages".to_string(), self.r#checkpoint_percentages.to_pulumi_value().await);
            map.insert("instance_warmup".to_string(), self.r#instance_warmup.to_pulumi_value().await);
            map.insert("max_healthy_percentage".to_string(), self.r#max_healthy_percentage.to_pulumi_value().await);
            map.insert("min_healthy_percentage".to_string(), self.r#min_healthy_percentage.to_pulumi_value().await);
            map.insert("scale_in_protected_instances".to_string(), self.r#scale_in_protected_instances.to_pulumi_value().await);
            map.insert("skip_matching".to_string(), self.r#skip_matching.to_pulumi_value().await);
            map.insert("standby_instances".to_string(), self.r#standby_instances.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GroupInstanceRefreshPreferences {
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
                    r#alarm_specification: {
                        let field_value = match fields_map.get("alarm_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'alarm_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::GroupInstanceRefreshPreferencesAlarmSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#auto_rollback: {
                        let field_value = match fields_map.get("auto_rollback") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_rollback' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#checkpoint_delay: {
                        let field_value = match fields_map.get("checkpoint_delay") {
                            Some(value) => value,
                            None => bail!("Missing field 'checkpoint_delay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#checkpoint_percentages: {
                        let field_value = match fields_map.get("checkpoint_percentages") {
                            Some(value) => value,
                            None => bail!("Missing field 'checkpoint_percentages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<i32>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#instance_warmup: {
                        let field_value = match fields_map.get("instance_warmup") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_warmup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_healthy_percentage: {
                        let field_value = match fields_map.get("max_healthy_percentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_healthy_percentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#min_healthy_percentage: {
                        let field_value = match fields_map.get("min_healthy_percentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_healthy_percentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#scale_in_protected_instances: {
                        let field_value = match fields_map.get("scale_in_protected_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_in_protected_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#skip_matching: {
                        let field_value = match fields_map.get("skip_matching") {
                            Some(value) => value,
                            None => bail!("Missing field 'skip_matching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#standby_instances: {
                        let field_value = match fields_map.get("standby_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'standby_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
