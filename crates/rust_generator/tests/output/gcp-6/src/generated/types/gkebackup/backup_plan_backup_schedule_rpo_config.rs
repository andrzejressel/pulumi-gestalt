#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupPlanBackupScheduleRpoConfig {
    /// User specified time windows during which backup can NOT happen for this BackupPlan.
    /// Backups should start and finish outside of any given exclusion window. Note: backup
    /// jobs will be scheduled to start and finish outside the duration of the window as
    /// much as possible, but running jobs will not get canceled when it runs into the window.
    /// All the time and date values in exclusionWindows entry in the API are in UTC. We
    /// only allow <=1 recurrence (daily or weekly) exclusion window for a BackupPlan while no
    /// restriction on number of single occurrence windows.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "exclusionWindows")]
    pub r#exclusion_windows: Option<Vec<super::super::types::gkebackup::BackupPlanBackupScheduleRpoConfigExclusionWindow>>,
    /// Defines the target RPO for the BackupPlan in minutes, which means the target
    /// maximum data loss in time that is acceptable for this BackupPlan. This must be
    /// at least 60, i.e., 1 hour, and at most 86400, i.e., 60 days.
    #[builder(into)]
    #[serde(rename = "targetRpoMinutes")]
    pub r#target_rpo_minutes: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackupPlanBackupScheduleRpoConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "exclusion_windows",
                    &self.r#exclusion_windows,
                ),
                to_pulumi_object_field(
                    "target_rpo_minutes",
                    &self.r#target_rpo_minutes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackupPlanBackupScheduleRpoConfig {
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
                    r#exclusion_windows: {
                        let field_value = match fields_map.get("exclusion_windows") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusion_windows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_rpo_minutes: {
                        let field_value = match fields_map.get("target_rpo_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_rpo_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
