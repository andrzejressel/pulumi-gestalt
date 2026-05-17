#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTriggerRepositoryEventConfig {
    /// Contains filter properties for matching Pull Requests.
    #[builder(into)]
    #[serde(rename = "pullRequests")]
    pub r#pull_requests: Vec<super::super::types::cloudbuild::GetTriggerRepositoryEventConfigPullRequest>,
    /// Contains filter properties for matching git pushes.
    #[builder(into)]
    #[serde(rename = "pushes")]
    pub r#pushes: Vec<super::super::types::cloudbuild::GetTriggerRepositoryEventConfigPush>,
    /// The resource name of the Repo API resource.
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTriggerRepositoryEventConfig {
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
                "pull_requests".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pull_requests,
                )
                .await,
            );
            map.insert(
                "pushes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pushes,
                )
                .await,
            );
            map.insert(
                "repository".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#repository,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTriggerRepositoryEventConfig {
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
                    r#pull_requests: {
                        let field_value = match fields_map.get("pull_requests") {
                            Some(value) => value,
                            None => bail!("Missing field 'pull_requests' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pushes: {
                        let field_value = match fields_map.get("pushes") {
                            Some(value) => value,
                            None => bail!("Missing field 'pushes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repository: {
                        let field_value = match fields_map.get("repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
