#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupMixedInstancesPolicyLaunchTemplateOverride {
    /// Override the instance type in the Launch Template with instance types that satisfy the requirements.
    #[builder(into)]
    #[serde(rename = "instanceRequirements")]
    pub r#instance_requirements: Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirements>>,
    /// Override the instance type in the Launch Template.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Option<String>,
    /// Override the instance launch template specification in the Launch Template.
    #[builder(into)]
    #[serde(rename = "launchTemplateSpecification")]
    pub r#launch_template_specification: Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideLaunchTemplateSpecification>>,
    /// Number of capacity units, which gives the instance type a proportional weight to other instance types.
    #[builder(into)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GroupMixedInstancesPolicyLaunchTemplateOverride {
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
                "instance_requirements".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_requirements,
                )
                .await,
            );
            map.insert(
                "instance_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_type,
                )
                .await,
            );
            map.insert(
                "launch_template_specification".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#launch_template_specification,
                )
                .await,
            );
            map.insert(
                "weighted_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weighted_capacity,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GroupMixedInstancesPolicyLaunchTemplateOverride {
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
                    r#instance_requirements: {
                        let field_value = match fields_map.get("instance_requirements") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_requirements' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_type: {
                        let field_value = match fields_map.get("instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_template_specification: {
                        let field_value = match fields_map.get("launch_template_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_template_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weighted_capacity: {
                        let field_value = match fields_map.get("weighted_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'weighted_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
