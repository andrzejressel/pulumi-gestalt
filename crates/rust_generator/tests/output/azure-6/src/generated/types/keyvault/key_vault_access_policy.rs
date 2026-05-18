#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KeyVaultAccessPolicy {
    /// The object ID of an Application in Azure Active Directory.
    #[builder(into)]
    #[serde(rename = "applicationId")]
    pub r#application_id: Option<String>,
    /// List of certificate permissions, must be one or more from the following: `Backup`, `Create`, `Delete`, `DeleteIssuers`, `Get`, `GetIssuers`, `Import`, `List`, `ListIssuers`, `ManageContacts`, `ManageIssuers`, `Purge`, `Recover`, `Restore`, `SetIssuers` and `Update`.
    #[builder(into)]
    #[serde(rename = "certificatePermissions")]
    pub r#certificate_permissions: Option<Vec<String>>,
    /// List of key permissions. Possible values are `Backup`, `Create`, `Decrypt`, `Delete`, `Encrypt`, `Get`, `Import`, `List`, `Purge`, `Recover`, `Restore`, `Sign`, `UnwrapKey`, `Update`, `Verify`, `WrapKey`, `Release`, `Rotate`, `GetRotationPolicy` and `SetRotationPolicy`.
    #[builder(into)]
    #[serde(rename = "keyPermissions")]
    pub r#key_permissions: Option<Vec<String>>,
    /// The object ID of a user, service principal or security group in the Azure Active Directory tenant for the vault. The object ID must be unique for the list of access policies.
    #[builder(into)]
    #[serde(rename = "objectId")]
    pub r#object_id: String,
    /// List of secret permissions, must be one or more from the following: `Backup`, `Delete`, `Get`, `List`, `Purge`, `Recover`, `Restore` and `Set`.
    #[builder(into)]
    #[serde(rename = "secretPermissions")]
    pub r#secret_permissions: Option<Vec<String>>,
    /// List of storage permissions, must be one or more from the following: `Backup`, `Delete`, `DeleteSAS`, `Get`, `GetSAS`, `List`, `ListSAS`, `Purge`, `Recover`, `RegenerateKey`, `Restore`, `Set`, `SetSAS` and `Update`.
    #[builder(into)]
    #[serde(rename = "storagePermissions")]
    pub r#storage_permissions: Option<Vec<String>>,
    /// The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault. Must match the `tenant_id` used above.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KeyVaultAccessPolicy {
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
                    "application_id",
                    &self.r#application_id,
                ),
                to_pulumi_object_field(
                    "certificate_permissions",
                    &self.r#certificate_permissions,
                ),
                to_pulumi_object_field(
                    "key_permissions",
                    &self.r#key_permissions,
                ),
                to_pulumi_object_field(
                    "object_id",
                    &self.r#object_id,
                ),
                to_pulumi_object_field(
                    "secret_permissions",
                    &self.r#secret_permissions,
                ),
                to_pulumi_object_field(
                    "storage_permissions",
                    &self.r#storage_permissions,
                ),
                to_pulumi_object_field(
                    "tenant_id",
                    &self.r#tenant_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KeyVaultAccessPolicy {
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
                    r#application_id: {
                        let field_value = match fields_map.get("application_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate_permissions: {
                        let field_value = match fields_map.get("certificate_permissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_permissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_permissions: {
                        let field_value = match fields_map.get("key_permissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_permissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#object_id: {
                        let field_value = match fields_map.get("object_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'object_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_permissions: {
                        let field_value = match fields_map.get("secret_permissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_permissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_permissions: {
                        let field_value = match fields_map.get("storage_permissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_permissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tenant_id: {
                        let field_value = match fields_map.get("tenant_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'tenant_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
