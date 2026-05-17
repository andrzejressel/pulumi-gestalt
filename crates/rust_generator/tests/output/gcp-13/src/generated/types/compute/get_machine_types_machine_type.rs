#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetMachineTypesMachineType {
    /// A list of accelerator configurations assigned to this machine type. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "accelerators")]
    pub r#accelerators: Vec<super::super::types::compute::GetMachineTypesMachineTypeAccelerator>,
    /// The configuration of bundled local SSD for the machine type. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bundledLocalSsds")]
    pub r#bundled_local_ssds: Vec<super::super::types::compute::GetMachineTypesMachineTypeBundledLocalSsd>,
    /// The deprecation status associated with this machine type. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "deprecateds")]
    pub r#deprecateds: Vec<super::super::types::compute::GetMachineTypesMachineTypeDeprecated>,
    /// A textual description of the machine type.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The number of virtual CPUs that are available to the instance.
    #[builder(into)]
    #[serde(rename = "guestCpus")]
    pub r#guest_cpus: i32,
    /// Whether this machine type has a shared CPU.
    #[builder(into)]
    #[serde(rename = "isSharedCpus")]
    pub r#is_shared_cpus: bool,
    /// The maximum persistent disks allowed.
    #[builder(into)]
    #[serde(rename = "maximumPersistentDisks")]
    pub r#maximum_persistent_disks: i32,
    /// The maximum total persistent disks size (GB) allowed.
    #[builder(into)]
    #[serde(rename = "maximumPersistentDisksSizeGb")]
    pub r#maximum_persistent_disks_size_gb: i32,
    /// The amount of physical memory available to the instance, defined in MB.
    #[builder(into)]
    #[serde(rename = "memoryMb")]
    pub r#memory_mb: i32,
    /// The name of the machine type.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The server-defined URL for the machine type.
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetMachineTypesMachineType {
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
                "accelerators".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#accelerators,
                )
                .await,
            );
            map.insert(
                "bundled_local_ssds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bundled_local_ssds,
                )
                .await,
            );
            map.insert(
                "deprecateds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#deprecateds,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "guest_cpus".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#guest_cpus,
                )
                .await,
            );
            map.insert(
                "is_shared_cpus".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_shared_cpus,
                )
                .await,
            );
            map.insert(
                "maximum_persistent_disks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_persistent_disks,
                )
                .await,
            );
            map.insert(
                "maximum_persistent_disks_size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_persistent_disks_size_gb,
                )
                .await,
            );
            map.insert(
                "memory_mb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#memory_mb,
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
                "self_link".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#self_link,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetMachineTypesMachineType {
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
                    r#accelerators: {
                        let field_value = match fields_map.get("accelerators") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerators' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bundled_local_ssds: {
                        let field_value = match fields_map.get("bundled_local_ssds") {
                            Some(value) => value,
                            None => bail!("Missing field 'bundled_local_ssds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deprecateds: {
                        let field_value = match fields_map.get("deprecateds") {
                            Some(value) => value,
                            None => bail!("Missing field 'deprecateds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#guest_cpus: {
                        let field_value = match fields_map.get("guest_cpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'guest_cpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_shared_cpus: {
                        let field_value = match fields_map.get("is_shared_cpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_shared_cpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_persistent_disks: {
                        let field_value = match fields_map.get("maximum_persistent_disks") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_persistent_disks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_persistent_disks_size_gb: {
                        let field_value = match fields_map.get("maximum_persistent_disks_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_persistent_disks_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_mb: {
                        let field_value = match fields_map.get("memory_mb") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_mb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#self_link: {
                        let field_value = match fields_map.get("self_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'self_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
