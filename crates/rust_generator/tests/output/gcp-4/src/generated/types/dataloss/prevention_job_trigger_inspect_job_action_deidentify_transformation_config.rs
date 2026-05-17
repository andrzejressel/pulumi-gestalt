#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobActionDeidentifyTransformationConfig {
    /// If this template is specified, it will serve as the default de-identify template.
    #[builder(into)]
    #[serde(rename = "deidentifyTemplate")]
    pub r#deidentify_template: Option<String>,
    /// If this template is specified, it will serve as the de-identify template for images.
    #[builder(into)]
    #[serde(rename = "imageRedactTemplate")]
    pub r#image_redact_template: Option<String>,
    /// If this template is specified, it will serve as the de-identify template for structured content such as delimited files and tables.
    #[builder(into)]
    #[serde(rename = "structuredDeidentifyTemplate")]
    pub r#structured_deidentify_template: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobActionDeidentifyTransformationConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "deidentify_template",
                    &self.r#deidentify_template,
                ),
                to_pulumi_object_field(
                    "image_redact_template",
                    &self.r#image_redact_template,
                ),
                to_pulumi_object_field(
                    "structured_deidentify_template",
                    &self.r#structured_deidentify_template,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobActionDeidentifyTransformationConfig {
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
                    r#deidentify_template: {
                        let field_value = match fields_map.get("deidentify_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'deidentify_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_redact_template: {
                        let field_value = match fields_map.get("image_redact_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_redact_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#structured_deidentify_template: {
                        let field_value = match fields_map.get("structured_deidentify_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'structured_deidentify_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
