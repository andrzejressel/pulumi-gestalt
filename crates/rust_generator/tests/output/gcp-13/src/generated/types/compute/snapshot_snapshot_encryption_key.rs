#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SnapshotSnapshotEncryptionKey {
    /// The name of the encryption key that is stored in Google Cloud KMS.
    #[builder(into)]
    #[serde(rename = "kmsKeySelfLink")]
    pub r#kms_key_self_link: Option<String>,
    /// The service account used for the encryption request for the given KMS key.
    /// If absent, the Compute Engine Service Agent service account is used.
    #[builder(into)]
    #[serde(rename = "kmsKeyServiceAccount")]
    pub r#kms_key_service_account: Option<String>,
    /// Specifies a 256-bit customer-supplied encryption key, encoded in
    /// RFC 4648 base64 to either encrypt or decrypt this resource.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "rawKey")]
    pub r#raw_key: Option<String>,
    /// (Output)
    /// The RFC 4648 base64 encoded SHA-256 hash of the customer-supplied
    /// encryption key that protects this resource.
    #[builder(into)]
    #[serde(rename = "sha256")]
    pub r#sha_256: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SnapshotSnapshotEncryptionKey {
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
                "kms_key_self_link".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kms_key_self_link,
                )
                .await,
            );
            map.insert(
                "kms_key_service_account".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kms_key_service_account,
                )
                .await,
            );
            map.insert(
                "raw_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#raw_key,
                )
                .await,
            );
            map.insert(
                "sha_256".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sha_256,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SnapshotSnapshotEncryptionKey {
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
                    r#kms_key_self_link: {
                        let field_value = match fields_map.get("kms_key_self_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_self_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_service_account: {
                        let field_value = match fields_map.get("kms_key_service_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_service_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#raw_key: {
                        let field_value = match fields_map.get("raw_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'raw_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sha_256: {
                        let field_value = match fields_map.get("sha_256") {
                            Some(value) => value,
                            None => bail!("Missing field 'sha_256' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
