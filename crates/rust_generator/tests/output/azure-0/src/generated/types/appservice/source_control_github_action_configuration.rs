#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SourceControlGithubActionConfiguration {
    /// A `code_configuration` block as defined above. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "codeConfiguration")]
    pub r#code_configuration: Option<Box<super::super::types::appservice::SourceControlGithubActionConfigurationCodeConfiguration>>,
    /// A `container_configuration` block as defined above.
    #[builder(into)]
    #[serde(rename = "containerConfiguration")]
    pub r#container_configuration: Option<Box<super::super::types::appservice::SourceControlGithubActionConfigurationContainerConfiguration>>,
    /// Whether to generate the GitHub work flow file. Defaults to `true`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "generateWorkflowFile")]
    pub r#generate_workflow_file: Option<bool>,
    /// Denotes this action uses a Linux base image.
    #[builder(into)]
    #[serde(rename = "linuxAction")]
    pub r#linux_action: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SourceControlGithubActionConfiguration {
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
                    "codeConfiguration",
                    &self.r#code_configuration,
                ),
                to_pulumi_object_field(
                    "containerConfiguration",
                    &self.r#container_configuration,
                ),
                to_pulumi_object_field(
                    "generateWorkflowFile",
                    &self.r#generate_workflow_file,
                ),
                to_pulumi_object_field(
                    "linuxAction",
                    &self.r#linux_action,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SourceControlGithubActionConfiguration {
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
                    r#code_configuration: {
                        let field_value = match fields_map.get("codeConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'codeConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_configuration: {
                        let field_value = match fields_map.get("containerConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#generate_workflow_file: {
                        let field_value = match fields_map.get("generateWorkflowFile") {
                            Some(value) => value,
                            None => bail!("Missing field 'generateWorkflowFile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linux_action: {
                        let field_value = match fields_map.get("linuxAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'linuxAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
