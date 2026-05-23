#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkflowOnExceptionStep {
    /// Details for a step that performs a file copy. See Copy Step Details below.
    #[builder(into)]
    pub r#copy_step_details: Option<Box<super::super::types::transfer::WorkflowOnExceptionStepCopyStepDetails>>,
    /// Details for a step that invokes a lambda function.
    #[builder(into)]
    pub r#custom_step_details: Option<Box<super::super::types::transfer::WorkflowOnExceptionStepCustomStepDetails>>,
    /// Details for a step that decrypts the file.
    #[builder(into)]
    pub r#decrypt_step_details: Option<Box<super::super::types::transfer::WorkflowOnExceptionStepDecryptStepDetails>>,
    /// Details for a step that deletes the file.
    #[builder(into)]
    pub r#delete_step_details: Option<Box<super::super::types::transfer::WorkflowOnExceptionStepDeleteStepDetails>>,
    /// Details for a step that creates one or more tags.
    #[builder(into)]
    pub r#tag_step_details: Option<Box<super::super::types::transfer::WorkflowOnExceptionStepTagStepDetails>>,
    #[builder(into)]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkflowOnExceptionStep {
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
                    "copyStepDetails",
                    &self.r#copy_step_details,
                ),
                to_pulumi_object_field(
                    "customStepDetails",
                    &self.r#custom_step_details,
                ),
                to_pulumi_object_field(
                    "decryptStepDetails",
                    &self.r#decrypt_step_details,
                ),
                to_pulumi_object_field(
                    "deleteStepDetails",
                    &self.r#delete_step_details,
                ),
                to_pulumi_object_field(
                    "tagStepDetails",
                    &self.r#tag_step_details,
                ),
                to_pulumi_object_field(
                    "type",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkflowOnExceptionStep {
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
                    r#copy_step_details: {
                        let field_value = match fields_map.get("copyStepDetails") {
                            Some(value) => value,
                            None => bail!("Missing field 'copyStepDetails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_step_details: {
                        let field_value = match fields_map.get("customStepDetails") {
                            Some(value) => value,
                            None => bail!("Missing field 'customStepDetails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#decrypt_step_details: {
                        let field_value = match fields_map.get("decryptStepDetails") {
                            Some(value) => value,
                            None => bail!("Missing field 'decryptStepDetails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delete_step_details: {
                        let field_value = match fields_map.get("deleteStepDetails") {
                            Some(value) => value,
                            None => bail!("Missing field 'deleteStepDetails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_step_details: {
                        let field_value = match fields_map.get("tagStepDetails") {
                            Some(value) => value,
                            None => bail!("Missing field 'tagStepDetails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type") {
                            Some(value) => value,
                            None => bail!("Missing field 'type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
