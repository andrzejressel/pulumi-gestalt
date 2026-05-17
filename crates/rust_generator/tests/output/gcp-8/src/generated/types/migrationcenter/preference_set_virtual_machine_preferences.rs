#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreferenceSetVirtualMachinePreferences {
    /// Commitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: `COMMITMENT_PLAN_UNSPECIFIED`, `COMMITMENT_PLAN_NONE`, `COMMITMENT_PLAN_ONE_YEAR`, `COMMITMENT_PLAN_THREE_YEARS`
    #[builder(into)]
    #[serde(rename = "commitmentPlan")]
    pub r#commitment_plan: Option<String>,
    /// The user preferences relating to Compute Engine target platform.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "computeEnginePreferences")]
    pub r#compute_engine_preferences: Option<Box<super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferencesComputeEnginePreferences>>,
    /// The user preferences relating to target regions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "regionPreferences")]
    pub r#region_preferences: Option<Box<super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferencesRegionPreferences>>,
    /// Sizing optimization strategy specifies the preferred strategy used when extrapolating usage data to calculate insights and recommendations for a virtual machine. If you are unsure which value to set, a moderate sizing optimization strategy is often a good value to start with. Possible values: `SIZING_OPTIMIZATION_STRATEGY_UNSPECIFIED`, `SIZING_OPTIMIZATION_STRATEGY_SAME_AS_SOURCE`, `SIZING_OPTIMIZATION_STRATEGY_MODERATE`, `SIZING_OPTIMIZATION_STRATEGY_AGGRESSIVE`
    #[builder(into)]
    #[serde(rename = "sizingOptimizationStrategy")]
    pub r#sizing_optimization_strategy: Option<String>,
    /// Preferences concerning Sole Tenancy nodes and VMs.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "soleTenancyPreferences")]
    pub r#sole_tenancy_preferences: Option<Box<super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferencesSoleTenancyPreferences>>,
    /// Target product for assets using this preference set. Specify either target product or business goal, but not both. Possible values: `COMPUTE_MIGRATION_TARGET_PRODUCT_UNSPECIFIED`, `COMPUTE_MIGRATION_TARGET_PRODUCT_COMPUTE_ENGINE`, `COMPUTE_MIGRATION_TARGET_PRODUCT_VMWARE_ENGINE`, `COMPUTE_MIGRATION_TARGET_PRODUCT_SOLE_TENANCY`
    #[builder(into)]
    #[serde(rename = "targetProduct")]
    pub r#target_product: Option<String>,
    /// The user preferences relating to Google Cloud VMware Engine target platform.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vmwareEnginePreferences")]
    pub r#vmware_engine_preferences: Option<Box<super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferencesVmwareEnginePreferences>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreferenceSetVirtualMachinePreferences {
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
                "commitment_plan".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#commitment_plan,
                )
                .await,
            );
            map.insert(
                "compute_engine_preferences".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compute_engine_preferences,
                )
                .await,
            );
            map.insert(
                "region_preferences".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#region_preferences,
                )
                .await,
            );
            map.insert(
                "sizing_optimization_strategy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sizing_optimization_strategy,
                )
                .await,
            );
            map.insert(
                "sole_tenancy_preferences".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sole_tenancy_preferences,
                )
                .await,
            );
            map.insert(
                "target_product".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_product,
                )
                .await,
            );
            map.insert(
                "vmware_engine_preferences".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vmware_engine_preferences,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreferenceSetVirtualMachinePreferences {
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
                    r#commitment_plan: {
                        let field_value = match fields_map.get("commitment_plan") {
                            Some(value) => value,
                            None => bail!("Missing field 'commitment_plan' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compute_engine_preferences: {
                        let field_value = match fields_map.get("compute_engine_preferences") {
                            Some(value) => value,
                            None => bail!("Missing field 'compute_engine_preferences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#region_preferences: {
                        let field_value = match fields_map.get("region_preferences") {
                            Some(value) => value,
                            None => bail!("Missing field 'region_preferences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sizing_optimization_strategy: {
                        let field_value = match fields_map.get("sizing_optimization_strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'sizing_optimization_strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sole_tenancy_preferences: {
                        let field_value = match fields_map.get("sole_tenancy_preferences") {
                            Some(value) => value,
                            None => bail!("Missing field 'sole_tenancy_preferences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_product: {
                        let field_value = match fields_map.get("target_product") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_product' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vmware_engine_preferences: {
                        let field_value = match fields_map.get("vmware_engine_preferences") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmware_engine_preferences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
