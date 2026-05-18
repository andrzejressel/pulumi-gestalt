#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkloadPartnerPermissions {
    /// Optional. Allow partner to view violation alerts.
    #[builder(into)]
    #[serde(rename = "assuredWorkloadsMonitoring")]
    pub r#assured_workloads_monitoring: Option<bool>,
    /// Allow the partner to view inspectability logs and monitoring violations.
    #[builder(into)]
    #[serde(rename = "dataLogsViewer")]
    pub r#data_logs_viewer: Option<bool>,
    /// Optional. Allow partner to view access approval logs.
    #[builder(into)]
    #[serde(rename = "serviceAccessApprover")]
    pub r#service_access_approver: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkloadPartnerPermissions {
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
                    "assured_workloads_monitoring",
                    &self.r#assured_workloads_monitoring,
                ),
                to_pulumi_object_field(
                    "data_logs_viewer",
                    &self.r#data_logs_viewer,
                ),
                to_pulumi_object_field(
                    "service_access_approver",
                    &self.r#service_access_approver,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkloadPartnerPermissions {
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
                    r#assured_workloads_monitoring: {
                        let field_value = match fields_map.get("assured_workloads_monitoring") {
                            Some(value) => value,
                            None => bail!("Missing field 'assured_workloads_monitoring' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_logs_viewer: {
                        let field_value = match fields_map.get("data_logs_viewer") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_logs_viewer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_access_approver: {
                        let field_value = match fields_map.get("service_access_approver") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_access_approver' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
