#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomTargetTypeCustomActionsIncludeSkaffoldModule {
    /// The Skaffold Config modules to use from the specified source.
    #[builder(into)]
    #[serde(rename = "configs")]
    pub r#configs: Option<Vec<String>>,
    /// Remote git repository containing the Skaffold Config modules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "git")]
    pub r#git: Option<Box<super::super::types::clouddeploy::CustomTargetTypeCustomActionsIncludeSkaffoldModuleGit>>,
    /// Cloud Build 2nd gen repository containing the Skaffold Config modules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "googleCloudBuildRepo")]
    pub r#google_cloud_build_repo: Option<Box<super::super::types::clouddeploy::CustomTargetTypeCustomActionsIncludeSkaffoldModuleGoogleCloudBuildRepo>>,
    /// Cloud Storage bucket containing Skaffold Config modules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "googleCloudStorage")]
    pub r#google_cloud_storage: Option<Box<super::super::types::clouddeploy::CustomTargetTypeCustomActionsIncludeSkaffoldModuleGoogleCloudStorage>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomTargetTypeCustomActionsIncludeSkaffoldModule {
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
                    "configs",
                    &self.r#configs,
                ),
                to_pulumi_object_field(
                    "git",
                    &self.r#git,
                ),
                to_pulumi_object_field(
                    "google_cloud_build_repo",
                    &self.r#google_cloud_build_repo,
                ),
                to_pulumi_object_field(
                    "google_cloud_storage",
                    &self.r#google_cloud_storage,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CustomTargetTypeCustomActionsIncludeSkaffoldModule {
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
                    r#configs: {
                        let field_value = match fields_map.get("configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#git: {
                        let field_value = match fields_map.get("git") {
                            Some(value) => value,
                            None => bail!("Missing field 'git' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#google_cloud_build_repo: {
                        let field_value = match fields_map.get("google_cloud_build_repo") {
                            Some(value) => value,
                            None => bail!("Missing field 'google_cloud_build_repo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#google_cloud_storage: {
                        let field_value = match fields_map.get("google_cloud_storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'google_cloud_storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
