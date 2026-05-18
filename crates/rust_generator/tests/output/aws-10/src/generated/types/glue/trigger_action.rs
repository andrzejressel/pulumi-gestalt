#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TriggerAction {
    /// Arguments to be passed to the job. You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.
    #[builder(into)]
    #[serde(rename = "arguments")]
    pub r#arguments: Option<std::collections::HashMap<String, String>>,
    /// The name of the crawler to be executed. Conflicts with `job_name`.
    #[builder(into)]
    #[serde(rename = "crawlerName")]
    pub r#crawler_name: Option<String>,
    /// The name of a job to be executed. Conflicts with `crawler_name`.
    #[builder(into)]
    #[serde(rename = "jobName")]
    pub r#job_name: Option<String>,
    /// Specifies configuration properties of a job run notification. See Notification Property details below.
    #[builder(into)]
    #[serde(rename = "notificationProperty")]
    pub r#notification_property: Option<Box<super::super::types::glue::TriggerActionNotificationProperty>>,
    /// The name of the Security Configuration structure to be used with this action.
    #[builder(into)]
    #[serde(rename = "securityConfiguration")]
    pub r#security_configuration: Option<String>,
    /// The job run timeout in minutes. It overrides the timeout value of the job.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TriggerAction {
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
                    "arguments",
                    &self.r#arguments,
                ),
                to_pulumi_object_field(
                    "crawler_name",
                    &self.r#crawler_name,
                ),
                to_pulumi_object_field(
                    "job_name",
                    &self.r#job_name,
                ),
                to_pulumi_object_field(
                    "notification_property",
                    &self.r#notification_property,
                ),
                to_pulumi_object_field(
                    "security_configuration",
                    &self.r#security_configuration,
                ),
                to_pulumi_object_field(
                    "timeout",
                    &self.r#timeout,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TriggerAction {
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
                    r#arguments: {
                        let field_value = match fields_map.get("arguments") {
                            Some(value) => value,
                            None => bail!("Missing field 'arguments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#crawler_name: {
                        let field_value = match fields_map.get("crawler_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'crawler_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#job_name: {
                        let field_value = match fields_map.get("job_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_property: {
                        let field_value = match fields_map.get("notification_property") {
                            Some(value) => value,
                            None => bail!("Missing field 'notification_property' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_configuration: {
                        let field_value = match fields_map.get("security_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout: {
                        let field_value = match fields_map.get("timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
