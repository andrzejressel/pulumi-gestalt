#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupInitContainerVolume {
    /// Boolean as to whether the mounted volume should be an empty directory. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "emptyDir")]
    pub r#empty_dir: Option<bool>,
    /// A `git_repo` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "gitRepo")]
    pub r#git_repo: Option<Box<super::super::types::containerservice::GroupInitContainerVolumeGitRepo>>,
    /// The path on which this volume is to be mounted. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: String,
    /// The name of the volume mount. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specify if the volume is to be mounted as read only or not. The default value is `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Option<bool>,
    /// A map of secrets that will be mounted as files in the volume. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** Exactly one of `empty_dir` volume, `git_repo` volume, `secret` volume or storage account volume (`share_name`, `storage_account_name`, and `storage_account_key`) must be specified.
    /// 
    /// > **Note** when using a storage account volume, all of `share_name`, `storage_account_name`, and `storage_account_key` must be specified.
    /// 
    /// > **Note:** The secret values must be supplied as Base64 encoded strings. The secret values are decoded to their original values when mounted in the volume on the container.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Option<std::collections::HashMap<String, String>>,
    /// The Azure storage share that is to be mounted as a volume. This must be created on the storage account specified as above. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "shareName")]
    pub r#share_name: Option<String>,
    /// The access key for the Azure Storage account specified as above. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageAccountKey")]
    pub r#storage_account_key: Option<String>,
    /// The Azure storage account from which the volume is to be mounted. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageAccountName")]
    pub r#storage_account_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GroupInitContainerVolume {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "empty_dir",
                    &self.r#empty_dir,
                ),
                to_pulumi_object_field(
                    "git_repo",
                    &self.r#git_repo,
                ),
                to_pulumi_object_field(
                    "mount_path",
                    &self.r#mount_path,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "read_only",
                    &self.r#read_only,
                ),
                to_pulumi_object_field(
                    "secret",
                    &self.r#secret,
                ),
                to_pulumi_object_field(
                    "share_name",
                    &self.r#share_name,
                ),
                to_pulumi_object_field(
                    "storage_account_key",
                    &self.r#storage_account_key,
                ),
                to_pulumi_object_field(
                    "storage_account_name",
                    &self.r#storage_account_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GroupInitContainerVolume {
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
                    r#empty_dir: {
                        let field_value = match fields_map.get("empty_dir") {
                            Some(value) => value,
                            None => bail!("Missing field 'empty_dir' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#git_repo: {
                        let field_value = match fields_map.get("git_repo") {
                            Some(value) => value,
                            None => bail!("Missing field 'git_repo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mount_path: {
                        let field_value = match fields_map.get("mount_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'mount_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#read_only: {
                        let field_value = match fields_map.get("read_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret: {
                        let field_value = match fields_map.get("secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#share_name: {
                        let field_value = match fields_map.get("share_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'share_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_key: {
                        let field_value = match fields_map.get("storage_account_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_name: {
                        let field_value = match fields_map.get("storage_account_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
