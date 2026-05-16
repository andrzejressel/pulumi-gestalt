#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionRetryStrategyEvaluateOnExit {
    /// Specifies the action to take if all of the specified conditions (onStatusReason, onReason, and onExitCode) are met. The values aren't case sensitive.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// Contains a glob pattern to match against the decimal representation of the ExitCode returned for a job.
    #[builder(into)]
    #[serde(rename = "onExitCode")]
    pub r#on_exit_code: String,
    /// Contains a glob pattern to match against the Reason returned for a job.
    #[builder(into)]
    #[serde(rename = "onReason")]
    pub r#on_reason: String,
    /// Contains a glob pattern to match against the StatusReason returned for a job.
    #[builder(into)]
    #[serde(rename = "onStatusReason")]
    pub r#on_status_reason: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobDefinitionRetryStrategyEvaluateOnExit {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("action".to_string(), self.r#action.to_pulumi_value().await);
            map.insert("on_exit_code".to_string(), self.r#on_exit_code.to_pulumi_value().await);
            map.insert("on_reason".to_string(), self.r#on_reason.to_pulumi_value().await);
            map.insert("on_status_reason".to_string(), self.r#on_status_reason.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobDefinitionRetryStrategyEvaluateOnExit {
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
                    r#action: {
                        let field_value = match fields_map.get("action") {
                            Some(value) => value,
                            None => bail!("Missing field 'action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#on_exit_code: {
                        let field_value = match fields_map.get("on_exit_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_exit_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#on_reason: {
                        let field_value = match fields_map.get("on_reason") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_reason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#on_status_reason: {
                        let field_value = match fields_map.get("on_status_reason") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_status_reason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
