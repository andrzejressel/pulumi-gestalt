#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRegionInstanceTemplateAdvancedMachineFeature {
    /// Whether to enable nested virtualization or not.
    #[builder(into)]
    pub r#enable_nested_virtualization: bool,
    /// Whether to enable UEFI networking or not.
    #[builder(into)]
    pub r#enable_uefi_networking: bool,
    /// The PMU is a hardware component within the CPU core that monitors how the processor runs code. Valid values for the level of PMU are "STANDARD", "ENHANCED", and "ARCHITECTURAL".
    #[builder(into)]
    pub r#performance_monitoring_unit: String,
    /// The number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed.
    #[builder(into)]
    pub r#threads_per_core: i32,
    /// Turbo frequency mode to use for the instance. Currently supported modes is "ALL_CORE_MAX".
    #[builder(into)]
    pub r#turbo_mode: String,
    /// The number of physical cores to expose to an instance. Multiply by the number of threads per core to compute the total number of virtual CPUs to expose to the instance. If unset, the number of cores is inferred from the instance\'s nominal CPU count and the underlying platform\'s SMT width.
    #[builder(into)]
    pub r#visible_core_count: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRegionInstanceTemplateAdvancedMachineFeature {
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
                    "enableNestedVirtualization",
                    &self.r#enable_nested_virtualization,
                ),
                to_pulumi_object_field(
                    "enableUefiNetworking",
                    &self.r#enable_uefi_networking,
                ),
                to_pulumi_object_field(
                    "performanceMonitoringUnit",
                    &self.r#performance_monitoring_unit,
                ),
                to_pulumi_object_field(
                    "threadsPerCore",
                    &self.r#threads_per_core,
                ),
                to_pulumi_object_field(
                    "turboMode",
                    &self.r#turbo_mode,
                ),
                to_pulumi_object_field(
                    "visibleCoreCount",
                    &self.r#visible_core_count,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRegionInstanceTemplateAdvancedMachineFeature {
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
                        let field_value = match fields_map.get("enableNestedVirtualization") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableNestedVirtualization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_uefi_networking: {
                        let field_value = match fields_map.get("enableUefiNetworking") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableUefiNetworking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#performance_monitoring_unit: {
                        let field_value = match fields_map.get("performanceMonitoringUnit") {
                            Some(value) => value,
                            None => bail!("Missing field 'performanceMonitoringUnit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threads_per_core: {
                        let field_value = match fields_map.get("threadsPerCore") {
                            Some(value) => value,
                            None => bail!("Missing field 'threadsPerCore' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#turbo_mode: {
                        let field_value = match fields_map.get("turboMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'turboMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#visible_core_count: {
                        let field_value = match fields_map.get("visibleCoreCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'visibleCoreCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
