#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterConfigPreemptibleWorkerConfigDiskConfig {
    /// Size of the primary disk attached to each preemptible worker node, specified
    /// in GB. The smallest allowed disk size is 10GB. GCP will default to a predetermined
    /// computed value if not set (currently 500GB). Note: If SSDs are not
    /// attached, it also contains the HDFS data blocks and Hadoop working directories.
    #[builder(into)]
    #[serde(rename = "bootDiskSizeGb")]
    pub r#boot_disk_size_gb: Option<i32>,
    /// The disk type of the primary disk attached to each preemptible worker node.
    /// One of `"pd-ssd"` or `"pd-standard"`. Defaults to `"pd-standard"`.
    #[builder(into)]
    #[serde(rename = "bootDiskType")]
    pub r#boot_disk_type: Option<String>,
    /// Interface type of local SSDs (default is "scsi"). Valid values: "scsi" (Small Computer System Interface), "nvme" (Non-Volatile Memory Express).
    #[builder(into)]
    #[serde(rename = "localSsdInterface")]
    pub r#local_ssd_interface: Option<String>,
    /// The amount of local SSD disks that will be
    /// attached to each preemptible worker node. Defaults to 0.
    #[builder(into)]
    #[serde(rename = "numLocalSsds")]
    pub r#num_local_ssds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterClusterConfigPreemptibleWorkerConfigDiskConfig {
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
                "boot_disk_size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#boot_disk_size_gb,
                )
                .await,
            );
            map.insert(
                "boot_disk_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#boot_disk_type,
                )
                .await,
            );
            map.insert(
                "local_ssd_interface".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_ssd_interface,
                )
                .await,
            );
            map.insert(
                "num_local_ssds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#num_local_ssds,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterClusterConfigPreemptibleWorkerConfigDiskConfig {
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
                    r#boot_disk_size_gb: {
                        let field_value = match fields_map.get("boot_disk_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'boot_disk_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#boot_disk_type: {
                        let field_value = match fields_map.get("boot_disk_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'boot_disk_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_ssd_interface: {
                        let field_value = match fields_map.get("local_ssd_interface") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_ssd_interface' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#num_local_ssds: {
                        let field_value = match fields_map.get("num_local_ssds") {
                            Some(value) => value,
                            None => bail!("Missing field 'num_local_ssds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
