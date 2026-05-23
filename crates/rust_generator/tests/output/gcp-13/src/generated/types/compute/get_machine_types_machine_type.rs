#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetMachineTypesMachineType {
    /// A list of accelerator configurations assigned to this machine type. Structure is documented below.
    #[builder(into)]
    pub r#accelerators: Vec<super::super::types::compute::GetMachineTypesMachineTypeAccelerator>,
    /// The configuration of bundled local SSD for the machine type. Structure is documented below.
    #[builder(into)]
    pub r#bundled_local_ssds: Vec<super::super::types::compute::GetMachineTypesMachineTypeBundledLocalSsd>,
    /// The deprecation status associated with this machine type. Structure is documented below.
    #[builder(into)]
    pub r#deprecateds: Vec<super::super::types::compute::GetMachineTypesMachineTypeDeprecated>,
    /// A textual description of the machine type.
    #[builder(into)]
    pub r#description: String,
    /// The number of virtual CPUs that are available to the instance.
    #[builder(into)]
    pub r#guest_cpus: i32,
    /// Whether this machine type has a shared CPU.
    #[builder(into)]
    pub r#is_shared_cpus: bool,
    /// The maximum persistent disks allowed.
    #[builder(into)]
    pub r#maximum_persistent_disks: i32,
    /// The maximum total persistent disks size (GB) allowed.
    #[builder(into)]
    pub r#maximum_persistent_disks_size_gb: i32,
    /// The amount of physical memory available to the instance, defined in MB.
    #[builder(into)]
    pub r#memory_mb: i32,
    /// The name of the machine type.
    #[builder(into)]
    pub r#name: String,
    /// The server-defined URL for the machine type.
    #[builder(into)]
    pub r#self_link: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetMachineTypesMachineType {
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
                    "accelerators",
                    &self.r#accelerators,
                ),
                to_pulumi_object_field(
                    "bundledLocalSsds",
                    &self.r#bundled_local_ssds,
                ),
                to_pulumi_object_field(
                    "deprecateds",
                    &self.r#deprecateds,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "guestCpus",
                    &self.r#guest_cpus,
                ),
                to_pulumi_object_field(
                    "isSharedCpus",
                    &self.r#is_shared_cpus,
                ),
                to_pulumi_object_field(
                    "maximumPersistentDisks",
                    &self.r#maximum_persistent_disks,
                ),
                to_pulumi_object_field(
                    "maximumPersistentDisksSizeGb",
                    &self.r#maximum_persistent_disks_size_gb,
                ),
                to_pulumi_object_field(
                    "memoryMb",
                    &self.r#memory_mb,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "selfLink",
                    &self.r#self_link,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("bundledLocalSsds") {
                            Some(value) => value,
                            None => bail!("Missing field 'bundledLocalSsds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("guestCpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'guestCpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_shared_cpus: {
                        let field_value = match fields_map.get("isSharedCpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'isSharedCpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_persistent_disks: {
                        let field_value = match fields_map.get("maximumPersistentDisks") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximumPersistentDisks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_persistent_disks_size_gb: {
                        let field_value = match fields_map.get("maximumPersistentDisksSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximumPersistentDisksSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_mb: {
                        let field_value = match fields_map.get("memoryMb") {
                            Some(value) => value,
                            None => bail!("Missing field 'memoryMb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("selfLink") {
                            Some(value) => value,
                            None => bail!("Missing field 'selfLink' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
