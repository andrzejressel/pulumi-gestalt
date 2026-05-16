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
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("automation_parameters".to_string(), self.r#automation_parameters.to_pulumi_value().await);
            map.insert("lambda_parameters".to_string(), self.r#lambda_parameters.to_pulumi_value().await);
            map.insert("run_command_parameters".to_string(), self.r#run_command_parameters.to_pulumi_value().await);
            map.insert("step_functions_parameters".to_string(), self.r#step_functions_parameters.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MaintenanceWindowTaskTaskInvocationParameters {
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
                    r#automation_parameters: {
                        let field_value = match fields_map.get("automation_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'automation_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersAutomationParameters>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#lambda_parameters: {
                        let field_value = match fields_map.get("lambda_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersLambdaParameters>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#run_command_parameters: {
                        let field_value = match fields_map.get("run_command_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'run_command_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersRunCommandParameters>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#step_functions_parameters: {
                        let field_value = match fields_map.get("step_functions_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'step_functions_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersStepFunctionsParameters>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
