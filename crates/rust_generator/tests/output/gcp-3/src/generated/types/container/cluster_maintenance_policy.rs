#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterMaintenancePolicy {
    /// Time window specified for daily maintenance operations.
    /// Specify `start_time` in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) format "HH:MM”,
    /// where HH : \[00-23\] and MM : \[00-59\] GMT. For example:
    /// 
    /// Examples:
    #[builder(into)]
    #[serde(rename = "dailyMaintenanceWindow")]
    pub r#daily_maintenance_window: Option<Box<super::super::types::container::ClusterMaintenancePolicyDailyMaintenanceWindow>>,
    /// Exceptions to maintenance window. Non-emergency maintenance should not occur in these windows. A cluster can have up to 20 maintenance exclusions at a time [Maintenance Window and Exclusions](https://cloud.google.com/kubernetes-engine/docs/concepts/maintenance-windows-and-exclusions)
    #[builder(into)]
    #[serde(rename = "maintenanceExclusions")]
    pub r#maintenance_exclusions: Option<Vec<super::super::types::container::ClusterMaintenancePolicyMaintenanceExclusion>>,
    /// Time window for recurring maintenance operations.
    /// 
    /// Specify `start_time` and `end_time` in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) "Zulu" date format.  The start time's date is
    /// the initial date that the window starts, and the end time is used for calculating duration.  Specify `recurrence` in
    /// [RFC5545](https://tools.ietf.org/html/rfc5545#section-3.8.5.3) RRULE format, to specify when this recurs.
    /// Note that GKE may accept other formats, but will return values in UTC, causing a permanent diff.
    /// 
    /// Examples:
    /// ```sh
    /// maintenance_policy {
    /// recurring_window {
    /// start_time = "2019-08-01T02:00:00Z"
    /// end_time = "2019-08-01T06:00:00Z"
    /// recurrence = "FREQ=DAILY"
    /// }
    /// }
    /// ```
    /// 
    /// ```sh
    /// maintenance_policy {
    /// recurring_window {
    /// start_time = "2019-01-01T09:00:00Z"
    /// end_time = "2019-01-01T17:00:00Z"
    /// recurrence = "FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR"
    /// }
    /// }
    /// ```
    #[builder(into)]
    #[serde(rename = "recurringWindow")]
    pub r#recurring_window: Option<Box<super::super::types::container::ClusterMaintenancePolicyRecurringWindow>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterMaintenancePolicy {
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
                "daily_maintenance_window".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#daily_maintenance_window,
                )
                .await,
            );
            map.insert(
                "maintenance_exclusions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maintenance_exclusions,
                )
                .await,
            );
            map.insert(
                "recurring_window".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#recurring_window,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterMaintenancePolicy {
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
                    r#daily_maintenance_window: {
                        let field_value = match fields_map.get("daily_maintenance_window") {
                            Some(value) => value,
                            None => bail!("Missing field 'daily_maintenance_window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#recurring_window: {
                        let field_value = match fields_map.get("recurring_window") {
                            Some(value) => value,
                            None => bail!("Missing field 'recurring_window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
