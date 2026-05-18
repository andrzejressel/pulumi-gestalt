#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PagesProjectBuildConfig {
    /// Enable build caching for the project.
    #[builder(into)]
    #[serde(rename = "buildCaching")]
    pub r#build_caching: Option<bool>,
    /// Command used to build project.
    #[builder(into)]
    #[serde(rename = "buildCommand")]
    pub r#build_command: Option<String>,
    /// Output directory of the build.
    #[builder(into)]
    #[serde(rename = "destinationDir")]
    pub r#destination_dir: Option<String>,
    /// Your project's root directory, where Cloudflare runs the build command. If your site is not in a subdirectory, leave this path value empty.
    #[builder(into)]
    #[serde(rename = "rootDir")]
    pub r#root_dir: Option<String>,
    /// The classifying tag for analytics.
    #[builder(into)]
    #[serde(rename = "webAnalyticsTag")]
    pub r#web_analytics_tag: Option<String>,
    /// The auth token for analytics.
    #[builder(into)]
    #[serde(rename = "webAnalyticsToken")]
    pub r#web_analytics_token: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PagesProjectBuildConfig {
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
                    "build_caching",
                    &self.r#build_caching,
                ),
                to_pulumi_object_field(
                    "build_command",
                    &self.r#build_command,
                ),
                to_pulumi_object_field(
                    "destination_dir",
                    &self.r#destination_dir,
                ),
                to_pulumi_object_field(
                    "root_dir",
                    &self.r#root_dir,
                ),
                to_pulumi_object_field(
                    "web_analytics_tag",
                    &self.r#web_analytics_tag,
                ),
                to_pulumi_object_field(
                    "web_analytics_token",
                    &self.r#web_analytics_token,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PagesProjectBuildConfig {
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
                    r#build_caching: {
                        let field_value = match fields_map.get("build_caching") {
                            Some(value) => value,
                            None => bail!("Missing field 'build_caching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#build_command: {
                        let field_value = match fields_map.get("build_command") {
                            Some(value) => value,
                            None => bail!("Missing field 'build_command' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_dir: {
                        let field_value = match fields_map.get("destination_dir") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_dir' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_dir: {
                        let field_value = match fields_map.get("root_dir") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_dir' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_analytics_tag: {
                        let field_value = match fields_map.get("web_analytics_tag") {
                            Some(value) => value,
                            None => bail!("Missing field 'web_analytics_tag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_analytics_token: {
                        let field_value = match fields_map.get("web_analytics_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'web_analytics_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
