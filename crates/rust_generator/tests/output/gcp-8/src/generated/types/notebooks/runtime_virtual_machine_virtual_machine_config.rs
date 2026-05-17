#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuntimeVirtualMachineVirtualMachineConfig {
    /// The Compute Engine accelerator configuration for this runtime.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "acceleratorConfig")]
    pub r#accelerator_config: Option<Box<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfigAcceleratorConfig>>,
    /// Use a list of container images to start the notebook instance.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "containerImages")]
    pub r#container_images: Option<Vec<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfigContainerImage>>,
    /// Data disk option configuration settings.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dataDisk")]
    pub r#data_disk: Box<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfigDataDisk>,
    /// Encryption settings for virtual machine data disk.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Option<Box<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfigEncryptionConfig>>,
    /// (Output)
    /// The Compute Engine guest attributes. (see [Project and instance
    /// guest attributes](https://cloud.google.com/compute/docs/
    /// storing-retrieving-metadata#guest_attributes)).
    #[builder(into)]
    #[serde(rename = "guestAttributes")]
    pub r#guest_attributes: Option<std::collections::HashMap<String, String>>,
    /// If true, runtime will only have internal IP addresses. By default,
    /// runtimes are not restricted to internal IP addresses, and will
    /// have ephemeral external IP addresses assigned to each vm. This
    /// `internal_ip_only` restriction can only be enabled for subnetwork
    /// enabled networks, and all dependencies must be configured to be
    /// accessible without external IP addresses.
    #[builder(into)]
    #[serde(rename = "internalIpOnly")]
    pub r#internal_ip_only: Option<bool>,
    /// The labels to associate with this runtime. Label **keys** must
    /// contain 1 to 63 characters, and must conform to [RFC 1035]
    /// (https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be
    /// empty, but, if present, must contain 1 to 63 characters, and must
    /// conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No
    /// more than 32 labels can be associated with a cluster.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// The Compute Engine machine type used for runtimes.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: String,
    /// The Compute Engine metadata entries to add to virtual machine.
    /// (see [Project and instance metadata](https://cloud.google.com
    /// /compute/docs/storing-retrieving-metadata#project_and_instance
    /// _metadata)).
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Option<std::collections::HashMap<String, String>>,
    /// The Compute Engine network to be used for machine communications.
    /// Cannot be specified with subnetwork. If neither `network` nor
    /// `subnet` is specified, the "default" network of the project is
    /// used, if it exists. A full URL or partial URI. Examples:
    /// * `https://www.googleapis.com/compute/v1/projects/[project_id]/
    /// regions/global/default`
    /// * `projects/[project_id]/regions/global/default`
    /// Runtimes are managed resources inside Google Infrastructure.
    /// Runtimes support the following network configurations:
    /// * Google Managed Network (Network & subnet are empty)
    /// * Consumer Project VPC (network & subnet are required). Requires
    /// configuring Private Service Access.
    /// * Shared VPC (network & subnet are required). Requires
    /// configuring Private Service Access.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// The type of vNIC to be used on this interface. This may be gVNIC
    /// or VirtioNet.
    /// Possible values are: `UNSPECIFIED_NIC_TYPE`, `VIRTIO_NET`, `GVNIC`.
    #[builder(into)]
    #[serde(rename = "nicType")]
    pub r#nic_type: Option<String>,
    /// Reserved IP Range name is used for VPC Peering. The
    /// subnetwork allocation will use the range *name* if it's assigned.
    #[builder(into)]
    #[serde(rename = "reservedIpRange")]
    pub r#reserved_ip_range: Option<String>,
    /// Shielded VM Instance configuration settings.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "shieldedInstanceConfig")]
    pub r#shielded_instance_config: Option<Box<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfigShieldedInstanceConfig>>,
    /// The Compute Engine subnetwork to be used for machine
    /// communications. Cannot be specified with network. A full URL or
    /// partial URI are valid. Examples:
    /// * `https://www.googleapis.com/compute/v1/projects/[project_id]/
    /// regions/us-east1/subnetworks/sub0`
    /// * `projects/[project_id]/regions/us-east1/subnetworks/sub0`
    #[builder(into)]
    #[serde(rename = "subnet")]
    pub r#subnet: Option<String>,
    /// The Compute Engine tags to add to runtime (see [Tagging instances]
    /// (https://cloud.google.com/compute/docs/
    /// label-or-tag-resources#tags)).
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<String>>,
    /// (Output)
    /// The zone where the virtual machine is located.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuntimeVirtualMachineVirtualMachineConfig {
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
                "accelerator_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#accelerator_config,
                )
                .await,
            );
            map.insert(
                "container_images".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_images,
                )
                .await,
            );
            map.insert(
                "data_disk".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_disk,
                )
                .await,
            );
            map.insert(
                "encryption_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_config,
                )
                .await,
            );
            map.insert(
                "guest_attributes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#guest_attributes,
                )
                .await,
            );
            map.insert(
                "internal_ip_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#internal_ip_only,
                )
                .await,
            );
            map.insert(
                "labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#labels,
                )
                .await,
            );
            map.insert(
                "machine_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#machine_type,
                )
                .await,
            );
            map.insert(
                "metadata".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metadata,
                )
                .await,
            );
            map.insert(
                "network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network,
                )
                .await,
            );
            map.insert(
                "nic_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nic_type,
                )
                .await,
            );
            map.insert(
                "reserved_ip_range".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reserved_ip_range,
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
                "subnet".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnet,
                )
                .await,
            );
            map.insert(
                "tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tags,
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
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuntimeVirtualMachineVirtualMachineConfig {
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
                    r#accelerator_config: {
                        let field_value = match fields_map.get("accelerator_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerator_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_images: {
                        let field_value = match fields_map.get("container_images") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_images' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_disk: {
                        let field_value = match fields_map.get("data_disk") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_disk' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_config: {
                        let field_value = match fields_map.get("encryption_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#guest_attributes: {
                        let field_value = match fields_map.get("guest_attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'guest_attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_type: {
                        let field_value = match fields_map.get("machine_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#nic_type: {
                        let field_value = match fields_map.get("nic_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'nic_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reserved_ip_range: {
                        let field_value = match fields_map.get("reserved_ip_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'reserved_ip_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#subnet: {
                        let field_value = match fields_map.get("subnet") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
