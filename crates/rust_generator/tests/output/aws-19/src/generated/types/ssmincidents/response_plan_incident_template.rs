#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResponsePlanIncidentTemplate {
    /// A string used to stop Incident Manager from creating multiple incident records for the same incident.
    #[builder(into)]
    #[serde(rename = "dedupeString")]
    pub r#dedupe_string: Option<String>,
    /// The impact value of a generated incident. The following values are supported:
    #[builder(into)]
    #[serde(rename = "impact")]
    pub r#impact: i32,
    /// The tags assigned to an incident template. When an incident starts, Incident Manager assigns the tags specified in the template to the incident.
    #[builder(into)]
    #[serde(rename = "incidentTags")]
    pub r#incident_tags: Option<std::collections::HashMap<String, String>>,
    /// The Amazon Simple Notification Service (Amazon SNS) targets that this incident notifies when it is updated. The `notification_target` configuration block supports the following argument:
    #[builder(into)]
    #[serde(rename = "notificationTargets")]
    pub r#notification_targets: Option<Vec<super::super::types::ssmincidents::ResponsePlanIncidentTemplateNotificationTarget>>,
    /// The summary of an incident.
    #[builder(into)]
    #[serde(rename = "summary")]
    pub r#summary: Option<String>,
    /// The title of a generated incident.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResponsePlanIncidentTemplate {
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
                    "dedupe_string",
                    &self.r#dedupe_string,
                ),
                to_pulumi_object_field(
                    "impact",
                    &self.r#impact,
                ),
                to_pulumi_object_field(
                    "incident_tags",
                    &self.r#incident_tags,
                ),
                to_pulumi_object_field(
                    "notification_targets",
                    &self.r#notification_targets,
                ),
                to_pulumi_object_field(
                    "summary",
                    &self.r#summary,
                ),
                to_pulumi_object_field(
                    "title",
                    &self.r#title,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResponsePlanIncidentTemplate {
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
                    r#dedupe_string: {
                        let field_value = match fields_map.get("dedupe_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'dedupe_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#impact: {
                        let field_value = match fields_map.get("impact") {
                            Some(value) => value,
                            None => bail!("Missing field 'impact' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#incident_tags: {
                        let field_value = match fields_map.get("incident_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'incident_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_targets: {
                        let field_value = match fields_map.get("notification_targets") {
                            Some(value) => value,
                            None => bail!("Missing field 'notification_targets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#summary: {
                        let field_value = match fields_map.get("summary") {
                            Some(value) => value,
                            None => bail!("Missing field 'summary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#title: {
                        let field_value = match fields_map.get("title") {
                            Some(value) => value,
                            None => bail!("Missing field 'title' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
