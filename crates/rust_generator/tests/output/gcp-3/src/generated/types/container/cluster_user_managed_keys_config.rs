#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterUserManagedKeysConfig {
    /// The Certificate Authority Service caPool to use for the aggreation CA in this cluster.
    #[builder(into)]
    #[serde(rename = "aggregationCa")]
    pub r#aggregation_ca: Option<String>,
    /// The Certificate Authority Service caPool to use for the cluster CA in this cluster.
    #[builder(into)]
    #[serde(rename = "clusterCa")]
    pub r#cluster_ca: Option<String>,
    /// The Cloud KMS cryptoKey to use for Confidential Hyperdisk on the control plane nodes.
    #[builder(into)]
    #[serde(rename = "controlPlaneDiskEncryptionKey")]
    pub r#control_plane_disk_encryption_key: Option<String>,
    /// The Certificate Authority Service caPool to use for the etcd API CA in this cluster.
    #[builder(into)]
    #[serde(rename = "etcdApiCa")]
    pub r#etcd_api_ca: Option<String>,
    /// The Certificate Authority Service caPool to use for the etcd peer CA in this cluster.
    #[builder(into)]
    #[serde(rename = "etcdPeerCa")]
    pub r#etcd_peer_ca: Option<String>,
    /// Resource path of the Cloud KMS cryptoKey to use for encryption of internal etcd backups.
    #[builder(into)]
    #[serde(rename = "gkeopsEtcdBackupEncryptionKey")]
    pub r#gkeops_etcd_backup_encryption_key: Option<String>,
    /// The Cloud KMS cryptoKeyVersions to use for signing service account JWTs issued by this cluster.
    #[builder(into)]
    #[serde(rename = "serviceAccountSigningKeys")]
    pub r#service_account_signing_keys: Option<Vec<String>>,
    /// The Cloud KMS cryptoKeyVersions to use for verifying service account JWTs issued by this cluster.
    #[builder(into)]
    #[serde(rename = "serviceAccountVerificationKeys")]
    pub r#service_account_verification_keys: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterUserManagedKeysConfig {
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
                    "aggregation_ca",
                    &self.r#aggregation_ca,
                ),
                to_pulumi_object_field(
                    "cluster_ca",
                    &self.r#cluster_ca,
                ),
                to_pulumi_object_field(
                    "control_plane_disk_encryption_key",
                    &self.r#control_plane_disk_encryption_key,
                ),
                to_pulumi_object_field(
                    "etcd_api_ca",
                    &self.r#etcd_api_ca,
                ),
                to_pulumi_object_field(
                    "etcd_peer_ca",
                    &self.r#etcd_peer_ca,
                ),
                to_pulumi_object_field(
                    "gkeops_etcd_backup_encryption_key",
                    &self.r#gkeops_etcd_backup_encryption_key,
                ),
                to_pulumi_object_field(
                    "service_account_signing_keys",
                    &self.r#service_account_signing_keys,
                ),
                to_pulumi_object_field(
                    "service_account_verification_keys",
                    &self.r#service_account_verification_keys,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterUserManagedKeysConfig {
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
                    r#aggregation_ca: {
                        let field_value = match fields_map.get("aggregation_ca") {
                            Some(value) => value,
                            None => bail!("Missing field 'aggregation_ca' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_ca: {
                        let field_value = match fields_map.get("cluster_ca") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_ca' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#control_plane_disk_encryption_key: {
                        let field_value = match fields_map.get("control_plane_disk_encryption_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'control_plane_disk_encryption_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#etcd_api_ca: {
                        let field_value = match fields_map.get("etcd_api_ca") {
                            Some(value) => value,
                            None => bail!("Missing field 'etcd_api_ca' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#etcd_peer_ca: {
                        let field_value = match fields_map.get("etcd_peer_ca") {
                            Some(value) => value,
                            None => bail!("Missing field 'etcd_peer_ca' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gkeops_etcd_backup_encryption_key: {
                        let field_value = match fields_map.get("gkeops_etcd_backup_encryption_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'gkeops_etcd_backup_encryption_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_signing_keys: {
                        let field_value = match fields_map.get("service_account_signing_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_signing_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_verification_keys: {
                        let field_value = match fields_map.get("service_account_verification_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_verification_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
