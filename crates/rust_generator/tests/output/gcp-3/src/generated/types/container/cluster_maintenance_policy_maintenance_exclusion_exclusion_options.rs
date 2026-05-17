#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterMaintenancePolicyMaintenanceExclusionExclusionOptions {
    /// The scope of automatic upgrades to restrict in the exclusion window. One of: **NO_UPGRADES | NO_MINOR_UPGRADES | NO_MINOR_OR_NODE_UPGRADES**
    /// 
    /// Specify `start_time` and `end_time` in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) "Zulu" date format.  The start time's date is
    /// the initial date that the window starts, and the end time is used for calculating duration.Specify `recurrence` in
    /// [RFC5545](https://tools.ietf.org/html/rfc5545#section-3.8.5.3) RRULE format, to specify when this recurs.
    /// Note that GKE may accept other formats, but will return values in UTC, causing a permanent diff.
    /// 
    /// Examples:
    /// 
    /// ```sh
    /// maintenance_policy {
    /// recurring_window {
    /// start_time = "2019-01-01T00:00:00Z"
    /// end_time = "2019-01-02T00:00:00Z"
    /// recurrence = "FREQ=DAILY"
    /// }
    /// maintenance_exclusion{
    /// exclusion_name = "batch job"
    /// start_time = "2019-01-01T00:00:00Z"
    /// end_time = "2019-01-02T00:00:00Z"
    /// exclusion_options {
    /// scope = "NO_UPGRADES"
    /// }
    /// }
    /// maintenance_exclusion{
    /// exclusion_name = "holiday data load"
    /// start_time = "2019-05-01T00:00:00Z"
    /// end_time = "2019-05-02T00:00:00Z"
    /// exclusion_options {
    /// scope = "NO_MINOR_UPGRADES"
    /// }
    /// }
    /// }
    /// ```
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterMaintenancePolicyMaintenanceExclusionExclusionOptions {
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
                "scope".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scope,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterMaintenancePolicyMaintenanceExclusionExclusionOptions {
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
                    r#scope: {
                        let field_value = match fields_map.get("scope") {
                            Some(value) => value,
                            None => bail!("Missing field 'scope' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
