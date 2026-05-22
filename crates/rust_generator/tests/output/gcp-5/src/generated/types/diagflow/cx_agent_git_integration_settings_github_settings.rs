#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxAgentGitIntegrationSettingsGithubSettings {
    /// The access token used to authenticate the access to the GitHub repository.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Option<String>,
    /// A list of branches configured to be used from Dialogflow.
    #[builder(into)]
    #[serde(rename = "branches")]
    pub r#branches: Option<Vec<String>>,
    /// The unique repository display name for the GitHub repository.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// The GitHub repository URI related to the agent.
    #[builder(into)]
    #[serde(rename = "repositoryUri")]
    pub r#repository_uri: Option<String>,
    /// The branch of the GitHub repository tracked for this agent.
    #[builder(into)]
    #[serde(rename = "trackingBranch")]
    pub r#tracking_branch: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxAgentGitIntegrationSettingsGithubSettings {
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
                    "accessToken",
                    &self.r#access_token,
                ),
                to_pulumi_object_field(
                    "branches",
                    &self.r#branches,
                ),
                to_pulumi_object_field(
                    "displayName",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "repositoryUri",
                    &self.r#repository_uri,
                ),
                to_pulumi_object_field(
                    "trackingBranch",
                    &self.r#tracking_branch,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxAgentGitIntegrationSettingsGithubSettings {
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
                    r#access_token: {
                        let field_value = match fields_map.get("accessToken") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessToken' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#branches: {
                        let field_value = match fields_map.get("branches") {
                            Some(value) => value,
                            None => bail!("Missing field 'branches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#repository_uri: {
                        let field_value = match fields_map.get("repositoryUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'repositoryUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tracking_branch: {
                        let field_value = match fields_map.get("trackingBranch") {
                            Some(value) => value,
                            None => bail!("Missing field 'trackingBranch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
