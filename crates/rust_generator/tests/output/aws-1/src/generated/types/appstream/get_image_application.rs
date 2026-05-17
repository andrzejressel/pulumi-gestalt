#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetImageApplication {
    /// The app block ARN of the application.
    #[builder(into)]
    #[serde(rename = "appBlockArn")]
    pub r#app_block_arn: String,
    /// Arn of the image being searched for. Cannot be used with name_regex or name.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// Time at which this image was created.
    #[builder(into)]
    #[serde(rename = "createdTime")]
    pub r#created_time: String,
    /// Description of image.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// Image name to display.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    /// Bool based on if the application is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// A list named icon_s3_location that contains the following:
    #[builder(into)]
    #[serde(rename = "iconS3Locations")]
    pub r#icon_s_3_locations: Vec<super::super::types::appstream::GetImageApplicationIconS3Location>,
    /// URL of the application icon. This URL may be time-limited.
    #[builder(into)]
    #[serde(rename = "iconUrl")]
    pub r#icon_url: String,
    /// List of the instance families of the application.
    #[builder(into)]
    #[serde(rename = "instanceFamilies")]
    pub r#instance_families: Vec<String>,
    /// Arguments that are passed to the application at it's launch.
    #[builder(into)]
    #[serde(rename = "launchParameters")]
    pub r#launch_parameters: String,
    /// Path to the application's excecutable in the instance.
    #[builder(into)]
    #[serde(rename = "launchPath")]
    pub r#launch_path: String,
    /// String to string map that contains additional attributes used to describe the application.
    /// * `Name` - Name of the application.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: std::collections::HashMap<String, String>,
    /// Name of the image being searched for. Cannot be used with name_regex or arn.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Array of strings describing the platforms on which the application can run.
    /// Values will be from: WINDOWS | WINDOWS_SERVER_2016 | WINDOWS_SERVER_2019 | WINDOWS_SERVER_2022 | AMAZON_LINUX2
    #[builder(into)]
    #[serde(rename = "platforms")]
    pub r#platforms: Vec<String>,
    /// Working directory for the application.
    #[builder(into)]
    #[serde(rename = "workingDirectory")]
    pub r#working_directory: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetImageApplication {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "app_block_arn",
                    &self.r#app_block_arn,
                ),
                to_pulumi_object_field(
                    "arn",
                    &self.r#arn,
                ),
                to_pulumi_object_field(
                    "created_time",
                    &self.r#created_time,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "display_name",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "icon_s_3_locations",
                    &self.r#icon_s_3_locations,
                ),
                to_pulumi_object_field(
                    "icon_url",
                    &self.r#icon_url,
                ),
                to_pulumi_object_field(
                    "instance_families",
                    &self.r#instance_families,
                ),
                to_pulumi_object_field(
                    "launch_parameters",
                    &self.r#launch_parameters,
                ),
                to_pulumi_object_field(
                    "launch_path",
                    &self.r#launch_path,
                ),
                to_pulumi_object_field(
                    "metadata",
                    &self.r#metadata,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "platforms",
                    &self.r#platforms,
                ),
                to_pulumi_object_field(
                    "working_directory",
                    &self.r#working_directory,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetImageApplication {
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
                    r#app_block_arn: {
                        let field_value = match fields_map.get("app_block_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_block_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#arn: {
                        let field_value = match fields_map.get("arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#created_time: {
                        let field_value = match fields_map.get("created_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'created_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#icon_s_3_locations: {
                        let field_value = match fields_map.get("icon_s_3_locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'icon_s_3_locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#icon_url: {
                        let field_value = match fields_map.get("icon_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'icon_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_families: {
                        let field_value = match fields_map.get("instance_families") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_families' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_parameters: {
                        let field_value = match fields_map.get("launch_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_path: {
                        let field_value = match fields_map.get("launch_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadata: {
                        let field_value = match fields_map.get("metadata") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#platforms: {
                        let field_value = match fields_map.get("platforms") {
                            Some(value) => value,
                            None => bail!("Missing field 'platforms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
