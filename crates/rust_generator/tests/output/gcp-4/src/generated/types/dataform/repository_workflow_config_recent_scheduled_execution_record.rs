#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryWorkflowConfigRecentScheduledExecutionRecord {
    /// (Output)
    /// The error status encountered upon this attempt to create the workflow invocation, if the attempt was unsuccessful.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "errorStatuses")]
    pub r#error_statuses: Option<Vec<super::super::types::dataform::RepositoryWorkflowConfigRecentScheduledExecutionRecordErrorStatus>>,
    /// (Output)
    /// The timestamp of this workflow attempt.
    #[builder(into)]
    #[serde(rename = "executionTime")]
    pub r#execution_time: Option<String>,
    /// (Output)
    /// The name of the created workflow invocation, if one was successfully created. In the format projects/*/locations/*/repositories/*/workflowInvocations/*.
    #[builder(into)]
    #[serde(rename = "workflowInvocation")]
    pub r#workflow_invocation: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RepositoryWorkflowConfigRecentScheduledExecutionRecord {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "error_statuses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#error_statuses,
                )
                .await,
            );
            map.insert(
                "execution_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#execution_time,
                )
                .await,
            );
            map.insert(
                "workflow_invocation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workflow_invocation,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RepositoryWorkflowConfigRecentScheduledExecutionRecord {
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
                    r#error_statuses: {
                        let field_value = match fields_map.get("error_statuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_statuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#execution_time: {
                        let field_value = match fields_map.get("execution_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'execution_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workflow_invocation: {
                        let field_value = match fields_map.get("workflow_invocation") {
                            Some(value) => value,
                            None => bail!("Missing field 'workflow_invocation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
