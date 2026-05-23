#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPoolMountAzureFileShare {
    /// The Azure Storage Account key.
    #[builder(into)]
    pub r#account_key: String,
    /// The Azure Storage Account name.
    #[builder(into)]
    pub r#account_name: String,
    /// The Azure Files URL. This is of the form 'https://{account}.file.core.windows.net/'.
    #[builder(into)]
    pub r#azure_file_url: String,
    /// Additional command line options to pass to the mount command. These are 'net use' options in Windows and 'mount' options in Linux.
    #[builder(into)]
    pub r#mount_options: String,
    /// The relative path on compute node where the file system will be mounted All file systems are mounted relative to the Batch mounts directory, accessible via the `AZ_BATCH_NODE_MOUNTS_DIR` environment variable.
    #[builder(into)]
    pub r#relative_mount_path: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPoolMountAzureFileShare {
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
                    "accountKey",
                    &self.r#account_key,
                ),
                to_pulumi_object_field(
                    "accountName",
                    &self.r#account_name,
                ),
                to_pulumi_object_field(
                    "azureFileUrl",
                    &self.r#azure_file_url,
                ),
                to_pulumi_object_field(
                    "mountOptions",
                    &self.r#mount_options,
                ),
                to_pulumi_object_field(
                    "relativeMountPath",
                    &self.r#relative_mount_path,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPoolMountAzureFileShare {
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
                    r#account_key: {
                        let field_value = match fields_map.get("accountKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'accountKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#account_name: {
                        let field_value = match fields_map.get("accountName") {
                            Some(value) => value,
                            None => bail!("Missing field 'accountName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#azure_file_url: {
                        let field_value = match fields_map.get("azureFileUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'azureFileUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mount_options: {
                        let field_value = match fields_map.get("mountOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'mountOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#relative_mount_path: {
                        let field_value = match fields_map.get("relativeMountPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'relativeMountPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
