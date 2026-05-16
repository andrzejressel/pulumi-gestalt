#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkflowStep {
    /// Details for a step that performs a file copy. See Copy Step Details below.
    #[builder(into)]
    #[serde(rename = "copyStepDetails")]
    pub r#copy_step_details: Option<Box<super::super::types::transfer::WorkflowStepCopyStepDetails>>,
    /// Details for a step that invokes a lambda function.
    #[builder(into)]
    #[serde(rename = "customStepDetails")]
    pub r#custom_step_details: Option<Box<super::super::types::transfer::WorkflowStepCustomStepDetails>>,
    /// Details for a step that decrypts the file.
    #[builder(into)]
    #[serde(rename = "decryptStepDetails")]
    pub r#decrypt_step_details: Option<Box<super::super::types::transfer::WorkflowStepDecryptStepDetails>>,
    /// Details for a step that deletes the file.
    #[builder(into)]
    #[serde(rename = "deleteStepDetails")]
    pub r#delete_step_details: Option<Box<super::super::types::transfer::WorkflowStepDeleteStepDetails>>,
    /// Details for a step that creates one or more tags.
    #[builder(into)]
    #[serde(rename = "tagStepDetails")]
    pub r#tag_step_details: Option<Box<super::super::types::transfer::WorkflowStepTagStepDetails>>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkflowStep {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("copy_step_details".to_string(), self.r#copy_step_details.to_pulumi_value().await);
            map.insert("custom_step_details".to_string(), self.r#custom_step_details.to_pulumi_value().await);
            map.insert("decrypt_step_details".to_string(), self.r#decrypt_step_details.to_pulumi_value().await);
            map.insert("delete_step_details".to_string(), self.r#delete_step_details.to_pulumi_value().await);
            map.insert("tag_step_details".to_string(), self.r#tag_step_details.to_pulumi_value().await);
            map.insert("type_".to_string(), self.r#type_.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkflowStep {
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
                    r#copy_step_details: {
                        let field_value = match fields_map.get("copy_step_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'copy_step_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::transfer::WorkflowStepCopyStepDetails>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_step_details: {
                        let field_value = match fields_map.get("custom_step_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_step_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::transfer::WorkflowStepCustomStepDetails>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#decrypt_step_details: {
                        let field_value = match fields_map.get("decrypt_step_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'decrypt_step_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::transfer::WorkflowStepDecryptStepDetails>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#delete_step_details: {
                        let field_value = match fields_map.get("delete_step_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete_step_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::transfer::WorkflowStepDeleteStepDetails>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tag_step_details: {
                        let field_value = match fields_map.get("tag_step_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_step_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::transfer::WorkflowStepTagStepDetails>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
