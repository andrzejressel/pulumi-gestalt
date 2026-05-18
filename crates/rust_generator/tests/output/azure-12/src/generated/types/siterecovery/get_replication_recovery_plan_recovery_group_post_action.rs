#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetReplicationRecoveryPlanRecoveryGroupPostAction {
    /// The fabric location of runbook or script.
    #[builder(into)]
    #[serde(rename = "fabricLocation")]
    pub r#fabric_location: String,
    /// Directions of fail over.
    #[builder(into)]
    #[serde(rename = "failOverDirections")]
    pub r#fail_over_directions: Vec<String>,
    /// Types of fail over.
    #[builder(into)]
    #[serde(rename = "failOverTypes")]
    pub r#fail_over_types: Vec<String>,
    /// Instructions of manual action.
    #[builder(into)]
    #[serde(rename = "manualActionInstruction")]
    pub r#manual_action_instruction: String,
    /// The name of the Replication Plan.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Id of runbook.
    #[builder(into)]
    #[serde(rename = "runbookId")]
    pub r#runbook_id: String,
    /// Path of action script.
    #[builder(into)]
    #[serde(rename = "scriptPath")]
    pub r#script_path: String,
    /// Type of the action detail.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetReplicationRecoveryPlanRecoveryGroupPostAction {
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
                    "fabric_location",
                    &self.r#fabric_location,
                ),
                to_pulumi_object_field(
                    "fail_over_directions",
                    &self.r#fail_over_directions,
                ),
                to_pulumi_object_field(
                    "fail_over_types",
                    &self.r#fail_over_types,
                ),
                to_pulumi_object_field(
                    "manual_action_instruction",
                    &self.r#manual_action_instruction,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "runbook_id",
                    &self.r#runbook_id,
                ),
                to_pulumi_object_field(
                    "script_path",
                    &self.r#script_path,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetReplicationRecoveryPlanRecoveryGroupPostAction {
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
                    r#fabric_location: {
                        let field_value = match fields_map.get("fabric_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'fabric_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fail_over_directions: {
                        let field_value = match fields_map.get("fail_over_directions") {
                            Some(value) => value,
                            None => bail!("Missing field 'fail_over_directions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fail_over_types: {
                        let field_value = match fields_map.get("fail_over_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'fail_over_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#manual_action_instruction: {
                        let field_value = match fields_map.get("manual_action_instruction") {
                            Some(value) => value,
                            None => bail!("Missing field 'manual_action_instruction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#runbook_id: {
                        let field_value = match fields_map.get("runbook_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'runbook_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#script_path: {
                        let field_value = match fields_map.get("script_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'script_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
