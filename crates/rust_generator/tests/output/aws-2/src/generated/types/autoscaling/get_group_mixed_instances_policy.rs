#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGroupMixedInstancesPolicy {
    /// List of instances distribution objects.
    #[builder(into)]
    #[serde(rename = "instancesDistributions")]
    pub r#instances_distributions: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyInstancesDistribution>,
    /// List of launch templates along with the overrides.
    #[builder(into)]
    #[serde(rename = "launchTemplates")]
    pub r#launch_templates: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplate>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetGroupMixedInstancesPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("instances_distributions".to_string(), self.r#instances_distributions.to_pulumi_value().await);
            map.insert("launch_templates".to_string(), self.r#launch_templates.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetGroupMixedInstancesPolicy {
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
                    r#instances_distributions: {
                        let field_value = match fields_map.get("instances_distributions") {
                            Some(value) => value,
                            None => bail!("Missing field 'instances_distributions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyInstancesDistribution> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#launch_templates: {
                        let field_value = match fields_map.get("launch_templates") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_templates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplate> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
