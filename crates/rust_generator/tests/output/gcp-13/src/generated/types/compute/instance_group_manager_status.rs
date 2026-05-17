#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceGroupManagerStatus {
    /// Properties to set on all instances in the group. After setting
    /// allInstancesConfig on the group, you must update the group's instances to
    /// apply the configuration.
    #[builder(into)]
    #[serde(rename = "allInstancesConfigs")]
    pub r#all_instances_configs: Option<Vec<super::super::types::compute::InstanceGroupManagerStatusAllInstancesConfig>>,
    /// A bit indicating whether the managed instance group is in a stable state. A stable state means that: none of the instances in the managed instance group is currently undergoing any type of change (for example, creation, restart, or deletion); no future changes are scheduled for instances in the managed instance group; and the managed instance group itself is not being modified.
    #[builder(into)]
    #[serde(rename = "isStable")]
    pub r#is_stable: Option<bool>,
    /// Stateful status of the given Instance Group Manager.
    #[builder(into)]
    #[serde(rename = "statefuls")]
    pub r#statefuls: Option<Vec<super::super::types::compute::InstanceGroupManagerStatusStateful>>,
    /// A bit indicating whether version target has been reached in this managed instance group, i.e. all instances are in their target version. Instances' target version are specified by version field on Instance Group Manager.
    #[builder(into)]
    #[serde(rename = "versionTargets")]
    pub r#version_targets: Option<Vec<super::super::types::compute::InstanceGroupManagerStatusVersionTarget>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceGroupManagerStatus {
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
                "all_instances_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#all_instances_configs,
                )
                .await,
            );
            map.insert(
                "is_stable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_stable,
                )
                .await,
            );
            map.insert(
                "statefuls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#statefuls,
                )
                .await,
            );
            map.insert(
                "version_targets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version_targets,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceGroupManagerStatus {
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
                    r#all_instances_configs: {
                        let field_value = match fields_map.get("all_instances_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'all_instances_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_stable: {
                        let field_value = match fields_map.get("is_stable") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_stable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#statefuls: {
                        let field_value = match fields_map.get("statefuls") {
                            Some(value) => value,
                            None => bail!("Missing field 'statefuls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version_targets: {
                        let field_value = match fields_map.get("version_targets") {
                            Some(value) => value,
                            None => bail!("Missing field 'version_targets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
