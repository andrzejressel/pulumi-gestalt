#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFunctionAppSourceControl {
    /// The branch of the remote repository in use.
    #[builder(into)]
    pub r#branch: String,
    /// Limits to manual integration.
    #[builder(into)]
    pub r#manual_integration: bool,
    /// The URL of the source code repository.
    #[builder(into)]
    pub r#repo_url: String,
    /// Is roll-back enabled for the repository.
    #[builder(into)]
    pub r#rollback_enabled: bool,
    /// Uses Mercurial if `true`, otherwise uses Git.
    #[builder(into)]
    pub r#use_mercurial: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetFunctionAppSourceControl {
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
                    "branch",
                    &self.r#branch,
                ),
                to_pulumi_object_field(
                    "manualIntegration",
                    &self.r#manual_integration,
                ),
                to_pulumi_object_field(
                    "repoUrl",
                    &self.r#repo_url,
                ),
                to_pulumi_object_field(
                    "rollbackEnabled",
                    &self.r#rollback_enabled,
                ),
                to_pulumi_object_field(
                    "useMercurial",
                    &self.r#use_mercurial,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetFunctionAppSourceControl {
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
                    r#branch: {
                        let field_value = match fields_map.get("branch") {
                            Some(value) => value,
                            None => bail!("Missing field 'branch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#manual_integration: {
                        let field_value = match fields_map.get("manualIntegration") {
                            Some(value) => value,
                            None => bail!("Missing field 'manualIntegration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repo_url: {
                        let field_value = match fields_map.get("repoUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'repoUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rollback_enabled: {
                        let field_value = match fields_map.get("rollbackEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'rollbackEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_mercurial: {
                        let field_value = match fields_map.get("useMercurial") {
                            Some(value) => value,
                            None => bail!("Missing field 'useMercurial' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
