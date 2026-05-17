#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEndpointRedshiftSetting {
    #[builder(into)]
    #[serde(rename = "bucketFolder")]
    pub r#bucket_folder: String,
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: String,
    #[builder(into)]
    #[serde(rename = "encryptionMode")]
    pub r#encryption_mode: String,
    #[builder(into)]
    #[serde(rename = "serverSideEncryptionKmsKeyId")]
    pub r#server_side_encryption_kms_key_id: String,
    #[builder(into)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetEndpointRedshiftSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "bucket_folder".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bucket_folder,
                )
                .await,
            );
            map.insert(
                "bucket_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bucket_name,
                )
                .await,
            );
            map.insert(
                "encryption_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_mode,
                )
                .await,
            );
            map.insert(
                "server_side_encryption_kms_key_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#server_side_encryption_kms_key_id,
                )
                .await,
            );
            map.insert(
                "service_access_role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_access_role_arn,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetEndpointRedshiftSetting {
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
                    r#bucket_folder: {
                        let field_value = match fields_map.get("bucket_folder") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_folder' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_name: {
                        let field_value = match fields_map.get("bucket_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_mode: {
                        let field_value = match fields_map.get("encryption_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_side_encryption_kms_key_id: {
                        let field_value = match fields_map.get("server_side_encryption_kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_side_encryption_kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_access_role_arn: {
                        let field_value = match fields_map.get("service_access_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_access_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
