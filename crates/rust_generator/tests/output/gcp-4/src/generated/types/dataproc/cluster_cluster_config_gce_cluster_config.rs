#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterConfigGceClusterConfig {
    /// Confidential Instance Config for clusters using [Confidential VMs](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/confidential-compute)
    #[builder(into)]
    #[serde(rename = "confidentialInstanceConfig")]
    pub r#confidential_instance_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigGceClusterConfigConfidentialInstanceConfig>>,
    /// By default, clusters are not restricted to internal IP addresses,
    /// and will have ephemeral external IP addresses assigned to each instance. If set to true, all
    /// instances in the cluster will only have internal IP addresses. Note: Private Google Access
    /// (also known as `privateIpGoogleAccess`) must be enabled on the subnetwork that the cluster
    /// will be launched in.
    #[builder(into)]
    #[serde(rename = "internalIpOnly")]
    pub r#internal_ip_only: Option<bool>,
    /// A map of the Compute Engine metadata entries to add to all instances
    /// (see [Project and instance metadata](https://cloud.google.com/compute/docs/storing-retrieving-metadata#project_and_instance_metadata)).
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Option<std::collections::HashMap<String, String>>,
    /// The name or self_link of the Google Compute Engine
    /// network to the cluster will be part of. Conflicts with `subnetwork`.
    /// If neither is specified, this defaults to the "default" network.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// Node Group Affinity for sole-tenant clusters.
    #[builder(into)]
    #[serde(rename = "nodeGroupAffinity")]
    pub r#node_group_affinity: Option<Box<super::super::types::dataproc::ClusterClusterConfigGceClusterConfigNodeGroupAffinity>>,
    /// Reservation Affinity for consuming zonal reservation.
    #[builder(into)]
    #[serde(rename = "reservationAffinity")]
    pub r#reservation_affinity: Option<Box<super::super::types::dataproc::ClusterClusterConfigGceClusterConfigReservationAffinity>>,
    /// The service account to be used by the Node VMs.
    /// If not specified, the "default" service account is used.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Option<String>,
    /// The set of Google API scopes
    /// to be made available on all of the node VMs under the `service_account`
    /// specified. Both OAuth2 URLs and gcloud
    /// short names are supported. To allow full access to all Cloud APIs, use the
    /// `cloud-platform` scope. See a complete list of scopes [here](https://cloud.google.com/sdk/gcloud/reference/alpha/compute/instances/set-scopes#--scopes).
    #[builder(into)]
    #[serde(rename = "serviceAccountScopes")]
    pub r#service_account_scopes: Option<Vec<String>>,
    /// Shielded Instance Config for clusters using [Compute Engine Shielded VMs](https://cloud.google.com/security/shielded-cloud/shielded-vm).
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "shieldedInstanceConfig")]
    pub r#shielded_instance_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigGceClusterConfigShieldedInstanceConfig>>,
    /// The name or self_link of the Google Compute Engine
    /// subnetwork the cluster will be part of. Conflicts with `network`.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Option<String>,
    /// The list of instance tags applied to instances in the cluster.
    /// Tags are used to identify valid sources or targets for network firewalls.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<String>>,
    /// The GCP zone where your data is stored and used (i.e. where
    /// the master and the worker nodes will be created in). If `region` is set to 'global' (default)
    /// then `zone` is mandatory, otherwise GCP is able to make use of [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/auto-zone)
    /// to determine this automatically for you.
    /// Note: This setting additionally determines and restricts
    /// which computing resources are available for use with other configs such as
    /// `cluster_config.master_config.machine_type` and `cluster_config.worker_config.machine_type`.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterClusterConfigGceClusterConfig {
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
                    "confidential_instance_config",
                    &self.r#confidential_instance_config,
                ),
                to_pulumi_object_field(
                    "internal_ip_only",
                    &self.r#internal_ip_only,
                ),
                to_pulumi_object_field(
                    "metadata",
                    &self.r#metadata,
                ),
                to_pulumi_object_field(
                    "network",
                    &self.r#network,
                ),
                to_pulumi_object_field(
                    "node_group_affinity",
                    &self.r#node_group_affinity,
                ),
                to_pulumi_object_field(
                    "reservation_affinity",
                    &self.r#reservation_affinity,
                ),
                to_pulumi_object_field(
                    "service_account",
                    &self.r#service_account,
                ),
                to_pulumi_object_field(
                    "service_account_scopes",
                    &self.r#service_account_scopes,
                ),
                to_pulumi_object_field(
                    "shielded_instance_config",
                    &self.r#shielded_instance_config,
                ),
                to_pulumi_object_field(
                    "subnetwork",
                    &self.r#subnetwork,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "zone",
                    &self.r#zone,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterClusterConfigGceClusterConfig {
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
                    r#confidential_instance_config: {
                        let field_value = match fields_map.get("confidential_instance_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'confidential_instance_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#internal_ip_only: {
                        let field_value = match fields_map.get("internal_ip_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'internal_ip_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadata: {
                        let field_value = match fields_map.get("metadata") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_group_affinity: {
                        let field_value = match fields_map.get("node_group_affinity") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_group_affinity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reservation_affinity: {
                        let field_value = match fields_map.get("reservation_affinity") {
                            Some(value) => value,
                            None => bail!("Missing field 'reservation_affinity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account: {
                        let field_value = match fields_map.get("service_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_scopes: {
                        let field_value = match fields_map.get("service_account_scopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_scopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shielded_instance_config: {
                        let field_value = match fields_map.get("shielded_instance_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'shielded_instance_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnetwork: {
                        let field_value = match fields_map.get("subnetwork") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetwork' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
