#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeGroupEncryption {
    /// The timestamp of the expiration time for the current version of the customer managed key.
    #[builder(into)]
    #[serde(rename = "currentVersionedKeyExpirationTimestamp")]
    pub r#current_versioned_key_expiration_timestamp: Option<String>,
    /// The ID of the current versioned Key Vault Key in use.
    #[builder(into)]
    #[serde(rename = "currentVersionedKeyId")]
    pub r#current_versioned_key_id: Option<String>,
    /// The Key Vault key URI for Customer Managed Key encryption, which can be either a full URI or a versionless URI.
    #[builder(into)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: String,
    /// The timestamp of the last rotation of the Key Vault Key.
    #[builder(into)]
    #[serde(rename = "lastKeyRotationTimestamp")]
    pub r#last_key_rotation_timestamp: Option<String>,
    /// The ID of the User Assigned Identity used by this Elastic SAN Volume Group.
    #[builder(into)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VolumeGroupEncryption {
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
                "current_versioned_key_expiration_timestamp".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#current_versioned_key_expiration_timestamp,
                )
                .await,
            );
            map.insert(
                "current_versioned_key_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#current_versioned_key_id,
                )
                .await,
            );
            map.insert(
                "key_vault_key_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_vault_key_id,
                )
                .await,
            );
            map.insert(
                "last_key_rotation_timestamp".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_key_rotation_timestamp,
                )
                .await,
            );
            map.insert(
                "user_assigned_identity_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_assigned_identity_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VolumeGroupEncryption {
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
                    r#current_versioned_key_expiration_timestamp: {
                        let field_value = match fields_map.get("current_versioned_key_expiration_timestamp") {
                            Some(value) => value,
                            None => bail!("Missing field 'current_versioned_key_expiration_timestamp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#current_versioned_key_id: {
                        let field_value = match fields_map.get("current_versioned_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'current_versioned_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_vault_key_id: {
                        let field_value = match fields_map.get("key_vault_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_vault_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_key_rotation_timestamp: {
                        let field_value = match fields_map.get("last_key_rotation_timestamp") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_key_rotation_timestamp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_assigned_identity_id: {
                        let field_value = match fields_map.get("user_assigned_identity_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_assigned_identity_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
