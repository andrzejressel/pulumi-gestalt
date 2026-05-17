#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudAppCustomPersistentDisk {
    /// These are the mount options for a persistent disk.
    #[builder(into)]
    #[serde(rename = "mountOptions")]
    pub r#mount_options: Option<Vec<String>>,
    /// The mount path of the persistent disk.
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: String,
    /// Indicates whether the persistent disk is a readOnly one.
    #[builder(into)]
    #[serde(rename = "readOnlyEnabled")]
    pub r#read_only_enabled: Option<bool>,
    /// The share name of the Azure File share.
    #[builder(into)]
    #[serde(rename = "shareName")]
    pub r#share_name: String,
    /// The name of the Spring Cloud Storage.
    #[builder(into)]
    #[serde(rename = "storageName")]
    pub r#storage_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpringCloudAppCustomPersistentDisk {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "mount_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mount_options,
                )
                .await,
            );
            map.insert(
                "mount_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mount_path,
                )
                .await,
            );
            map.insert(
                "read_only_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#read_only_enabled,
                )
                .await,
            );
            map.insert(
                "share_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#share_name,
                )
                .await,
            );
            map.insert(
                "storage_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_name,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpringCloudAppCustomPersistentDisk {
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
                    r#mount_options: {
                        let field_value = match fields_map.get("mount_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'mount_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#read_only_enabled: {
                        let field_value = match fields_map.get("read_only_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_only_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#storage_name: {
                        let field_value = match fields_map.get("storage_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
