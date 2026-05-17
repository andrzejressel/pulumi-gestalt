#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StandardAppVersionAutomaticScaling {
    /// Number of concurrent requests an automatic scaling instance can accept before the scheduler spawns a new instance.
    /// Defaults to a runtime-specific value.
    #[builder(into)]
    #[serde(rename = "maxConcurrentRequests")]
    pub r#max_concurrent_requests: Option<i32>,
    /// Maximum number of idle instances that should be maintained for this version.
    #[builder(into)]
    #[serde(rename = "maxIdleInstances")]
    pub r#max_idle_instances: Option<i32>,
    /// Maximum amount of time that a request should wait in the pending queue before starting a new instance to handle it.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "maxPendingLatency")]
    pub r#max_pending_latency: Option<String>,
    /// Minimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service.
    #[builder(into)]
    #[serde(rename = "minIdleInstances")]
    pub r#min_idle_instances: Option<i32>,
    /// Minimum amount of time a request should wait in the pending queue before starting a new instance to handle it.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "minPendingLatency")]
    pub r#min_pending_latency: Option<String>,
    /// Scheduler settings for standard environment.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "standardSchedulerSettings")]
    pub r#standard_scheduler_settings: Option<Box<super::super::types::appengine::StandardAppVersionAutomaticScalingStandardSchedulerSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StandardAppVersionAutomaticScaling {
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
                    "max_concurrent_requests",
                    &self.r#max_concurrent_requests,
                ),
                to_pulumi_object_field(
                    "max_idle_instances",
                    &self.r#max_idle_instances,
                ),
                to_pulumi_object_field(
                    "max_pending_latency",
                    &self.r#max_pending_latency,
                ),
                to_pulumi_object_field(
                    "min_idle_instances",
                    &self.r#min_idle_instances,
                ),
                to_pulumi_object_field(
                    "min_pending_latency",
                    &self.r#min_pending_latency,
                ),
                to_pulumi_object_field(
                    "standard_scheduler_settings",
                    &self.r#standard_scheduler_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StandardAppVersionAutomaticScaling {
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
                    r#max_concurrent_requests: {
                        let field_value = match fields_map.get("max_concurrent_requests") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_concurrent_requests' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_idle_instances: {
                        let field_value = match fields_map.get("max_idle_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_idle_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_pending_latency: {
                        let field_value = match fields_map.get("max_pending_latency") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_pending_latency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_idle_instances: {
                        let field_value = match fields_map.get("min_idle_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_idle_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_pending_latency: {
                        let field_value = match fields_map.get("min_pending_latency") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_pending_latency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#standard_scheduler_settings: {
                        let field_value = match fields_map.get("standard_scheduler_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'standard_scheduler_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
