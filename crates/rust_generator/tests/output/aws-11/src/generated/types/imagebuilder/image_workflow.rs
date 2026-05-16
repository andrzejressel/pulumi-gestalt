#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ImageWorkflow {
    /// The action to take if the workflow fails. Must be one of `CONTINUE` or `ABORT`.
    #[builder(into)]
    #[serde(rename = "onFailure")]
    pub r#on_failure: Option<String>,
    /// The parallel group in which to run a test Workflow.
    #[builder(into)]
    #[serde(rename = "parallelGroup")]
    pub r#parallel_group: Option<String>,
    /// Configuration block for the workflow parameters. Detailed below.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::super::types::imagebuilder::ImageWorkflowParameter>>,
    /// Amazon Resource Name (ARN) of the Image Builder Workflow.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "workflowArn")]
    pub r#workflow_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ImageWorkflow {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("on_failure".to_string(), self.r#on_failure.to_pulumi_value().await);
            map.insert("parallel_group".to_string(), self.r#parallel_group.to_pulumi_value().await);
            map.insert("parameters".to_string(), self.r#parameters.to_pulumi_value().await);
            map.insert("workflow_arn".to_string(), self.r#workflow_arn.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ImageWorkflow {
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
                    r#on_failure: {
                        let field_value = match fields_map.get("on_failure") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_failure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#parallel_group: {
                        let field_value = match fields_map.get("parallel_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'parallel_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::imagebuilder::ImageWorkflowParameter>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#workflow_arn: {
                        let field_value = match fields_map.get("workflow_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'workflow_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
