#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkflowTemplatePlacementManagedClusterConfigGceClusterConfig {
    /// If true, all instances in the cluster will only have internal IP addresses. By default, clusters are not restricted to internal IP addresses, and will have ephemeral external IP addresses assigned to each instance. This `internal_ip_only` restriction can only be enabled for subnetwork enabled networks, and all off-cluster dependencies must be configured to be accessible without external IP addresses.
    #[builder(into)]
    #[serde(rename = "internalIpOnly")]
    pub r#internal_ip_only: Option<bool>,
    /// The Compute Engine metadata entries to add to all instances (see [About VM metadata](https://cloud.google.com/compute/docs/metadata/overview)).
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Option<std::collections::HashMap<String, String>>,
    /// The Compute Engine network to be used for machine communications. Cannot be specified with subnetwork_uri. If neither `network_uri` nor `subnetwork_uri` is specified, the "default" network of the project is used, if it exists. Cannot be a "Custom Subnet Network" (see /regions/global/default` * `default`
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// Node Group Affinity for sole-tenant clusters.
    #[builder(into)]
    #[serde(rename = "nodeGroupAffinity")]
    pub r#node_group_affinity: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigGceClusterConfigNodeGroupAffinity>>,
    /// The type of IPv6 access for a cluster. Possible values: PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED, INHERIT_FROM_SUBNETWORK, OUTBOUND, BIDIRECTIONAL
    #[builder(into)]
    #[serde(rename = "privateIpv6GoogleAccess")]
    pub r#private_ipv_6_google_access: Option<String>,
    /// Reservation Affinity for consuming Zonal reservation.
    #[builder(into)]
    #[serde(rename = "reservationAffinity")]
    pub r#reservation_affinity: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigGceClusterConfigReservationAffinity>>,
    /// The (https://cloud.google.com/compute/docs/access/service-accounts#default_service_account) is used.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Option<String>,
    /// The URIs of service account scopes to be included in Compute Engine instances. The following base set of scopes is always included: * https://www.googleapis.com/auth/cloud.useraccounts.readonly * https://www.googleapis.com/auth/devstorage.read_write * https://www.googleapis.com/auth/logging.write If no scopes are specified, the following defaults are also provided: * https://www.googleapis.com/auth/bigquery * https://www.googleapis.com/auth/bigtable.admin.table * https://www.googleapis.com/auth/bigtable.data * https://www.googleapis.com/auth/devstorage.full_control
    #[builder(into)]
    #[serde(rename = "serviceAccountScopes")]
    pub r#service_account_scopes: Option<Vec<String>>,
    /// Shielded Instance Config for clusters using [Compute Engine Shielded VMs](https://cloud.google.com/security/shielded-cloud/shielded-vm). Structure defined below.
    #[builder(into)]
    #[serde(rename = "shieldedInstanceConfig")]
    pub r#shielded_instance_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigGceClusterConfigShieldedInstanceConfig>>,
    /// The Compute Engine subnetwork to be used for machine communications. Cannot be specified with network_uri. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects//regions/us-east1/subnetworks/sub0` * `sub0`
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Option<String>,
    /// The Compute Engine tags to add to all instances (see [Manage tags for resources](https://cloud.google.com/compute/docs/tag-resources)).
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<String>>,
    /// The zone where the Compute Engine cluster will be located. On a create request, it is required in the "global" region. If omitted in a non-global Dataproc region, the service will pick a zone in the corresponding Compute Engine region. On a get request, zone will always be present. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/` * `us-central1-f`
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkflowTemplatePlacementManagedClusterConfigGceClusterConfig {
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
                    "private_ipv_6_google_access",
                    &self.r#private_ipv_6_google_access,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkflowTemplatePlacementManagedClusterConfigGceClusterConfig {
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
                    r#private_ipv_6_google_access: {
                        let field_value = match fields_map.get("private_ipv_6_google_access") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_ipv_6_google_access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
