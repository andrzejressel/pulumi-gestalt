#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupMixedInstancesPolicy {
    /// Nested argument containing settings on how to mix on-demand and Spot instances in the Auto Scaling group. Defined below.
    #[builder(into)]
    #[serde(rename = "instancesDistribution")]
    pub r#instances_distribution: Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyInstancesDistribution>>,
    /// Nested argument containing launch template settings along with the overrides to specify multiple instance types and weights. Defined below.
    #[builder(into)]
    #[serde(rename = "launchTemplate")]
    pub r#launch_template: Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplate>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GroupMixedInstancesPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("instances_distribution".to_string(), self.r#instances_distribution.to_pulumi_value().await);
            map.insert("launch_template".to_string(), self.r#launch_template.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GroupMixedInstancesPolicy {
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
                    r#instances_distribution: {
                        let field_value = match fields_map.get("instances_distribution") {
                            Some(value) => value,
                            None => bail!("Missing field 'instances_distribution' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyInstancesDistribution>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#launch_template: {
                        let field_value = match fields_map.get("launch_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplate> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
