#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TriggerGithub {
    /// The resource name of the github enterprise config that should be applied to this installation.
    /// For example: "projects/{$projectId}/locations/{$locationId}/githubEnterpriseConfigs/{$configId}"
    #[builder(into)]
    #[serde(rename = "enterpriseConfigResourceName")]
    pub r#enterprise_config_resource_name: Option<String>,
    /// Name of the repository. For example: The name for
    /// https://github.com/googlecloudplatform/cloud-builders is "cloud-builders".
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Owner of the repository. For example: The owner for
    /// https://github.com/googlecloudplatform/cloud-builders is "googlecloudplatform".
    #[builder(into)]
    #[serde(rename = "owner")]
    pub r#owner: Option<String>,
    /// filter to match changes in pull requests. Specify only one of `pull_request` or `push`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pullRequest")]
    pub r#pull_request: Option<Box<super::super::types::cloudbuild::TriggerGithubPullRequest>>,
    /// filter to match changes in refs, like branches or tags. Specify only one of `pull_request` or `push`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "push")]
    pub r#push: Option<Box<super::super::types::cloudbuild::TriggerGithubPush>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TriggerGithub {
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
                    "enterprise_config_resource_name",
                    &self.r#enterprise_config_resource_name,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "owner",
                    &self.r#owner,
                ),
                to_pulumi_object_field(
                    "pull_request",
                    &self.r#pull_request,
                ),
                to_pulumi_object_field(
                    "push",
                    &self.r#push,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TriggerGithub {
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
                    r#enterprise_config_resource_name: {
                        let field_value = match fields_map.get("enterprise_config_resource_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'enterprise_config_resource_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#owner: {
                        let field_value = match fields_map.get("owner") {
                            Some(value) => value,
                            None => bail!("Missing field 'owner' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pull_request: {
                        let field_value = match fields_map.get("pull_request") {
                            Some(value) => value,
                            None => bail!("Missing field 'pull_request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#push: {
                        let field_value = match fields_map.get("push") {
                            Some(value) => value,
                            None => bail!("Missing field 'push' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
