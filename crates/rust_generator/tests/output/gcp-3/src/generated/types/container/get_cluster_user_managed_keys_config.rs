#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterUserManagedKeysConfig {
    /// The Certificate Authority Service caPool to use for the aggreation CA in this cluster.
    #[builder(into)]
    #[serde(rename = "aggregationCa")]
    pub r#aggregation_ca: String,
    /// The Certificate Authority Service caPool to use for the cluster CA in this cluster.
    #[builder(into)]
    #[serde(rename = "clusterCa")]
    pub r#cluster_ca: String,
    /// The Cloud KMS cryptoKey to use for Confidential Hyperdisk on the control plane nodes.
    #[builder(into)]
    #[serde(rename = "controlPlaneDiskEncryptionKey")]
    pub r#control_plane_disk_encryption_key: String,
    /// The Certificate Authority Service caPool to use for the etcd API CA in this cluster.
    #[builder(into)]
    #[serde(rename = "etcdApiCa")]
    pub r#etcd_api_ca: String,
    /// The Certificate Authority Service caPool to use for the etcd peer CA in this cluster.
    #[builder(into)]
    #[serde(rename = "etcdPeerCa")]
    pub r#etcd_peer_ca: String,
    /// Resource path of the Cloud KMS cryptoKey to use for encryption of internal etcd backups.
    #[builder(into)]
    #[serde(rename = "gkeopsEtcdBackupEncryptionKey")]
    pub r#gkeops_etcd_backup_encryption_key: String,
    /// The Cloud KMS cryptoKeyVersions to use for signing service account JWTs issued by this cluster.
    #[builder(into)]
    #[serde(rename = "serviceAccountSigningKeys")]
    pub r#service_account_signing_keys: Vec<String>,
    /// The Cloud KMS cryptoKeyVersions to use for verifying service account JWTs issued by this cluster.
    #[builder(into)]
    #[serde(rename = "serviceAccountVerificationKeys")]
    pub r#service_account_verification_keys: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterUserManagedKeysConfig {
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
                    "aggregationCa",
                    &self.r#aggregation_ca,
                ),
                to_pulumi_object_field(
                    "clusterCa",
                    &self.r#cluster_ca,
                ),
                to_pulumi_object_field(
                    "controlPlaneDiskEncryptionKey",
                    &self.r#control_plane_disk_encryption_key,
                ),
                to_pulumi_object_field(
                    "etcdApiCa",
                    &self.r#etcd_api_ca,
                ),
                to_pulumi_object_field(
                    "etcdPeerCa",
                    &self.r#etcd_peer_ca,
                ),
                to_pulumi_object_field(
                    "gkeopsEtcdBackupEncryptionKey",
                    &self.r#gkeops_etcd_backup_encryption_key,
                ),
                to_pulumi_object_field(
                    "serviceAccountSigningKeys",
                    &self.r#service_account_signing_keys,
                ),
                to_pulumi_object_field(
                    "serviceAccountVerificationKeys",
                    &self.r#service_account_verification_keys,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterUserManagedKeysConfig {
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
                        let field_value = match fields_map.get("aggregationCa") {
                            Some(value) => value,
                            None => bail!("Missing field 'aggregationCa' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_ca: {
                        let field_value = match fields_map.get("clusterCa") {
                            Some(value) => value,
                            None => bail!("Missing field 'clusterCa' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#control_plane_disk_encryption_key: {
                        let field_value = match fields_map.get("controlPlaneDiskEncryptionKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'controlPlaneDiskEncryptionKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#etcd_api_ca: {
                        let field_value = match fields_map.get("etcdApiCa") {
                            Some(value) => value,
                            None => bail!("Missing field 'etcdApiCa' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#etcd_peer_ca: {
                        let field_value = match fields_map.get("etcdPeerCa") {
                            Some(value) => value,
                            None => bail!("Missing field 'etcdPeerCa' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gkeops_etcd_backup_encryption_key: {
                        let field_value = match fields_map.get("gkeopsEtcdBackupEncryptionKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'gkeopsEtcdBackupEncryptionKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_signing_keys: {
                        let field_value = match fields_map.get("serviceAccountSigningKeys") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccountSigningKeys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_verification_keys: {
                        let field_value = match fields_map.get("serviceAccountVerificationKeys") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccountVerificationKeys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
