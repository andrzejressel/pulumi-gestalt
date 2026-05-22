#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterWorkerProfile {
    /// The resource ID of an associated disk encryption set. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "diskEncryptionSetId")]
    pub r#disk_encryption_set_id: Option<String>,
    /// The internal OS disk size of the worker Virtual Machines in GB. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: i32,
    /// Whether worker virtual machines are encrypted at host. Defaults to `false`. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** `encryption_at_host_enabled` is only available for certain VM sizes and the `EncryptionAtHost` feature must be enabled for your subscription. Please see the [Azure documentation](https://learn.microsoft.com/azure/virtual-machines/disks-enable-host-based-encryption-portal?tabs=azure-powershell) for more information.
    #[builder(into)]
    #[serde(rename = "encryptionAtHostEnabled")]
    pub r#encryption_at_host_enabled: Option<bool>,
    /// The initial number of worker nodes which should exist in the cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: i32,
    /// The ID of the subnet where worker nodes will be hosted. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
    /// The size of the Virtual Machines for the worker nodes. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterWorkerProfile {
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
                    "diskEncryptionSetId",
                    &self.r#disk_encryption_set_id,
                ),
                to_pulumi_object_field(
                    "diskSizeGb",
                    &self.r#disk_size_gb,
                ),
                to_pulumi_object_field(
                    "encryptionAtHostEnabled",
                    &self.r#encryption_at_host_enabled,
                ),
                to_pulumi_object_field(
                    "nodeCount",
                    &self.r#node_count,
                ),
                to_pulumi_object_field(
                    "subnetId",
                    &self.r#subnet_id,
                ),
                to_pulumi_object_field(
                    "vmSize",
                    &self.r#vm_size,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterWorkerProfile {
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
                    r#disk_encryption_set_id: {
                        let field_value = match fields_map.get("diskEncryptionSetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskEncryptionSetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_size_gb: {
                        let field_value = match fields_map.get("diskSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_at_host_enabled: {
                        let field_value = match fields_map.get("encryptionAtHostEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionAtHostEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_count: {
                        let field_value = match fields_map.get("nodeCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_size: {
                        let field_value = match fields_map.get("vmSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
