#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSnapshotClusterConfiguration {
    /// Description for the cluster.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The engine that will run on cluster nodes.
    #[builder(into)]
    #[serde(rename = "engine")]
    pub r#engine: String,
    /// Version number of the engine used by the cluster.
    #[builder(into)]
    #[serde(rename = "engineVersion")]
    pub r#engine_version: String,
    /// The weekly time range during which maintenance on the cluster is performed.
    #[builder(into)]
    #[serde(rename = "maintenanceWindow")]
    pub r#maintenance_window: String,
    /// Name of the snapshot.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Compute and memory capacity of the nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "nodeType")]
    pub r#node_type: String,
    /// Number of shards in the cluster.
    #[builder(into)]
    #[serde(rename = "numShards")]
    pub r#num_shards: i32,
    /// Name of the parameter group associated with the cluster.
    #[builder(into)]
    #[serde(rename = "parameterGroupName")]
    pub r#parameter_group_name: String,
    /// Port number on which the cluster accepts connections.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// Number of days for which MemoryDB retains automatic snapshots before deleting them.
    #[builder(into)]
    #[serde(rename = "snapshotRetentionLimit")]
    pub r#snapshot_retention_limit: i32,
    /// The daily time range (in UTC) during which MemoryDB begins taking a daily snapshot of the shard.
    #[builder(into)]
    #[serde(rename = "snapshotWindow")]
    pub r#snapshot_window: String,
    /// Name of the subnet group used by the cluster.
    #[builder(into)]
    #[serde(rename = "subnetGroupName")]
    pub r#subnet_group_name: String,
    /// ARN of the SNS topic to which cluster notifications are sent.
    #[builder(into)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: String,
    /// The VPC in which the cluster exists.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSnapshotClusterConfiguration {
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
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "engine".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#engine,
                )
                .await,
            );
            map.insert(
                "engine_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#engine_version,
                )
                .await,
            );
            map.insert(
                "maintenance_window".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maintenance_window,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "node_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_type,
                )
                .await,
            );
            map.insert(
                "num_shards".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#num_shards,
                )
                .await,
            );
            map.insert(
                "parameter_group_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parameter_group_name,
                )
                .await,
            );
            map.insert(
                "port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#port,
                )
                .await,
            );
            map.insert(
                "snapshot_retention_limit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#snapshot_retention_limit,
                )
                .await,
            );
            map.insert(
                "snapshot_window".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#snapshot_window,
                )
                .await,
            );
            map.insert(
                "subnet_group_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnet_group_name,
                )
                .await,
            );
            map.insert(
                "topic_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#topic_arn,
                )
                .await,
            );
            map.insert(
                "vpc_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpc_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSnapshotClusterConfiguration {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#engine: {
                        let field_value = match fields_map.get("engine") {
                            Some(value) => value,
                            None => bail!("Missing field 'engine' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#engine_version: {
                        let field_value = match fields_map.get("engine_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'engine_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_window: {
                        let field_value = match fields_map.get("maintenance_window") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenance_window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_type: {
                        let field_value = match fields_map.get("node_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#num_shards: {
                        let field_value = match fields_map.get("num_shards") {
                            Some(value) => value,
                            None => bail!("Missing field 'num_shards' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parameter_group_name: {
                        let field_value = match fields_map.get("parameter_group_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameter_group_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot_retention_limit: {
                        let field_value = match fields_map.get("snapshot_retention_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshot_retention_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot_window: {
                        let field_value = match fields_map.get("snapshot_window") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshot_window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_group_name: {
                        let field_value = match fields_map.get("subnet_group_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_group_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topic_arn: {
                        let field_value = match fields_map.get("topic_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'topic_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_id: {
                        let field_value = match fields_map.get("vpc_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
