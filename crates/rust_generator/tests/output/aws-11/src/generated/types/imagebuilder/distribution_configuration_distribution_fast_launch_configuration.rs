#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionConfigurationDistributionFastLaunchConfiguration {
    /// The owner account ID for the fast-launch enabled Windows AMI.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: String,
    /// A Boolean that represents the current state of faster launching for the Windows AMI. Set to `true` to start using Windows faster launching, or `false` to stop using it.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Configuration block for the launch template that the fast-launch enabled Windows AMI uses when it launches Windows instances to create pre-provisioned snapshots. Detailed below.
    #[builder(into)]
    #[serde(rename = "launchTemplate")]
    pub r#launch_template: Option<Box<super::super::types::imagebuilder::DistributionConfigurationDistributionFastLaunchConfigurationLaunchTemplate>>,
    /// The maximum number of parallel instances that are launched for creating resources.
    #[builder(into)]
    #[serde(rename = "maxParallelLaunches")]
    pub r#max_parallel_launches: Option<i32>,
    /// Configuration block for managing the number of snapshots that are created from pre-provisioned instances for the Windows AMI when faster launching is enabled. Detailed below.
    #[builder(into)]
    #[serde(rename = "snapshotConfiguration")]
    pub r#snapshot_configuration: Option<Box<super::super::types::imagebuilder::DistributionConfigurationDistributionFastLaunchConfigurationSnapshotConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionConfigurationDistributionFastLaunchConfiguration {
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
                "account_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#account_id,
                )
                .await,
            );
            map.insert(
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "launch_template".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#launch_template,
                )
                .await,
            );
            map.insert(
                "max_parallel_launches".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_parallel_launches,
                )
                .await,
            );
            map.insert(
                "snapshot_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#snapshot_configuration,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionConfigurationDistributionFastLaunchConfiguration {
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
                    r#launch_template: {
                        let field_value = match fields_map.get("launch_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#snapshot_configuration: {
                        let field_value = match fields_map.get("snapshot_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshot_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
