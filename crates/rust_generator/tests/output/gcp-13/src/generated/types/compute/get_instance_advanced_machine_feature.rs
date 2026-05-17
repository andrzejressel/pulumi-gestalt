#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInstanceAdvancedMachineFeature {
    /// Whether to enable nested virtualization or not.
    #[builder(into)]
    #[serde(rename = "enableNestedVirtualization")]
    pub r#enable_nested_virtualization: bool,
    /// Whether to enable UEFI networking for the instance.
    #[builder(into)]
    #[serde(rename = "enableUefiNetworking")]
    pub r#enable_uefi_networking: bool,
    /// The PMU is a hardware component within the CPU core that monitors how the processor runs code. Valid values for the level of PMU are "STANDARD", "ENHANCED", and "ARCHITECTURAL".
    #[builder(into)]
    #[serde(rename = "performanceMonitoringUnit")]
    pub r#performance_monitoring_unit: String,
    /// The number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed.
    #[builder(into)]
    #[serde(rename = "threadsPerCore")]
    pub r#threads_per_core: i32,
    /// Turbo frequency mode to use for the instance. Currently supported modes is "ALL_CORE_MAX".
    #[builder(into)]
    #[serde(rename = "turboMode")]
    pub r#turbo_mode: String,
    /// The number of physical cores to expose to an instance. Multiply by the number of threads per core to compute the total number of virtual CPUs to expose to the instance. If unset, the number of cores is inferred from the instance\'s nominal CPU count and the underlying platform\'s SMT width.
    #[builder(into)]
    #[serde(rename = "visibleCoreCount")]
    pub r#visible_core_count: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetInstanceAdvancedMachineFeature {
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
                "enable_nested_virtualization".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_nested_virtualization,
                )
                .await,
            );
            map.insert(
                "enable_uefi_networking".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_uefi_networking,
                )
                .await,
            );
            map.insert(
                "performance_monitoring_unit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#performance_monitoring_unit,
                )
                .await,
            );
            map.insert(
                "threads_per_core".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#threads_per_core,
                )
                .await,
            );
            map.insert(
                "turbo_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#turbo_mode,
                )
                .await,
            );
            map.insert(
                "visible_core_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#visible_core_count,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetInstanceAdvancedMachineFeature {
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
                    r#enable_nested_virtualization: {
                        let field_value = match fields_map.get("enable_nested_virtualization") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_nested_virtualization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_uefi_networking: {
                        let field_value = match fields_map.get("enable_uefi_networking") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_uefi_networking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#performance_monitoring_unit: {
                        let field_value = match fields_map.get("performance_monitoring_unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'performance_monitoring_unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threads_per_core: {
                        let field_value = match fields_map.get("threads_per_core") {
                            Some(value) => value,
                            None => bail!("Missing field 'threads_per_core' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#turbo_mode: {
                        let field_value = match fields_map.get("turbo_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'turbo_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#visible_core_count: {
                        let field_value = match fields_map.get("visible_core_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'visible_core_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
