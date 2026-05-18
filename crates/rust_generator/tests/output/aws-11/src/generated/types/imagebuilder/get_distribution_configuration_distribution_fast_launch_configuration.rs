#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDistributionConfigurationDistributionFastLaunchConfiguration {
    /// The account ID that this configuration applies to.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: String,
    /// A Boolean that represents the current state of faster launching for the Windows AMI.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Nested list of launch templates that the fast-launch enabled Windows AMI uses when it launches Windows instances to create pre-provisioned snapshots.
    #[builder(into)]
    #[serde(rename = "launchTemplates")]
    pub r#launch_templates: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionFastLaunchConfigurationLaunchTemplate>,
    /// The maximum number of parallel instances that are launched for creating resources.
    #[builder(into)]
    #[serde(rename = "maxParallelLaunches")]
    pub r#max_parallel_launches: i32,
    /// Nested list of configurations for managing the number of snapshots that are created from pre-provisioned instances for the Windows AMI when faster launching is enabled.
    #[builder(into)]
    #[serde(rename = "snapshotConfigurations")]
    pub r#snapshot_configurations: Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionFastLaunchConfigurationSnapshotConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDistributionConfigurationDistributionFastLaunchConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "account_id",
                    &self.r#account_id,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "launch_templates",
                    &self.r#launch_templates,
                ),
                to_pulumi_object_field(
                    "max_parallel_launches",
                    &self.r#max_parallel_launches,
                ),
                to_pulumi_object_field(
                    "snapshot_configurations",
                    &self.r#snapshot_configurations,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDistributionConfigurationDistributionFastLaunchConfiguration {
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
                    r#account_id: {
                        let field_value = match fields_map.get("account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_templates: {
                        let field_value = match fields_map.get("launch_templates") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_templates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_parallel_launches: {
                        let field_value = match fields_map.get("max_parallel_launches") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_parallel_launches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot_configurations: {
                        let field_value = match fields_map.get("snapshot_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshot_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
