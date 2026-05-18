#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PoolStartTaskContainer {
    /// The image to use to create the container in which the task will run. This is the full image reference, as would be specified to "docker pull". If no tag is provided as part of the image name, the tag ":latest" is used as a default.
    #[builder(into)]
    #[serde(rename = "imageName")]
    pub r#image_name: String,
    /// The `container_registries` block defined as below.
    #[builder(into)]
    #[serde(rename = "registries")]
    pub r#registries: Option<Vec<super::super::types::batch::PoolStartTaskContainerRegistry>>,
    /// Additional options to the container create command. These additional options are supplied as arguments to the "docker create" command, in addition to those controlled by the Batch Service.
    #[builder(into)]
    #[serde(rename = "runOptions")]
    pub r#run_options: Option<String>,
    /// A flag to indicate where the container task working directory is. Possible values are `TaskWorkingDirectory` and `ContainerImageDefault`.
    #[builder(into)]
    #[serde(rename = "workingDirectory")]
    pub r#working_directory: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PoolStartTaskContainer {
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
                    "image_name",
                    &self.r#image_name,
                ),
                to_pulumi_object_field(
                    "registries",
                    &self.r#registries,
                ),
                to_pulumi_object_field(
                    "run_options",
                    &self.r#run_options,
                ),
                to_pulumi_object_field(
                    "working_directory",
                    &self.r#working_directory,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PoolStartTaskContainer {
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
                    r#image_name: {
                        let field_value = match fields_map.get("image_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#registries: {
                        let field_value = match fields_map.get("registries") {
                            Some(value) => value,
                            None => bail!("Missing field 'registries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#run_options: {
                        let field_value = match fields_map.get("run_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'run_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#working_directory: {
                        let field_value = match fields_map.get("working_directory") {
                            Some(value) => value,
                            None => bail!("Missing field 'working_directory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
