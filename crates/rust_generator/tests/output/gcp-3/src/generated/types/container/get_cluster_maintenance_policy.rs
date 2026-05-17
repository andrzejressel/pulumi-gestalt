#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterMaintenancePolicy {
    /// Time window specified for daily maintenance operations. Specify start_time in RFC3339 format "HH:MM”, where HH : [00-23] and MM : [00-59] GMT.
    #[builder(into)]
    #[serde(rename = "dailyMaintenanceWindows")]
    pub r#daily_maintenance_windows: Vec<super::super::types::container::GetClusterMaintenancePolicyDailyMaintenanceWindow>,
    /// Exceptions to maintenance window. Non-emergency maintenance should not occur in these windows.
    #[builder(into)]
    #[serde(rename = "maintenanceExclusions")]
    pub r#maintenance_exclusions: Vec<super::super::types::container::GetClusterMaintenancePolicyMaintenanceExclusion>,
    /// Time window for recurring maintenance operations.
    #[builder(into)]
    #[serde(rename = "recurringWindows")]
    pub r#recurring_windows: Vec<super::super::types::container::GetClusterMaintenancePolicyRecurringWindow>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterMaintenancePolicy {
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
                    "daily_maintenance_windows",
                    &self.r#daily_maintenance_windows,
                ),
                to_pulumi_object_field(
                    "maintenance_exclusions",
                    &self.r#maintenance_exclusions,
                ),
                to_pulumi_object_field(
                    "recurring_windows",
                    &self.r#recurring_windows,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterMaintenancePolicy {
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
                    r#daily_maintenance_windows: {
                        let field_value = match fields_map.get("daily_maintenance_windows") {
                            Some(value) => value,
                            None => bail!("Missing field 'daily_maintenance_windows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_exclusions: {
                        let field_value = match fields_map.get("maintenance_exclusions") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenance_exclusions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recurring_windows: {
                        let field_value = match fields_map.get("recurring_windows") {
                            Some(value) => value,
                            None => bail!("Missing field 'recurring_windows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
