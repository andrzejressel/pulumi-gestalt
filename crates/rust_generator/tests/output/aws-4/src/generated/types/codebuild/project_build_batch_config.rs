#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProjectBuildBatchConfig {
    /// Specifies if the build artifacts for the batch build should be combined into a single artifact location.
    #[builder(into)]
    #[serde(rename = "combineArtifacts")]
    pub r#combine_artifacts: Option<bool>,
    /// Configuration block specifying the restrictions for the batch build. Detailed below.
    #[builder(into)]
    #[serde(rename = "restrictions")]
    pub r#restrictions: Option<Box<super::super::types::codebuild::ProjectBuildBatchConfigRestrictions>>,
    /// Specifies the service role ARN for the batch build project.
    #[builder(into)]
    #[serde(rename = "serviceRole")]
    pub r#service_role: String,
    /// Specifies the maximum amount of time, in minutes, that the batch build must be completed in.
    #[builder(into)]
    #[serde(rename = "timeoutInMins")]
    pub r#timeout_in_mins: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProjectBuildBatchConfig {
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
                    "combine_artifacts",
                    &self.r#combine_artifacts,
                ),
                to_pulumi_object_field(
                    "restrictions",
                    &self.r#restrictions,
                ),
                to_pulumi_object_field(
                    "service_role",
                    &self.r#service_role,
                ),
                to_pulumi_object_field(
                    "timeout_in_mins",
                    &self.r#timeout_in_mins,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProjectBuildBatchConfig {
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
                    r#combine_artifacts: {
                        let field_value = match fields_map.get("combine_artifacts") {
                            Some(value) => value,
                            None => bail!("Missing field 'combine_artifacts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restrictions: {
                        let field_value = match fields_map.get("restrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'restrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_role: {
                        let field_value = match fields_map.get("service_role") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_in_mins: {
                        let field_value = match fields_map.get("timeout_in_mins") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_in_mins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
