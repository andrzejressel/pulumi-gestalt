#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateJobTemplateDataJobDriverSparkSubmitJobDriver {
    /// The entry point of job application.
    #[builder(into)]
    #[serde(rename = "entryPoint")]
    pub r#entry_point: String,
    /// The arguments for job application.
    #[builder(into)]
    #[serde(rename = "entryPointArguments")]
    pub r#entry_point_arguments: Option<Vec<String>>,
    /// The Spark submit parameters that are used for job runs.
    #[builder(into)]
    #[serde(rename = "sparkSubmitParameters")]
    pub r#spark_submit_parameters: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateJobTemplateDataJobDriverSparkSubmitJobDriver {
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
                    "entry_point",
                    &self.r#entry_point,
                ),
                to_pulumi_object_field(
                    "entry_point_arguments",
                    &self.r#entry_point_arguments,
                ),
                to_pulumi_object_field(
                    "spark_submit_parameters",
                    &self.r#spark_submit_parameters,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobTemplateJobTemplateDataJobDriverSparkSubmitJobDriver {
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
                    r#entry_point: {
                        let field_value = match fields_map.get("entry_point") {
                            Some(value) => value,
                            None => bail!("Missing field 'entry_point' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#entry_point_arguments: {
                        let field_value = match fields_map.get("entry_point_arguments") {
                            Some(value) => value,
                            None => bail!("Missing field 'entry_point_arguments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spark_submit_parameters: {
                        let field_value = match fields_map.get("spark_submit_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'spark_submit_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
