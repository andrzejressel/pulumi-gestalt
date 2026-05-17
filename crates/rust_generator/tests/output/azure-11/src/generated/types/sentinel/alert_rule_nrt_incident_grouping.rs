#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertRuleNrtIncidentGrouping {
    /// A list of alert details to group by, only when the `entity_matching_method` is `Selected`. Possible values are `DisplayName` and `Severity`.
    #[builder(into)]
    #[serde(rename = "byAlertDetails")]
    pub r#by_alert_details: Option<Vec<String>>,
    /// A list of custom details keys to group by, only when the `entity_matching_method` is `Selected`. Only keys defined in the `custom_details` may be used.
    #[builder(into)]
    #[serde(rename = "byCustomDetails")]
    pub r#by_custom_details: Option<Vec<String>>,
    /// A list of entity types to group by, only when the `entity_matching_method` is `Selected`. Possible values are `Account`, `AzureResource`, `CloudApplication`, `DNS`, `File`, `FileHash`, `Host`, `IP`, `Mailbox`, `MailCluster`, `MailMessage`, `Malware`, `Process`, `RegistryKey`, `RegistryValue`, `SecurityGroup`, `SubmissionMail`, `URL`.
    #[builder(into)]
    #[serde(rename = "byEntities")]
    pub r#by_entities: Option<Vec<String>>,
    /// Enable grouping incidents created from alerts triggered by this Sentinel NRT Alert Rule. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The method used to group incidents. Possible values are `AnyAlert`, `Selected` and `AllEntities`. Defaults to `AnyAlert`.
    #[builder(into)]
    #[serde(rename = "entityMatchingMethod")]
    pub r#entity_matching_method: Option<String>,
    /// Limit the group to alerts created within the lookback duration (in ISO 8601 duration format). Defaults to `PT5M`.
    #[builder(into)]
    #[serde(rename = "lookbackDuration")]
    pub r#lookback_duration: Option<String>,
    /// Whether to re-open closed matching incidents? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "reopenClosedIncidents")]
    pub r#reopen_closed_incidents: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AlertRuleNrtIncidentGrouping {
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
                    "by_alert_details",
                    &self.r#by_alert_details,
                ),
                to_pulumi_object_field(
                    "by_custom_details",
                    &self.r#by_custom_details,
                ),
                to_pulumi_object_field(
                    "by_entities",
                    &self.r#by_entities,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "entity_matching_method",
                    &self.r#entity_matching_method,
                ),
                to_pulumi_object_field(
                    "lookback_duration",
                    &self.r#lookback_duration,
                ),
                to_pulumi_object_field(
                    "reopen_closed_incidents",
                    &self.r#reopen_closed_incidents,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AlertRuleNrtIncidentGrouping {
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
                    r#by_alert_details: {
                        let field_value = match fields_map.get("by_alert_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'by_alert_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#by_custom_details: {
                        let field_value = match fields_map.get("by_custom_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'by_custom_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#by_entities: {
                        let field_value = match fields_map.get("by_entities") {
                            Some(value) => value,
                            None => bail!("Missing field 'by_entities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#entity_matching_method: {
                        let field_value = match fields_map.get("entity_matching_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'entity_matching_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lookback_duration: {
                        let field_value = match fields_map.get("lookback_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'lookback_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reopen_closed_incidents: {
                        let field_value = match fields_map.get("reopen_closed_incidents") {
                            Some(value) => value,
                            None => bail!("Missing field 'reopen_closed_incidents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
