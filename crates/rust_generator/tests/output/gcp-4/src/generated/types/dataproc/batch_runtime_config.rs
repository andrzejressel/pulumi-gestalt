#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BatchRuntimeConfig {
    /// Optional. Autotuning configuration of the workload.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "autotuningConfig")]
    pub r#autotuning_config: Option<Box<super::super::types::dataproc::BatchRuntimeConfigAutotuningConfig>>,
    /// Optional. Cohort identifier. Identifies families of the workloads having the same shape, e.g. daily ETL jobs.
    #[builder(into)]
    #[serde(rename = "cohort")]
    pub r#cohort: Option<String>,
    /// Optional custom container image for the job runtime environment. If not specified, a default container image will be used.
    #[builder(into)]
    #[serde(rename = "containerImage")]
    pub r#container_image: Option<String>,
    /// (Output)
    /// A mapping of property names to values, which are used to configure workload execution.
    #[builder(into)]
    #[serde(rename = "effectiveProperties")]
    pub r#effective_properties: Option<std::collections::HashMap<String, String>>,
    /// A mapping of property names to values, which are used to configure workload execution.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
    /// Version of the batch runtime.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BatchRuntimeConfig {
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
                    "autotuning_config",
                    &self.r#autotuning_config,
                ),
                to_pulumi_object_field(
                    "cohort",
                    &self.r#cohort,
                ),
                to_pulumi_object_field(
                    "container_image",
                    &self.r#container_image,
                ),
                to_pulumi_object_field(
                    "effective_properties",
                    &self.r#effective_properties,
                ),
                to_pulumi_object_field(
                    "properties",
                    &self.r#properties,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BatchRuntimeConfig {
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
                    r#autotuning_config: {
                        let field_value = match fields_map.get("autotuning_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'autotuning_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cohort: {
                        let field_value = match fields_map.get("cohort") {
                            Some(value) => value,
                            None => bail!("Missing field 'cohort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_image: {
                        let field_value = match fields_map.get("container_image") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_image' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#effective_properties: {
                        let field_value = match fields_map.get("effective_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'effective_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#properties: {
                        let field_value = match fields_map.get("properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
