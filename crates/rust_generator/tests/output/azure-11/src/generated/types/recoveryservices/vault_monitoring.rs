#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VaultMonitoring {
    /// Enabling/Disabling built-in Azure Monitor alerts for security scenarios and job failure scenarios. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "alertsForAllJobFailuresEnabled")]
    pub r#alerts_for_all_job_failures_enabled: Option<bool>,
    /// Enabling/Disabling alerts from the older (classic alerts) solution. Defaults to `true`. More details could be found [here](https://learn.microsoft.com/en-us/azure/backup/monitoring-and-alerts-overview).
    #[builder(into)]
    #[serde(rename = "alertsForCriticalOperationFailuresEnabled")]
    pub r#alerts_for_critical_operation_failures_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VaultMonitoring {
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
                    "alerts_for_all_job_failures_enabled",
                    &self.r#alerts_for_all_job_failures_enabled,
                ),
                to_pulumi_object_field(
                    "alerts_for_critical_operation_failures_enabled",
                    &self.r#alerts_for_critical_operation_failures_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VaultMonitoring {
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
                    r#alerts_for_all_job_failures_enabled: {
                        let field_value = match fields_map.get("alerts_for_all_job_failures_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'alerts_for_all_job_failures_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#alerts_for_critical_operation_failures_enabled: {
                        let field_value = match fields_map.get("alerts_for_critical_operation_failures_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'alerts_for_critical_operation_failures_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
