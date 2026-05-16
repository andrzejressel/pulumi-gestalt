#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDomainAutoTuneOption {
    /// Auto-Tune desired state for the domain.
    #[builder(into)]
    #[serde(rename = "desiredState")]
    pub r#desired_state: String,
    /// A list of the nested configurations for the Auto-Tune maintenance windows of the domain.
    #[builder(into)]
    #[serde(rename = "maintenanceSchedules")]
    pub r#maintenance_schedules: Vec<super::super::types::opensearch::GetDomainAutoTuneOptionMaintenanceSchedule>,
    /// Whether the domain is set to roll back to default Auto-Tune settings when disabling Auto-Tune.
    #[builder(into)]
    #[serde(rename = "rollbackOnDisable")]
    pub r#rollback_on_disable: String,
    /// Whether to schedule Auto-Tune optimizations that require blue/green deployments during the domain's configured daily off-peak window.
    #[builder(into)]
    #[serde(rename = "useOffPeakWindow")]
    pub r#use_off_peak_window: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDomainAutoTuneOption {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("desired_state".to_string(), self.r#desired_state.to_pulumi_value().await);
            map.insert("maintenance_schedules".to_string(), self.r#maintenance_schedules.to_pulumi_value().await);
            map.insert("rollback_on_disable".to_string(), self.r#rollback_on_disable.to_pulumi_value().await);
            map.insert("use_off_peak_window".to_string(), self.r#use_off_peak_window.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDomainAutoTuneOption {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#desired_state: {
                        let field_value = match fields_map.get("desired_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'desired_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#maintenance_schedules: {
                        let field_value = match fields_map.get("maintenance_schedules") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenance_schedules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::opensearch::GetDomainAutoTuneOptionMaintenanceSchedule> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rollback_on_disable: {
                        let field_value = match fields_map.get("rollback_on_disable") {
                            Some(value) => value,
                            None => bail!("Missing field 'rollback_on_disable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#use_off_peak_window: {
                        let field_value = match fields_map.get("use_off_peak_window") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_off_peak_window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
