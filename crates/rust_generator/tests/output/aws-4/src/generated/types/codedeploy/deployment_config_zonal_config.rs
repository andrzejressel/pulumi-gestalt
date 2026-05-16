#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeploymentConfigZonalConfig {
    /// The period of time, in seconds, that CodeDeploy must wait after completing a deployment to the first Availability Zone. CodeDeploy will wait this amount of time before starting a deployment to the second Availability Zone. If you don't specify a value for `first_zone_monitor_duration_in_seconds`, then CodeDeploy uses the `monitor_duration_in_seconds` value for the first Availability Zone.
    #[builder(into)]
    #[serde(rename = "firstZoneMonitorDurationInSeconds")]
    pub r#first_zone_monitor_duration_in_seconds: Option<i32>,
    /// The number or percentage of instances that must remain available per Availability Zone during a deployment. If you don't specify a value under `minimum_healthy_hosts_per_zone`, then CodeDeploy uses a default value of 0 percent. This block is more documented below.
    #[builder(into)]
    #[serde(rename = "minimumHealthyHostsPerZone")]
    pub r#minimum_healthy_hosts_per_zone: Option<Box<super::super::types::codedeploy::DeploymentConfigZonalConfigMinimumHealthyHostsPerZone>>,
    /// The period of time, in seconds, that CodeDeploy must wait after completing a deployment to an Availability Zone. CodeDeploy will wait this amount of time before starting a deployment to the next Availability Zone. If you don't specify a `monitor_duration_in_seconds`, CodeDeploy starts deploying to the next Availability Zone immediately.
    #[builder(into)]
    #[serde(rename = "monitorDurationInSeconds")]
    pub r#monitor_duration_in_seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeploymentConfigZonalConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("first_zone_monitor_duration_in_seconds".to_string(), self.r#first_zone_monitor_duration_in_seconds.to_pulumi_value().await);
            map.insert("minimum_healthy_hosts_per_zone".to_string(), self.r#minimum_healthy_hosts_per_zone.to_pulumi_value().await);
            map.insert("monitor_duration_in_seconds".to_string(), self.r#monitor_duration_in_seconds.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeploymentConfigZonalConfig {
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
                    r#first_zone_monitor_duration_in_seconds: {
                        let field_value = match fields_map.get("first_zone_monitor_duration_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'first_zone_monitor_duration_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#minimum_healthy_hosts_per_zone: {
                        let field_value = match fields_map.get("minimum_healthy_hosts_per_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_healthy_hosts_per_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::codedeploy::DeploymentConfigZonalConfigMinimumHealthyHostsPerZone>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#monitor_duration_in_seconds: {
                        let field_value = match fields_map.get("monitor_duration_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'monitor_duration_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
