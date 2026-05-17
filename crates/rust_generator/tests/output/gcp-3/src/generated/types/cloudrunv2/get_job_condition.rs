#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobCondition {
    /// A reason for the execution condition.
    #[builder(into)]
    #[serde(rename = "executionReason")]
    pub r#execution_reason: String,
    /// Last time the condition transitioned from one status to another.
    /// 
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "lastTransitionTime")]
    pub r#last_transition_time: String,
    /// Human readable message indicating details about the current status.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: String,
    /// A common (service-level) reason for this condition.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: String,
    /// A reason for the revision condition.
    #[builder(into)]
    #[serde(rename = "revisionReason")]
    pub r#revision_reason: String,
    /// How to interpret failures of this condition, one of Error, Warning, Info
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: String,
    /// State of the condition.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
    /// type is used to communicate the status of the reconciliation process. See also: https://github.com/knative/serving/blob/main/docs/spec/errors.md#error-conditions-and-reporting Types common to all resources include: * "Ready": True when the Resource is ready.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobCondition {
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
                "execution_reason".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#execution_reason,
                )
                .await,
            );
            map.insert(
                "last_transition_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_transition_time,
                )
                .await,
            );
            map.insert(
                "message".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#message,
                )
                .await,
            );
            map.insert(
                "reason".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reason,
                )
                .await,
            );
            map.insert(
                "revision_reason".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#revision_reason,
                )
                .await,
            );
            map.insert(
                "severity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#severity,
                )
                .await,
            );
            map.insert(
                "state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobCondition {
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
                    r#execution_reason: {
                        let field_value = match fields_map.get("execution_reason") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_reason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_transition_time: {
                        let field_value = match fields_map.get("last_transition_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_transition_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#message: {
                        let field_value = match fields_map.get("message") {
                            Some(value) => value,
                            None => bail!("Missing field 'message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reason: {
                        let field_value = match fields_map.get("reason") {
                            Some(value) => value,
                            None => bail!("Missing field 'reason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#revision_reason: {
                        let field_value = match fields_map.get("revision_reason") {
                            Some(value) => value,
                            None => bail!("Missing field 'revision_reason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#severity: {
                        let field_value = match fields_map.get("severity") {
                            Some(value) => value,
                            None => bail!("Missing field 'severity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
