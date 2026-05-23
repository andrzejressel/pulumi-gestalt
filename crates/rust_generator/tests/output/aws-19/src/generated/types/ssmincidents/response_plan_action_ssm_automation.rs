#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResponsePlanActionSsmAutomation {
    /// The automation document's name.
    #[builder(into)]
    pub r#document_name: String,
    /// The version of the automation document to use at runtime.
    #[builder(into)]
    pub r#document_version: Option<String>,
    /// The key-value pair to resolve dynamic parameter values when processing a Systems Manager Automation runbook.
    #[builder(into)]
    pub r#dynamic_parameters: Option<std::collections::HashMap<String, String>>,
    /// The key-value pair parameters to use when the automation document runs. The following values are supported:
    #[builder(into)]
    pub r#parameters: Option<Vec<super::super::types::ssmincidents::ResponsePlanActionSsmAutomationParameter>>,
    /// The Amazon Resource Name (ARN) of the role that the automation document assumes when it runs commands.
    #[builder(into)]
    pub r#role_arn: String,
    /// The account that the automation document runs in. This can be in either the management account or an application account.
    #[builder(into)]
    pub r#target_account: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResponsePlanActionSsmAutomation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "documentName",
                    &self.r#document_name,
                ),
                to_pulumi_object_field(
                    "documentVersion",
                    &self.r#document_version,
                ),
                to_pulumi_object_field(
                    "dynamicParameters",
                    &self.r#dynamic_parameters,
                ),
                to_pulumi_object_field(
                    "parameters",
                    &self.r#parameters,
                ),
                to_pulumi_object_field(
                    "roleArn",
                    &self.r#role_arn,
                ),
                to_pulumi_object_field(
                    "targetAccount",
                    &self.r#target_account,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResponsePlanActionSsmAutomation {
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
                    r#document_name: {
                        let field_value = match fields_map.get("documentName") {
                            Some(value) => value,
                            None => bail!("Missing field 'documentName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#document_version: {
                        let field_value = match fields_map.get("documentVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'documentVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamic_parameters: {
                        let field_value = match fields_map.get("dynamicParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamicParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("roleArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'roleArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_account: {
                        let field_value = match fields_map.get("targetAccount") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetAccount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
