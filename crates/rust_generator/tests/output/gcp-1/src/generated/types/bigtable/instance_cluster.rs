#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceCluster {
    /// [Autoscaling](https://cloud.google.com/bigtable/docs/autoscaling#parameters) config for the cluster, contains the following arguments:
    #[builder(into)]
    #[serde(rename = "autoscalingConfig")]
    pub r#autoscaling_config: Option<Box<super::super::types::bigtable::InstanceClusterAutoscalingConfig>>,
    /// The ID of the Cloud Bigtable cluster. Must be 6-30 characters and must only contain hyphens, lowercase letters and numbers.
    #[builder(into)]
    #[serde(rename = "clusterId")]
    pub r#cluster_id: String,
    /// Describes the Cloud KMS encryption key that will be used to protect the destination Bigtable cluster. The requirements for this key are: 1) The Cloud Bigtable service account associated with the project that contains this cluster must be granted the `cloudkms.cryptoKeyEncrypterDecrypter` role on the CMEK key. 2) Only regional keys can be used and the region of the CMEK key must match the region of the cluster.
    /// 
    /// > **Note**: Removing the field entirely from the config will cause the provider to default to the backend value.
    /// 
    /// !> **Warning**: Modifying this field will cause the provider to delete/recreate the entire resource.
    /// 
    /// !> **Warning:** Modifying the `storage_type`, `zone` or `kms_key_name` of an existing cluster (by
    /// `cluster_id`) will cause the provider to delete/recreate the entire
    /// `gcp.bigtable.Instance` resource. If these values are changing, use a new
    /// `cluster_id`.
    #[builder(into)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Option<String>,
    /// The number of nodes in the cluster.
    /// If no value is set, Cloud Bigtable automatically allocates nodes based on your data footprint and optimized for 50% storage utilization.
    #[builder(into)]
    #[serde(rename = "numNodes")]
    pub r#num_nodes: Option<i32>,
    /// describes the current state of the cluster.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// The storage type to use. One of `"SSD"` or
    /// `"HDD"`. Defaults to `"SSD"`.
    #[builder(into)]
    #[serde(rename = "storageType")]
    pub r#storage_type: Option<String>,
    /// The zone to create the Cloud Bigtable cluster in. If it not
    /// specified, the provider zone is used. Each cluster must have a different zone in the same region. Zones that support
    /// Bigtable instances are noted on the [Cloud Bigtable locations page](https://cloud.google.com/bigtable/docs/locations).
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceCluster {
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
                "autoscaling_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#autoscaling_config,
                )
                .await,
            );
            map.insert(
                "cluster_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_id,
                )
                .await,
            );
            map.insert(
                "kms_key_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kms_key_name,
                )
                .await,
            );
            map.insert(
                "num_nodes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#num_nodes,
                )
                .await,
            );
            map.insert(
                "state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state,
                )
                .await,
            );
            map.insert(
                "storage_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_type,
                )
                .await,
            );
            map.insert(
                "zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zone,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceCluster {
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
                    r#autoscaling_config: {
                        let field_value = match fields_map.get("autoscaling_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscaling_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_id: {
                        let field_value = match fields_map.get("cluster_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_name: {
                        let field_value = match fields_map.get("kms_key_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#num_nodes: {
                        let field_value = match fields_map.get("num_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'num_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_type: {
                        let field_value = match fields_map.get("storage_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zone: {
                        let field_value = match fields_map.get("zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
