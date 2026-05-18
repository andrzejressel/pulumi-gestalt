#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionEksPropertyPodPropertyContainerSecurityContext {
    /// When this parameter is true, the container is given elevated permissions on the host container instance (similar to the root user).
    #[builder(into)]
    #[serde(rename = "privileged")]
    pub r#privileged: bool,
    #[builder(into)]
    #[serde(rename = "readOnlyRootFileSystem")]
    pub r#read_only_root_file_system: bool,
    /// When this parameter is specified, the container is run as the specified group ID (gid). If this parameter isn't specified, the default is the group that's specified in the image metadata.
    #[builder(into)]
    #[serde(rename = "runAsGroup")]
    pub r#run_as_group: i32,
    /// When this parameter is specified, the container is run as a user with a uid other than 0. If this parameter isn't specified, so such rule is enforced.
    #[builder(into)]
    #[serde(rename = "runAsNonRoot")]
    pub r#run_as_non_root: bool,
    /// When this parameter is specified, the container is run as the specified user ID (uid). If this parameter isn't specified, the default is the user that's specified in the image metadata.
    #[builder(into)]
    #[serde(rename = "runAsUser")]
    pub r#run_as_user: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobDefinitionEksPropertyPodPropertyContainerSecurityContext {
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
                    "privileged",
                    &self.r#privileged,
                ),
                to_pulumi_object_field(
                    "read_only_root_file_system",
                    &self.r#read_only_root_file_system,
                ),
                to_pulumi_object_field(
                    "run_as_group",
                    &self.r#run_as_group,
                ),
                to_pulumi_object_field(
                    "run_as_non_root",
                    &self.r#run_as_non_root,
                ),
                to_pulumi_object_field(
                    "run_as_user",
                    &self.r#run_as_user,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobDefinitionEksPropertyPodPropertyContainerSecurityContext {
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
                    r#privileged: {
                        let field_value = match fields_map.get("privileged") {
                            Some(value) => value,
                            None => bail!("Missing field 'privileged' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_only_root_file_system: {
                        let field_value = match fields_map.get("read_only_root_file_system") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_only_root_file_system' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#run_as_group: {
                        let field_value = match fields_map.get("run_as_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'run_as_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#run_as_non_root: {
                        let field_value = match fields_map.get("run_as_non_root") {
                            Some(value) => value,
                            None => bail!("Missing field 'run_as_non_root' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#run_as_user: {
                        let field_value = match fields_map.get("run_as_user") {
                            Some(value) => value,
                            None => bail!("Missing field 'run_as_user' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
