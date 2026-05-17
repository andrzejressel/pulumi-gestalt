#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudCustomizedAcceleratorGitRepository {
    /// A `basic_auth` block as defined below. Conflicts with `git_repository[0].ssh_auth`. Changing this forces a new Spring Cloud Customized Accelerator to be created.
    #[builder(into)]
    #[serde(rename = "basicAuth")]
    pub r#basic_auth: Option<Box<super::super::types::appplatform::SpringCloudCustomizedAcceleratorGitRepositoryBasicAuth>>,
    /// Specifies the Git repository branch to be used.
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: Option<String>,
    /// Specifies the ID of the CA Spring Cloud Certificate for https URL of Git repository.
    #[builder(into)]
    #[serde(rename = "caCertificateId")]
    pub r#ca_certificate_id: Option<String>,
    /// Specifies the Git repository commit to be used.
    #[builder(into)]
    #[serde(rename = "commit")]
    pub r#commit: Option<String>,
    /// Specifies the Git repository tag to be used.
    #[builder(into)]
    #[serde(rename = "gitTag")]
    pub r#git_tag: Option<String>,
    /// Specifies the interval for checking for updates to Git or image repository. It should be greater than 10.
    #[builder(into)]
    #[serde(rename = "intervalInSeconds")]
    pub r#interval_in_seconds: Option<i32>,
    /// Specifies the path under the git repository to be treated as the root directory of the accelerator or the fragment (depending on `accelerator_type`).
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// A `ssh_auth` block as defined below. Conflicts with `git_repository[0].basic_auth`. Changing this forces a new Spring Cloud Customized Accelerator to be created.
    #[builder(into)]
    #[serde(rename = "sshAuth")]
    pub r#ssh_auth: Option<Box<super::super::types::appplatform::SpringCloudCustomizedAcceleratorGitRepositorySshAuth>>,
    /// Specifies Git repository URL for the accelerator.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpringCloudCustomizedAcceleratorGitRepository {
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
                    "basic_auth",
                    &self.r#basic_auth,
                ),
                to_pulumi_object_field(
                    "branch",
                    &self.r#branch,
                ),
                to_pulumi_object_field(
                    "ca_certificate_id",
                    &self.r#ca_certificate_id,
                ),
                to_pulumi_object_field(
                    "commit",
                    &self.r#commit,
                ),
                to_pulumi_object_field(
                    "git_tag",
                    &self.r#git_tag,
                ),
                to_pulumi_object_field(
                    "interval_in_seconds",
                    &self.r#interval_in_seconds,
                ),
                to_pulumi_object_field(
                    "path",
                    &self.r#path,
                ),
                to_pulumi_object_field(
                    "ssh_auth",
                    &self.r#ssh_auth,
                ),
                to_pulumi_object_field(
                    "url",
                    &self.r#url,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpringCloudCustomizedAcceleratorGitRepository {
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
                    r#basic_auth: {
                        let field_value = match fields_map.get("basic_auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'basic_auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ca_certificate_id: {
                        let field_value = match fields_map.get("ca_certificate_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'ca_certificate_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#commit: {
                        let field_value = match fields_map.get("commit") {
                            Some(value) => value,
                            None => bail!("Missing field 'commit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#git_tag: {
                        let field_value = match fields_map.get("git_tag") {
                            Some(value) => value,
                            None => bail!("Missing field 'git_tag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interval_in_seconds: {
                        let field_value = match fields_map.get("interval_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'interval_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#url: {
                        let field_value = match fields_map.get("url") {
                            Some(value) => value,
                            None => bail!("Missing field 'url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
