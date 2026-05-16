#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResponsePlanActionSsmAutomation {
    /// The automation document's name.
    #[builder(into)]
    #[serde(rename = "documentName")]
    pub r#document_name: String,
    /// The version of the automation document to use at runtime.
    #[builder(into)]
    #[serde(rename = "documentVersion")]
    pub r#document_version: Option<String>,
    /// The key-value pair to resolve dynamic parameter values when processing a Systems Manager Automation runbook.
    #[builder(into)]
    #[serde(rename = "dynamicParameters")]
    pub r#dynamic_parameters: Option<std::collections::HashMap<String, String>>,
    /// The key-value pair parameters to use when the automation document runs. The following values are supported:
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::super::types::ssmincidents::ResponsePlanActionSsmAutomationParameter>>,
    /// The Amazon Resource Name (ARN) of the role that the automation document assumes when it runs commands.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The account that the automation document runs in. This can be in either the management account or an application account.
    #[builder(into)]
    #[serde(rename = "targetAccount")]
    pub r#target_account: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResponsePlanActionSsmAutomation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("document_name".to_string(), self.r#document_name.to_pulumi_value().await);
            map.insert("document_version".to_string(), self.r#document_version.to_pulumi_value().await);
            map.insert("dynamic_parameters".to_string(), self.r#dynamic_parameters.to_pulumi_value().await);
            map.insert("parameters".to_string(), self.r#parameters.to_pulumi_value().await);
            map.insert("role_arn".to_string(), self.r#role_arn.to_pulumi_value().await);
            map.insert("target_account".to_string(), self.r#target_account.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResponsePlanActionSsmAutomation {
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
                    r#document_name: {
                        let field_value = match fields_map.get("document_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'document_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#document_version: {
                        let field_value = match fields_map.get("document_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'document_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dynamic_parameters: {
                        let field_value = match fields_map.get("dynamic_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamic_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::ssmincidents::ResponsePlanActionSsmAutomationParameter>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#target_account: {
                        let field_value = match fields_map.get("target_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
