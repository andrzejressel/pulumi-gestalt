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
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "access_token".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#access_token,
                )
                .await,
            );
            map.insert(
                "branches".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#branches,
                )
                .await,
            );
            map.insert(
                "display_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#display_name,
                )
                .await,
            );
            map.insert(
                "repository_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#repository_uri,
                )
                .await,
            );
            map.insert(
                "tracking_branch".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tracking_branch,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
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
                        let field_value = match fields_map.get("access_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repository_uri: {
                        let field_value = match fields_map.get("repository_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'repository_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tracking_branch: {
                        let field_value = match fields_map.get("tracking_branch") {
                            Some(value) => value,
                            None => bail!("Missing field 'tracking_branch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
