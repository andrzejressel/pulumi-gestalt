#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetControlControlMappingSource {
    #[builder(into)]
    #[serde(rename = "sourceDescription")]
    pub r#source_description: String,
    #[builder(into)]
    #[serde(rename = "sourceFrequency")]
    pub r#source_frequency: String,
    #[builder(into)]
    #[serde(rename = "sourceId")]
    pub r#source_id: String,
    #[builder(into)]
    #[serde(rename = "sourceKeyword")]
    pub r#source_keyword: Option<Box<super::super::types::auditmanager::GetControlControlMappingSourceSourceKeyword>>,
    #[builder(into)]
    #[serde(rename = "sourceName")]
    pub r#source_name: String,
    #[builder(into)]
    #[serde(rename = "sourceSetUpOption")]
    pub r#source_set_up_option: String,
    #[builder(into)]
    #[serde(rename = "sourceType")]
    pub r#source_type: String,
    #[builder(into)]
    #[serde(rename = "troubleshootingText")]
    pub r#troubleshooting_text: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetControlControlMappingSource {
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
                    "source_description",
                    &self.r#source_description,
                ),
                to_pulumi_object_field(
                    "source_frequency",
                    &self.r#source_frequency,
                ),
                to_pulumi_object_field(
                    "source_id",
                    &self.r#source_id,
                ),
                to_pulumi_object_field(
                    "source_keyword",
                    &self.r#source_keyword,
                ),
                to_pulumi_object_field(
                    "source_name",
                    &self.r#source_name,
                ),
                to_pulumi_object_field(
                    "source_set_up_option",
                    &self.r#source_set_up_option,
                ),
                to_pulumi_object_field(
                    "source_type",
                    &self.r#source_type,
                ),
                to_pulumi_object_field(
                    "troubleshooting_text",
                    &self.r#troubleshooting_text,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetControlControlMappingSource {
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
                    r#source_description: {
                        let field_value = match fields_map.get("source_description") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_frequency: {
                        let field_value = match fields_map.get("source_frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_id: {
                        let field_value = match fields_map.get("source_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_keyword: {
                        let field_value = match fields_map.get("source_keyword") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_keyword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_name: {
                        let field_value = match fields_map.get("source_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_set_up_option: {
                        let field_value = match fields_map.get("source_set_up_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_set_up_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_type: {
                        let field_value = match fields_map.get("source_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#troubleshooting_text: {
                        let field_value = match fields_map.get("troubleshooting_text") {
                            Some(value) => value,
                            None => bail!("Missing field 'troubleshooting_text' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
