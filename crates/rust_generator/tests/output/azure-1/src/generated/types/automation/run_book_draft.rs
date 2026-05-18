#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RunBookDraft {
    /// A `publish_content_link` block as defined above.
    #[builder(into)]
    #[serde(rename = "contentLink")]
    pub r#content_link: Option<Box<super::super::types::automation::RunBookDraftContentLink>>,
    #[builder(into)]
    #[serde(rename = "creationTime")]
    pub r#creation_time: Option<String>,
    /// Whether the draft in edit mode.
    #[builder(into)]
    #[serde(rename = "editModeEnabled")]
    pub r#edit_mode_enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "lastModifiedTime")]
    pub r#last_modified_time: Option<String>,
    /// Specifies the output types of the runbook.
    #[builder(into)]
    #[serde(rename = "outputTypes")]
    pub r#output_types: Option<Vec<String>>,
    /// A list of `parameters` block as defined below.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::super::types::automation::RunBookDraftParameter>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RunBookDraft {
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
                    "content_link",
                    &self.r#content_link,
                ),
                to_pulumi_object_field(
                    "creation_time",
                    &self.r#creation_time,
                ),
                to_pulumi_object_field(
                    "edit_mode_enabled",
                    &self.r#edit_mode_enabled,
                ),
                to_pulumi_object_field(
                    "last_modified_time",
                    &self.r#last_modified_time,
                ),
                to_pulumi_object_field(
                    "output_types",
                    &self.r#output_types,
                ),
                to_pulumi_object_field(
                    "parameters",
                    &self.r#parameters,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RunBookDraft {
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
                    r#content_link: {
                        let field_value = match fields_map.get("content_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#creation_time: {
                        let field_value = match fields_map.get("creation_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'creation_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#edit_mode_enabled: {
                        let field_value = match fields_map.get("edit_mode_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'edit_mode_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_modified_time: {
                        let field_value = match fields_map.get("last_modified_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_modified_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_types: {
                        let field_value = match fields_map.get("output_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
