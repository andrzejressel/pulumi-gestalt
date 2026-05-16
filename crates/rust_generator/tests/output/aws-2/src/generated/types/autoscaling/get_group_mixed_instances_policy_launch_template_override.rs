#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGroupMixedInstancesPolicyLaunchTemplateOverride {
    /// List of instance requirements objects.
    /// * `accelerator_count - List of objects describing the minimum and maximum number of accelerators for an instance type.
    #[builder(into)]
    #[serde(rename = "instanceRequirements")]
    pub r#instance_requirements: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirement>,
    /// Overriding instance type.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: String,
    /// List of overriding launch template specification objects.
    #[builder(into)]
    #[serde(rename = "launchTemplateSpecifications")]
    pub r#launch_template_specifications: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideLaunchTemplateSpecification>,
    /// Number of capacity units, which gives the instance type a proportional weight to other instance types.
    #[builder(into)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetGroupMixedInstancesPolicyLaunchTemplateOverride {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("instance_requirements".to_string(), self.r#instance_requirements.to_pulumi_value().await);
            map.insert("instance_type".to_string(), self.r#instance_type.to_pulumi_value().await);
            map.insert("launch_template_specifications".to_string(), self.r#launch_template_specifications.to_pulumi_value().await);
            map.insert("weighted_capacity".to_string(), self.r#weighted_capacity.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetGroupMixedInstancesPolicyLaunchTemplateOverride {
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
                    r#instance_requirements: {
                        let field_value = match fields_map.get("instance_requirements") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_requirements' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirement> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#instance_type: {
                        let field_value = match fields_map.get("instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#launch_template_specifications: {
                        let field_value = match fields_map.get("launch_template_specifications") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_template_specifications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideLaunchTemplateSpecification> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#weighted_capacity: {
                        let field_value = match fields_map.get("weighted_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'weighted_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
