#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudServiceConfigServerGitSetting {
    /// A `http_basic_auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "httpBasicAuth")]
    pub r#http_basic_auth: Option<Box<super::super::types::appplatform::SpringCloudServiceConfigServerGitSettingHttpBasicAuth>>,
    /// The default label of the Git repository, should be the branch name, tag name, or commit-id of the repository.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// One or more `repository` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "repositories")]
    pub r#repositories: Option<Vec<super::super::types::appplatform::SpringCloudServiceConfigServerGitSettingRepository>>,
    /// An array of strings used to search subdirectories of the Git repository.
    #[builder(into)]
    #[serde(rename = "searchPaths")]
    pub r#search_paths: Option<Vec<String>>,
    /// A `ssh_auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "sshAuth")]
    pub r#ssh_auth: Option<Box<super::super::types::appplatform::SpringCloudServiceConfigServerGitSettingSshAuth>>,
    /// The URI of the default Git repository used as the Config Server back end, should be started with `http://`, `https://`, `git@`, or `ssh://`.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpringCloudServiceConfigServerGitSetting {
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
                    "http_basic_auth",
                    &self.r#http_basic_auth,
                ),
                to_pulumi_object_field(
                    "label",
                    &self.r#label,
                ),
                to_pulumi_object_field(
                    "repositories",
                    &self.r#repositories,
                ),
                to_pulumi_object_field(
                    "search_paths",
                    &self.r#search_paths,
                ),
                to_pulumi_object_field(
                    "ssh_auth",
                    &self.r#ssh_auth,
                ),
                to_pulumi_object_field(
                    "uri",
                    &self.r#uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpringCloudServiceConfigServerGitSetting {
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
                    r#http_basic_auth: {
                        let field_value = match fields_map.get("http_basic_auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_basic_auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#label: {
                        let field_value = match fields_map.get("label") {
                            Some(value) => value,
                            None => bail!("Missing field 'label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repositories: {
                        let field_value = match fields_map.get("repositories") {
                            Some(value) => value,
                            None => bail!("Missing field 'repositories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#search_paths: {
                        let field_value = match fields_map.get("search_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'search_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_auth: {
                        let field_value = match fields_map.get("ssh_auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uri: {
                        let field_value = match fields_map.get("uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
