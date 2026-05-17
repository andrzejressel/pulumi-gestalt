#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MaintenanceWindowTaskTaskInvocationParameters {
    /// The parameters for an AUTOMATION task type. Documented below.
    #[builder(into)]
    #[serde(rename = "automationParameters")]
    pub r#automation_parameters: Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersAutomationParameters>>,
    /// The parameters for a LAMBDA task type. Documented below.
    #[builder(into)]
    #[serde(rename = "lambdaParameters")]
    pub r#lambda_parameters: Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersLambdaParameters>>,
    /// The parameters for a RUN_COMMAND task type. Documented below.
    #[builder(into)]
    #[serde(rename = "runCommandParameters")]
    pub r#run_command_parameters: Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersRunCommandParameters>>,
    /// The parameters for a STEP_FUNCTIONS task type. Documented below.
    #[builder(into)]
    #[serde(rename = "stepFunctionsParameters")]
    pub r#step_functions_parameters: Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersStepFunctionsParameters>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MaintenanceWindowTaskTaskInvocationParameters {
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
                    "automation_parameters",
                    &self.r#automation_parameters,
                ),
                to_pulumi_object_field(
                    "lambda_parameters",
                    &self.r#lambda_parameters,
                ),
                to_pulumi_object_field(
                    "run_command_parameters",
                    &self.r#run_command_parameters,
                ),
                to_pulumi_object_field(
                    "step_functions_parameters",
                    &self.r#step_functions_parameters,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MaintenanceWindowTaskTaskInvocationParameters {
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
                    r#automation_parameters: {
                        let field_value = match fields_map.get("automation_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'automation_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lambda_parameters: {
                        let field_value = match fields_map.get("lambda_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#run_command_parameters: {
                        let field_value = match fields_map.get("run_command_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'run_command_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#step_functions_parameters: {
                        let field_value = match fields_map.get("step_functions_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'step_functions_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
