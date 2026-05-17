#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetResponsePlanIncidentTemplate {
    /// A string used to stop Incident Manager from creating multiple incident records for the same incident.
    #[builder(into)]
    #[serde(rename = "dedupeString")]
    pub r#dedupe_string: String,
    /// The impact value of a generated incident. The following values are supported:
    #[builder(into)]
    #[serde(rename = "impact")]
    pub r#impact: i32,
    /// The tags assigned to an incident template. When an incident starts, Incident Manager assigns the tags specified in the template to the incident.
    #[builder(into)]
    #[serde(rename = "incidentTags")]
    pub r#incident_tags: std::collections::HashMap<String, String>,
    /// The Amazon Simple Notification Service (Amazon SNS) targets that this incident notifies when it is updated. The `notification_target` configuration block supports the following argument:
    #[builder(into)]
    #[serde(rename = "notificationTargets")]
    pub r#notification_targets: Vec<super::super::types::ssmincidents::GetResponsePlanIncidentTemplateNotificationTarget>,
    /// The summary of an incident.
    #[builder(into)]
    #[serde(rename = "summary")]
    pub r#summary: String,
    /// The title of a generated incident.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetResponsePlanIncidentTemplate {
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
                "dedupe_string".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dedupe_string,
                )
                .await,
            );
            map.insert(
                "impact".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#impact,
                )
                .await,
            );
            map.insert(
                "incident_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#incident_tags,
                )
                .await,
            );
            map.insert(
                "notification_targets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#notification_targets,
                )
                .await,
            );
            map.insert(
                "summary".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#summary,
                )
                .await,
            );
            map.insert(
                "title".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#title,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetResponsePlanIncidentTemplate {
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
