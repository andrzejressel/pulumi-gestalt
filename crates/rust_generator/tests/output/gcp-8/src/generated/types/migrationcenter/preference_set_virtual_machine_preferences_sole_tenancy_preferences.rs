#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreferenceSetVirtualMachinePreferencesSoleTenancyPreferences {
    /// Commitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: `COMMITMENT_PLAN_UNSPECIFIED`, `ON_DEMAND`, `COMMITMENT_1_YEAR`, `COMMITMENT_3_YEAR`
    #[builder(into)]
    #[serde(rename = "commitmentPlan")]
    pub r#commitment_plan: Option<String>,
    /// CPU overcommit ratio. Acceptable values are between 1.0 and 2.0 inclusive.
    #[builder(into)]
    #[serde(rename = "cpuOvercommitRatio")]
    pub r#cpu_overcommit_ratio: Option<f64>,
    /// Sole Tenancy nodes maintenance policy. Possible values: `HOST_MAINTENANCE_POLICY_UNSPECIFIED`, `HOST_MAINTENANCE_POLICY_DEFAULT`, `HOST_MAINTENANCE_POLICY_RESTART_IN_PLACE`, `HOST_MAINTENANCE_POLICY_MIGRATE_WITHIN_NODE_GROUP`
    #[builder(into)]
    #[serde(rename = "hostMaintenancePolicy")]
    pub r#host_maintenance_policy: Option<String>,
    /// A list of sole tenant node types. An empty list means that all possible node types will be considered.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "nodeTypes")]
    pub r#node_types: Option<Vec<super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferencesSoleTenancyPreferencesNodeType>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreferenceSetVirtualMachinePreferencesSoleTenancyPreferences {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "commitment_plan",
                    &self.r#commitment_plan,
                ),
                to_pulumi_object_field(
                    "cpu_overcommit_ratio",
                    &self.r#cpu_overcommit_ratio,
                ),
                to_pulumi_object_field(
                    "host_maintenance_policy",
                    &self.r#host_maintenance_policy,
                ),
                to_pulumi_object_field(
                    "node_types",
                    &self.r#node_types,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreferenceSetVirtualMachinePreferencesSoleTenancyPreferences {
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
                    r#cpu_overcommit_ratio: {
                        let field_value = match fields_map.get("cpu_overcommit_ratio") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_overcommit_ratio' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_maintenance_policy: {
                        let field_value = match fields_map.get("host_maintenance_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_maintenance_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_types: {
                        let field_value = match fields_map.get("node_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
