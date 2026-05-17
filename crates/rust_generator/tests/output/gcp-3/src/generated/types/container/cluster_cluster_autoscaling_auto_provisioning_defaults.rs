#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterAutoscalingAutoProvisioningDefaults {
    /// The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool. This should be of the form projects/[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cryptoKeys/[KEY_NAME]. For more information about protecting resources with Cloud KMS Keys please see: https://cloud.google.com/compute/docs/disks/customer-managed-encryption
    #[builder(into)]
    #[serde(rename = "bootDiskKmsKey")]
    pub r#boot_disk_kms_key: Option<String>,
    /// Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB. Defaults to `100`
    #[builder(into)]
    #[serde(rename = "diskSize")]
    pub r#disk_size: Option<i32>,
    /// Type of the disk attached to each node (e.g. 'pd-standard', 'pd-ssd', 'pd-balanced', or 'hyperdisk-balanced'). Defaults to `hyperdisk-balanced` if `hyperdisk-balanced` is supported and `pd-balanced` is not supported for the machine type; otherwise defaults to `pd-balanced`.
    #[builder(into)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Option<String>,
    /// The default image type used by NAP once a new node pool is being created. Please note that according to the [official documentation](https://cloud.google.com/kubernetes-engine/docs/how-to/node-auto-provisioning#default-image-type) the value must be one of the [COS_CONTAINERD, COS, UBUNTU_CONTAINERD, UBUNTU]. __NOTE__ : COS AND UBUNTU are deprecated as of `GKE 1.24`
    #[builder(into)]
    #[serde(rename = "imageType")]
    pub r#image_type: Option<String>,
    /// NodeManagement configuration for this NodePool. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "management")]
    pub r#management: Option<Box<super::super::types::container::ClusterClusterAutoscalingAutoProvisioningDefaultsManagement>>,
    /// Minimum CPU platform to be used for NAP created node pools. The instance may be scheduled on the
    /// specified or newer CPU platform. Applicable values are the friendly names of CPU platforms, such
    /// as "Intel Haswell" or "Intel Sandy Bridge".
    #[builder(into)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Option<String>,
    /// Scopes that are used by NAP and GKE Autopilot when creating node pools. Use the "https://www.googleapis.com/auth/cloud-platform" scope to grant access to all APIs. It is recommended that you set `service_account` to a non-default service account and grant IAM roles to that service account for only the resources that it needs.
    /// 
    /// > `monitoring.write` is always enabled regardless of user input.  `monitoring` and `logging.write` may also be enabled depending on the values for `monitoring_service` and `logging_service`.
    #[builder(into)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Option<Vec<String>>,
    /// The Google Cloud Platform Service Account to be used by the node VMs created by GKE Autopilot or NAP.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Option<String>,
    /// Shielded Instance options. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "shieldedInstanceConfig")]
    pub r#shielded_instance_config: Option<Box<super::super::types::container::ClusterClusterAutoscalingAutoProvisioningDefaultsShieldedInstanceConfig>>,
    /// Specifies the upgrade settings for NAP created node pools
    #[builder(into)]
    #[serde(rename = "upgradeSettings")]
    pub r#upgrade_settings: Option<Box<super::super::types::container::ClusterClusterAutoscalingAutoProvisioningDefaultsUpgradeSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterClusterAutoscalingAutoProvisioningDefaults {
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
                "boot_disk_kms_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#boot_disk_kms_key,
                )
                .await,
            );
            map.insert(
                "disk_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disk_size,
                )
                .await,
            );
            map.insert(
                "disk_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disk_type,
                )
                .await,
            );
            map.insert(
                "image_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_type,
                )
                .await,
            );
            map.insert(
                "management".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#management,
                )
                .await,
            );
            map.insert(
                "min_cpu_platform".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_cpu_platform,
                )
                .await,
            );
            map.insert(
                "oauth_scopes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oauth_scopes,
                )
                .await,
            );
            map.insert(
                "service_account".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_account,
                )
                .await,
            );
            map.insert(
                "shielded_instance_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#shielded_instance_config,
                )
                .await,
            );
            map.insert(
                "upgrade_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#upgrade_settings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterClusterAutoscalingAutoProvisioningDefaults {
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
                    r#boot_disk_kms_key: {
                        let field_value = match fields_map.get("boot_disk_kms_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'boot_disk_kms_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_size: {
                        let field_value = match fields_map.get("disk_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_type: {
                        let field_value = match fields_map.get("disk_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_type: {
                        let field_value = match fields_map.get("image_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#management: {
                        let field_value = match fields_map.get("management") {
                            Some(value) => value,
                            None => bail!("Missing field 'management' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_cpu_platform: {
                        let field_value = match fields_map.get("min_cpu_platform") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_cpu_platform' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_scopes: {
                        let field_value = match fields_map.get("oauth_scopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_scopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#shielded_instance_config: {
                        let field_value = match fields_map.get("shielded_instance_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'shielded_instance_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upgrade_settings: {
                        let field_value = match fields_map.get("upgrade_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'upgrade_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
