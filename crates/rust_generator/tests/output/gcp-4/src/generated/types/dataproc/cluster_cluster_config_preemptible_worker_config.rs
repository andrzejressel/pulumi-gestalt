#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterConfigPreemptibleWorkerConfig {
    /// Disk Config
    #[builder(into)]
    #[serde(rename = "diskConfig")]
    pub r#disk_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigPreemptibleWorkerConfigDiskConfig>>,
    /// Instance flexibility Policy allowing a mixture of VM shapes and provisioning models.
    #[builder(into)]
    #[serde(rename = "instanceFlexibilityPolicy")]
    pub r#instance_flexibility_policy: Option<Box<super::super::types::dataproc::ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicy>>,
    /// List of preemptible instance names which have been assigned
    /// to the cluster.
    #[builder(into)]
    #[serde(rename = "instanceNames")]
    pub r#instance_names: Option<Vec<String>>,
    /// Specifies the number of preemptible nodes to create.
    /// Defaults to 0.
    #[builder(into)]
    #[serde(rename = "numInstances")]
    pub r#num_instances: Option<i32>,
    /// Specifies the preemptibility of the secondary workers. The default value is `PREEMPTIBLE`
    /// Accepted values are:
    /// * PREEMPTIBILITY_UNSPECIFIED
    /// * NON_PREEMPTIBLE
    /// * PREEMPTIBLE
    #[builder(into)]
    #[serde(rename = "preemptibility")]
    pub r#preemptibility: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterClusterConfigPreemptibleWorkerConfig {
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
                "disk_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disk_config,
                )
                .await,
            );
            map.insert(
                "instance_flexibility_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_flexibility_policy,
                )
                .await,
            );
            map.insert(
                "instance_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_names,
                )
                .await,
            );
            map.insert(
                "num_instances".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#num_instances,
                )
                .await,
            );
            map.insert(
                "preemptibility".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preemptibility,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterClusterConfigPreemptibleWorkerConfig {
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
                    r#disk_config: {
                        let field_value = match fields_map.get("disk_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_flexibility_policy: {
                        let field_value = match fields_map.get("instance_flexibility_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_flexibility_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_names: {
                        let field_value = match fields_map.get("instance_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#num_instances: {
                        let field_value = match fields_map.get("num_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'num_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preemptibility: {
                        let field_value = match fields_map.get("preemptibility") {
                            Some(value) => value,
                            None => bail!("Missing field 'preemptibility' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
