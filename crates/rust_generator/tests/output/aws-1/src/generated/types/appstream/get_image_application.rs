#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetImageApplication {
    /// The app block ARN of the application.
    #[builder(into)]
    pub r#app_block_arn: String,
    /// Arn of the image being searched for. Cannot be used with name_regex or name.
    #[builder(into)]
    pub r#arn: String,
    /// Time at which this image was created.
    #[builder(into)]
    pub r#created_time: String,
    /// Description of image.
    #[builder(into)]
    pub r#description: String,
    /// Image name to display.
    #[builder(into)]
    pub r#display_name: String,
    /// Bool based on if the application is enabled.
    #[builder(into)]
    pub r#enabled: bool,
    /// A list named icon_s3_location that contains the following:
    #[builder(into)]
    pub r#icon_s_3_locations: Vec<super::super::types::appstream::GetImageApplicationIconS3Location>,
    /// URL of the application icon. This URL may be time-limited.
    #[builder(into)]
    pub r#icon_url: String,
    /// List of the instance families of the application.
    #[builder(into)]
    pub r#instance_families: Vec<String>,
    /// Arguments that are passed to the application at it's launch.
    #[builder(into)]
    pub r#launch_parameters: String,
    /// Path to the application's excecutable in the instance.
    #[builder(into)]
    pub r#launch_path: String,
    /// String to string map that contains additional attributes used to describe the application.
    /// * `Name` - Name of the application.
    #[builder(into)]
    pub r#metadata: std::collections::HashMap<String, String>,
    /// Name of the image being searched for. Cannot be used with name_regex or arn.
    #[builder(into)]
    pub r#name: String,
    /// Array of strings describing the platforms on which the application can run.
    /// Values will be from: WINDOWS | WINDOWS_SERVER_2016 | WINDOWS_SERVER_2019 | WINDOWS_SERVER_2022 | AMAZON_LINUX2
    #[builder(into)]
    pub r#platforms: Vec<String>,
    /// Working directory for the application.
    #[builder(into)]
    pub r#working_directory: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetImageApplication {
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
                    "appBlockArn",
                    &self.r#app_block_arn,
                ),
                to_pulumi_object_field(
                    "arn",
                    &self.r#arn,
                ),
                to_pulumi_object_field(
                    "createdTime",
                    &self.r#created_time,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "displayName",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "iconS3Locations",
                    &self.r#icon_s_3_locations,
                ),
                to_pulumi_object_field(
                    "iconUrl",
                    &self.r#icon_url,
                ),
                to_pulumi_object_field(
                    "instanceFamilies",
                    &self.r#instance_families,
                ),
                to_pulumi_object_field(
                    "launchParameters",
                    &self.r#launch_parameters,
                ),
                to_pulumi_object_field(
                    "launchPath",
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
                    "workingDirectory",
                    &self.r#working_directory,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("appBlockArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'appBlockArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("createdTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'createdTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("displayName") {
                            Some(value) => value,
                            None => bail!("Missing field 'displayName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("iconS3Locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'iconS3Locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#icon_url: {
                        let field_value = match fields_map.get("iconUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'iconUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_families: {
                        let field_value = match fields_map.get("instanceFamilies") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceFamilies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_parameters: {
                        let field_value = match fields_map.get("launchParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'launchParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_path: {
                        let field_value = match fields_map.get("launchPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'launchPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("workingDirectory") {
                            Some(value) => value,
                            None => bail!("Missing field 'workingDirectory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
