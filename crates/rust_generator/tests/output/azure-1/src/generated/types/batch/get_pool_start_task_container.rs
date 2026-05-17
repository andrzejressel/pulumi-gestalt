#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPoolStartTaskContainer {
    /// The image to use to create the container in which the task will run.
    #[builder(into)]
    #[serde(rename = "imageName")]
    pub r#image_name: String,
    /// The same reference as `container_registries` block defined as follows.
    #[builder(into)]
    #[serde(rename = "registries")]
    pub r#registries: Vec<super::super::types::batch::GetPoolStartTaskContainerRegistry>,
    /// Additional options to the container create command.
    #[builder(into)]
    #[serde(rename = "runOptions")]
    pub r#run_options: String,
    /// A flag to indicate where the container task working directory is.
    #[builder(into)]
    #[serde(rename = "workingDirectory")]
    pub r#working_directory: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPoolStartTaskContainer {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "image_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_name,
                )
                .await,
            );
            map.insert(
                "registries".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#registries,
                )
                .await,
            );
            map.insert(
                "run_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#run_options,
                )
                .await,
            );
            map.insert(
                "working_directory".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#working_directory,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPoolStartTaskContainer {
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
