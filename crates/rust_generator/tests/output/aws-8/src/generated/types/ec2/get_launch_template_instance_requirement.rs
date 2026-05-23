#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLaunchTemplateInstanceRequirement {
    #[builder(into)]
    pub r#accelerator_counts: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementAcceleratorCount>,
    #[builder(into)]
    pub r#accelerator_manufacturers: Vec<String>,
    #[builder(into)]
    pub r#accelerator_names: Vec<String>,
    #[builder(into)]
    pub r#accelerator_total_memory_mibs: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementAcceleratorTotalMemoryMib>,
    #[builder(into)]
    pub r#accelerator_types: Vec<String>,
    #[builder(into)]
    pub r#allowed_instance_types: Vec<String>,
    #[builder(into)]
    pub r#bare_metal: String,
    #[builder(into)]
    pub r#baseline_ebs_bandwidth_mbps: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementBaselineEbsBandwidthMbp>,
    #[builder(into)]
    pub r#burstable_performance: String,
    #[builder(into)]
    pub r#cpu_manufacturers: Vec<String>,
    #[builder(into)]
    pub r#excluded_instance_types: Vec<String>,
    #[builder(into)]
    pub r#instance_generations: Vec<String>,
    #[builder(into)]
    pub r#local_storage: String,
    #[builder(into)]
    pub r#local_storage_types: Vec<String>,
    #[builder(into)]
    pub r#max_spot_price_as_percentage_of_optimal_on_demand_price: i32,
    #[builder(into)]
    pub r#memory_gib_per_vcpus: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementMemoryGibPerVcpus>,
    #[builder(into)]
    pub r#memory_mibs: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementMemoryMib>,
    #[builder(into)]
    pub r#network_bandwidth_gbps: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementNetworkBandwidthGbp>,
    #[builder(into)]
    pub r#network_interface_counts: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementNetworkInterfaceCount>,
    #[builder(into)]
    pub r#on_demand_max_price_percentage_over_lowest_price: i32,
    #[builder(into)]
    pub r#require_hibernate_support: bool,
    #[builder(into)]
    pub r#spot_max_price_percentage_over_lowest_price: i32,
    #[builder(into)]
    pub r#total_local_storage_gbs: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementTotalLocalStorageGb>,
    #[builder(into)]
    pub r#vcpu_counts: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementVcpuCount>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLaunchTemplateInstanceRequirement {
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
                    "acceleratorCounts",
                    &self.r#accelerator_counts,
                ),
                to_pulumi_object_field(
                    "acceleratorManufacturers",
                    &self.r#accelerator_manufacturers,
                ),
                to_pulumi_object_field(
                    "acceleratorNames",
                    &self.r#accelerator_names,
                ),
                to_pulumi_object_field(
                    "acceleratorTotalMemoryMibs",
                    &self.r#accelerator_total_memory_mibs,
                ),
                to_pulumi_object_field(
                    "acceleratorTypes",
                    &self.r#accelerator_types,
                ),
                to_pulumi_object_field(
                    "allowedInstanceTypes",
                    &self.r#allowed_instance_types,
                ),
                to_pulumi_object_field(
                    "bareMetal",
                    &self.r#bare_metal,
                ),
                to_pulumi_object_field(
                    "baselineEbsBandwidthMbps",
                    &self.r#baseline_ebs_bandwidth_mbps,
                ),
                to_pulumi_object_field(
                    "burstablePerformance",
                    &self.r#burstable_performance,
                ),
                to_pulumi_object_field(
                    "cpuManufacturers",
                    &self.r#cpu_manufacturers,
                ),
                to_pulumi_object_field(
                    "excludedInstanceTypes",
                    &self.r#excluded_instance_types,
                ),
                to_pulumi_object_field(
                    "instanceGenerations",
                    &self.r#instance_generations,
                ),
                to_pulumi_object_field(
                    "localStorage",
                    &self.r#local_storage,
                ),
                to_pulumi_object_field(
                    "localStorageTypes",
                    &self.r#local_storage_types,
                ),
                to_pulumi_object_field(
                    "maxSpotPriceAsPercentageOfOptimalOnDemandPrice",
                    &self.r#max_spot_price_as_percentage_of_optimal_on_demand_price,
                ),
                to_pulumi_object_field(
                    "memoryGibPerVcpus",
                    &self.r#memory_gib_per_vcpus,
                ),
                to_pulumi_object_field(
                    "memoryMibs",
                    &self.r#memory_mibs,
                ),
                to_pulumi_object_field(
                    "networkBandwidthGbps",
                    &self.r#network_bandwidth_gbps,
                ),
                to_pulumi_object_field(
                    "networkInterfaceCounts",
                    &self.r#network_interface_counts,
                ),
                to_pulumi_object_field(
                    "onDemandMaxPricePercentageOverLowestPrice",
                    &self.r#on_demand_max_price_percentage_over_lowest_price,
                ),
                to_pulumi_object_field(
                    "requireHibernateSupport",
                    &self.r#require_hibernate_support,
                ),
                to_pulumi_object_field(
                    "spotMaxPricePercentageOverLowestPrice",
                    &self.r#spot_max_price_percentage_over_lowest_price,
                ),
                to_pulumi_object_field(
                    "totalLocalStorageGbs",
                    &self.r#total_local_storage_gbs,
                ),
                to_pulumi_object_field(
                    "vcpuCounts",
                    &self.r#vcpu_counts,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLaunchTemplateInstanceRequirement {
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
                    r#accelerator_counts: {
                        let field_value = match fields_map.get("acceleratorCounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'acceleratorCounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#accelerator_manufacturers: {
                        let field_value = match fields_map.get("acceleratorManufacturers") {
                            Some(value) => value,
                            None => bail!("Missing field 'acceleratorManufacturers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#accelerator_names: {
                        let field_value = match fields_map.get("acceleratorNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'acceleratorNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#accelerator_total_memory_mibs: {
                        let field_value = match fields_map.get("acceleratorTotalMemoryMibs") {
                            Some(value) => value,
                            None => bail!("Missing field 'acceleratorTotalMemoryMibs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#accelerator_types: {
                        let field_value = match fields_map.get("acceleratorTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'acceleratorTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_instance_types: {
                        let field_value = match fields_map.get("allowedInstanceTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedInstanceTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bare_metal: {
                        let field_value = match fields_map.get("bareMetal") {
                            Some(value) => value,
                            None => bail!("Missing field 'bareMetal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#baseline_ebs_bandwidth_mbps: {
                        let field_value = match fields_map.get("baselineEbsBandwidthMbps") {
                            Some(value) => value,
                            None => bail!("Missing field 'baselineEbsBandwidthMbps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#burstable_performance: {
                        let field_value = match fields_map.get("burstablePerformance") {
                            Some(value) => value,
                            None => bail!("Missing field 'burstablePerformance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_manufacturers: {
                        let field_value = match fields_map.get("cpuManufacturers") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpuManufacturers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_instance_types: {
                        let field_value = match fields_map.get("excludedInstanceTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'excludedInstanceTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_generations: {
                        let field_value = match fields_map.get("instanceGenerations") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceGenerations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_storage: {
                        let field_value = match fields_map.get("localStorage") {
                            Some(value) => value,
                            None => bail!("Missing field 'localStorage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_storage_types: {
                        let field_value = match fields_map.get("localStorageTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'localStorageTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_spot_price_as_percentage_of_optimal_on_demand_price: {
                        let field_value = match fields_map.get("maxSpotPriceAsPercentageOfOptimalOnDemandPrice") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxSpotPriceAsPercentageOfOptimalOnDemandPrice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_gib_per_vcpus: {
                        let field_value = match fields_map.get("memoryGibPerVcpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'memoryGibPerVcpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_mibs: {
                        let field_value = match fields_map.get("memoryMibs") {
                            Some(value) => value,
                            None => bail!("Missing field 'memoryMibs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_bandwidth_gbps: {
                        let field_value = match fields_map.get("networkBandwidthGbps") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkBandwidthGbps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_interface_counts: {
                        let field_value = match fields_map.get("networkInterfaceCounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkInterfaceCounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_demand_max_price_percentage_over_lowest_price: {
                        let field_value = match fields_map.get("onDemandMaxPricePercentageOverLowestPrice") {
                            Some(value) => value,
                            None => bail!("Missing field 'onDemandMaxPricePercentageOverLowestPrice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_hibernate_support: {
                        let field_value = match fields_map.get("requireHibernateSupport") {
                            Some(value) => value,
                            None => bail!("Missing field 'requireHibernateSupport' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot_max_price_percentage_over_lowest_price: {
                        let field_value = match fields_map.get("spotMaxPricePercentageOverLowestPrice") {
                            Some(value) => value,
                            None => bail!("Missing field 'spotMaxPricePercentageOverLowestPrice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_local_storage_gbs: {
                        let field_value = match fields_map.get("totalLocalStorageGbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'totalLocalStorageGbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vcpu_counts: {
                        let field_value = match fields_map.get("vcpuCounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'vcpuCounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
