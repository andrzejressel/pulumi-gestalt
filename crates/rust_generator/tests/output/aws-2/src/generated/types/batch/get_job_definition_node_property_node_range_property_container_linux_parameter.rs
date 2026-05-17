#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerLinuxParameter {
    /// Any of the host devices to expose to the container.
    #[builder(into)]
    #[serde(rename = "devices")]
    pub r#devices: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerLinuxParameterDevice>,
    /// If true, run an init process inside the container that forwards signals and reaps processes.
    #[builder(into)]
    #[serde(rename = "initProcessEnabled")]
    pub r#init_process_enabled: bool,
    /// The total amount of swap memory (in MiB) a container can use.
    #[builder(into)]
    #[serde(rename = "maxSwap")]
    pub r#max_swap: i32,
    /// The value for the size (in MiB) of the `/dev/shm` volume.
    #[builder(into)]
    #[serde(rename = "sharedMemorySize")]
    pub r#shared_memory_size: i32,
    /// You can use this parameter to tune a container's memory swappiness behavior.
    #[builder(into)]
    #[serde(rename = "swappiness")]
    pub r#swappiness: i32,
    /// The container path, mount options, and size (in MiB) of the tmpfs mount.
    #[builder(into)]
    #[serde(rename = "tmpfs")]
    pub r#tmpfs: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerLinuxParameterTmpf>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainerLinuxParameter {
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
                "devices".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#devices,
                )
                .await,
            );
            map.insert(
                "init_process_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#init_process_enabled,
                )
                .await,
            );
            map.insert(
                "max_swap".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_swap,
                )
                .await,
            );
            map.insert(
                "shared_memory_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#shared_memory_size,
                )
                .await,
            );
            map.insert(
                "swappiness".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#swappiness,
                )
                .await,
            );
            map.insert(
                "tmpfs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tmpfs,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainerLinuxParameter {
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
                    r#devices: {
                        let field_value = match fields_map.get("devices") {
                            Some(value) => value,
                            None => bail!("Missing field 'devices' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#init_process_enabled: {
                        let field_value = match fields_map.get("init_process_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'init_process_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_swap: {
                        let field_value = match fields_map.get("max_swap") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_swap' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shared_memory_size: {
                        let field_value = match fields_map.get("shared_memory_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'shared_memory_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#swappiness: {
                        let field_value = match fields_map.get("swappiness") {
                            Some(value) => value,
                            None => bail!("Missing field 'swappiness' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tmpfs: {
                        let field_value = match fields_map.get("tmpfs") {
                            Some(value) => value,
                            None => bail!("Missing field 'tmpfs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
