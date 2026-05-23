#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleServerCustomerManagedKey {
    /// The ID of the geo backup Key Vault Key. It can't cross region and need Customer Managed Key in same region as geo backup.
    #[builder(into)]
    pub r#geo_backup_key_vault_key_id: Option<String>,
    /// The geo backup user managed identity id for a Customer Managed Key. Should be added with `identity_ids`. It can't cross region and need identity in same region as geo backup.
    /// 
    /// > **NOTE:** `primary_user_assigned_identity_id` or `geo_backup_user_assigned_identity_id` is required when `type` is set to `UserAssigned` or `SystemAssigned, UserAssigned`.
    #[builder(into)]
    pub r#geo_backup_user_assigned_identity_id: Option<String>,
    /// The ID of the Key Vault Key.
    #[builder(into)]
    pub r#key_vault_key_id: Option<String>,
    /// Specifies the primary user managed identity id for a Customer Managed Key. Should be added with `identity_ids`.
    #[builder(into)]
    pub r#primary_user_assigned_identity_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlexibleServerCustomerManagedKey {
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
                    "geoBackupKeyVaultKeyId",
                    &self.r#geo_backup_key_vault_key_id,
                ),
                to_pulumi_object_field(
                    "geoBackupUserAssignedIdentityId",
                    &self.r#geo_backup_user_assigned_identity_id,
                ),
                to_pulumi_object_field(
                    "keyVaultKeyId",
                    &self.r#key_vault_key_id,
                ),
                to_pulumi_object_field(
                    "primaryUserAssignedIdentityId",
                    &self.r#primary_user_assigned_identity_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlexibleServerCustomerManagedKey {
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
                    r#geo_backup_key_vault_key_id: {
                        let field_value = match fields_map.get("geoBackupKeyVaultKeyId") {
                            Some(value) => value,
                            None => bail!("Missing field 'geoBackupKeyVaultKeyId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#geo_backup_user_assigned_identity_id: {
                        let field_value = match fields_map.get("geoBackupUserAssignedIdentityId") {
                            Some(value) => value,
                            None => bail!("Missing field 'geoBackupUserAssignedIdentityId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_vault_key_id: {
                        let field_value = match fields_map.get("keyVaultKeyId") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyVaultKeyId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary_user_assigned_identity_id: {
                        let field_value = match fields_map.get("primaryUserAssignedIdentityId") {
                            Some(value) => value,
                            None => bail!("Missing field 'primaryUserAssignedIdentityId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
