#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTaskSpecContainerSpecSecret {
    /// Represents the file GID. Defaults to `0`
    #[builder(into)]
    pub r#file_gid: Option<String>,
    /// Represents represents the FileMode of the file. Defaults to `0o444`
    #[builder(into)]
    pub r#file_mode: Option<i32>,
    /// Represents the final filename in the filesystem
    #[builder(into)]
    pub r#file_name: String,
    /// Represents the file UID. Defaults to `0`
    #[builder(into)]
    pub r#file_uid: Option<String>,
    /// ID of the specific secret that we're referencing
    #[builder(into)]
    pub r#secret_id: String,
    /// Name of the secret that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID
    #[builder(into)]
    pub r#secret_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceTaskSpecContainerSpecSecret {
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
                    "fileGid",
                    &self.r#file_gid,
                ),
                to_pulumi_object_field(
                    "fileMode",
                    &self.r#file_mode,
                ),
                to_pulumi_object_field(
                    "fileName",
                    &self.r#file_name,
                ),
                to_pulumi_object_field(
                    "fileUid",
                    &self.r#file_uid,
                ),
                to_pulumi_object_field(
                    "secretId",
                    &self.r#secret_id,
                ),
                to_pulumi_object_field(
                    "secretName",
                    &self.r#secret_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceTaskSpecContainerSpecSecret {
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
                    r#file_gid: {
                        let field_value = match fields_map.get("fileGid") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileGid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_mode: {
                        let field_value = match fields_map.get("fileMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_name: {
                        let field_value = match fields_map.get("fileName") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_uid: {
                        let field_value = match fields_map.get("fileUid") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileUid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_id: {
                        let field_value = match fields_map.get("secretId") {
                            Some(value) => value,
                            None => bail!("Missing field 'secretId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_name: {
                        let field_value = match fields_map.get("secretName") {
                            Some(value) => value,
                            None => bail!("Missing field 'secretName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
