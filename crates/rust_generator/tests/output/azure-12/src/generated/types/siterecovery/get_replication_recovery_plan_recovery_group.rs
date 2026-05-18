#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetReplicationRecoveryPlanRecoveryGroup {
    /// one or more `action` block. which will be executed after the group recovery.
    #[builder(into)]
    #[serde(rename = "postActions")]
    pub r#post_actions: Vec<Vec<super::super::types::siterecovery::GetReplicationRecoveryPlanRecoveryGroupPostAction>>,
    /// one or more `action` block. which will be executed before the group recovery.
    #[builder(into)]
    #[serde(rename = "preActions")]
    pub r#pre_actions: Vec<Vec<super::super::types::siterecovery::GetReplicationRecoveryPlanRecoveryGroupPreAction>>,
    /// one or more id of protected VM.
    #[builder(into)]
    #[serde(rename = "replicatedProtectedItems")]
    pub r#replicated_protected_items: Vec<String>,
    /// Type of the action detail.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetReplicationRecoveryPlanRecoveryGroup {
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
                    "post_actions",
                    &self.r#post_actions,
                ),
                to_pulumi_object_field(
                    "pre_actions",
                    &self.r#pre_actions,
                ),
                to_pulumi_object_field(
                    "replicated_protected_items",
                    &self.r#replicated_protected_items,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetReplicationRecoveryPlanRecoveryGroup {
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
                    r#post_actions: {
                        let field_value = match fields_map.get("post_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pre_actions: {
                        let field_value = match fields_map.get("pre_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replicated_protected_items: {
                        let field_value = match fields_map.get("replicated_protected_items") {
                            Some(value) => value,
                            None => bail!("Missing field 'replicated_protected_items' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
