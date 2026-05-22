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
                    "applicationId",
                    &self.r#application_id,
                ),
                to_pulumi_object_field(
                    "certificatePermissions",
                    &self.r#certificate_permissions,
                ),
                to_pulumi_object_field(
                    "keyPermissions",
                    &self.r#key_permissions,
                ),
                to_pulumi_object_field(
                    "objectId",
                    &self.r#object_id,
                ),
                to_pulumi_object_field(
                    "secretPermissions",
                    &self.r#secret_permissions,
                ),
                to_pulumi_object_field(
                    "storagePermissions",
                    &self.r#storage_permissions,
                ),
                to_pulumi_object_field(
                    "tenantId",
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
                        let field_value = match fields_map.get("applicationId") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate_permissions: {
                        let field_value = match fields_map.get("certificatePermissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificatePermissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_permissions: {
                        let field_value = match fields_map.get("keyPermissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyPermissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#object_id: {
                        let field_value = match fields_map.get("objectId") {
                            Some(value) => value,
                            None => bail!("Missing field 'objectId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_permissions: {
                        let field_value = match fields_map.get("secretPermissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'secretPermissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_permissions: {
                        let field_value = match fields_map.get("storagePermissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'storagePermissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tenant_id: {
                        let field_value = match fields_map.get("tenantId") {
                            Some(value) => value,
                            None => bail!("Missing field 'tenantId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
