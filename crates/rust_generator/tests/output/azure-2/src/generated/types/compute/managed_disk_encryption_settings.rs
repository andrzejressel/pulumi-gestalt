#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagedDiskEncryptionSettings {
    /// A `disk_encryption_key` block as defined above.
    #[builder(into)]
    #[serde(rename = "diskEncryptionKey")]
    pub r#disk_encryption_key: Box<super::super::types::compute::ManagedDiskEncryptionSettingsDiskEncryptionKey>,
    /// A `key_encryption_key` block as defined below.
    #[builder(into)]
    #[serde(rename = "keyEncryptionKey")]
    pub r#key_encryption_key: Option<Box<super::super::types::compute::ManagedDiskEncryptionSettingsKeyEncryptionKey>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ManagedDiskEncryptionSettings {
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
                "disk_encryption_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disk_encryption_key,
                )
                .await,
            );
            map.insert(
                "key_encryption_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_encryption_key,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ManagedDiskEncryptionSettings {
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
                    r#disk_encryption_key: {
                        let field_value = match fields_map.get("disk_encryption_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_encryption_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_encryption_key: {
                        let field_value = match fields_map.get("key_encryption_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_encryption_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
