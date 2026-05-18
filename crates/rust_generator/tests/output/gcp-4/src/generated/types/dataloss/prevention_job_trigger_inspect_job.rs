#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJob {
    /// Configuration block for the actions to execute on the completion of a job. Can be specified multiple times, but only one for each type. Each action block supports fields documented below. This argument is processed in attribute-as-blocks mode.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobAction>>,
    /// The core content of the template.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "inspectConfig")]
    pub r#inspect_config: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfig>>,
    /// The name of the template to run when this job is triggered.
    #[builder(into)]
    #[serde(rename = "inspectTemplateName")]
    pub r#inspect_template_name: Option<String>,
    /// Information on where to inspect
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "storageConfig")]
    pub r#storage_config: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJob {
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
                    "actions",
                    &self.r#actions,
                ),
                to_pulumi_object_field(
                    "inspect_config",
                    &self.r#inspect_config,
                ),
                to_pulumi_object_field(
                    "inspect_template_name",
                    &self.r#inspect_template_name,
                ),
                to_pulumi_object_field(
                    "storage_config",
                    &self.r#storage_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJob {
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
                    r#actions: {
                        let field_value = match fields_map.get("actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inspect_config: {
                        let field_value = match fields_map.get("inspect_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'inspect_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inspect_template_name: {
                        let field_value = match fields_map.get("inspect_template_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'inspect_template_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_config: {
                        let field_value = match fields_map.get("storage_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
