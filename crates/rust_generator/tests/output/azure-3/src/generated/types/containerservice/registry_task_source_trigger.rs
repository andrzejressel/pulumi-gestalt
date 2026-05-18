#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistryTaskSourceTrigger {
    /// A `authentication` block as defined above.
    #[builder(into)]
    #[serde(rename = "authentication")]
    pub r#authentication: Option<Box<super::super::types::containerservice::RegistryTaskSourceTriggerAuthentication>>,
    /// The branch name of the source code.
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: Option<String>,
    /// Should the trigger be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Specifies a list of source events corresponding to the trigger. Possible values are `commit` and `pullrequest`.
    #[builder(into)]
    #[serde(rename = "events")]
    pub r#events: Vec<String>,
    /// The name which should be used for this trigger.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The full URL to the source code repository.
    #[builder(into)]
    #[serde(rename = "repositoryUrl")]
    pub r#repository_url: String,
    /// The type of the source control service. Possible values are `Github` and `VisualStudioTeamService`.
    #[builder(into)]
    #[serde(rename = "sourceType")]
    pub r#source_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegistryTaskSourceTrigger {
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
                    "authentication",
                    &self.r#authentication,
                ),
                to_pulumi_object_field(
                    "branch",
                    &self.r#branch,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "events",
                    &self.r#events,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "repository_url",
                    &self.r#repository_url,
                ),
                to_pulumi_object_field(
                    "source_type",
                    &self.r#source_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegistryTaskSourceTrigger {
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
                    r#authentication: {
                        let field_value = match fields_map.get("authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#branch: {
                        let field_value = match fields_map.get("branch") {
                            Some(value) => value,
                            None => bail!("Missing field 'branch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#events: {
                        let field_value = match fields_map.get("events") {
                            Some(value) => value,
                            None => bail!("Missing field 'events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#repository_url: {
                        let field_value = match fields_map.get("repository_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'repository_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_type: {
                        let field_value = match fields_map.get("source_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
