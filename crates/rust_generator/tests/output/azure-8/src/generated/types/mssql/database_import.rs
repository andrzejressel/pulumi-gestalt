#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatabaseImport {
    /// Specifies the name of the SQL administrator.
    #[builder(into)]
    pub r#administrator_login: String,
    /// Specifies the password of the SQL administrator.
    #[builder(into)]
    pub r#administrator_login_password: String,
    /// Specifies the type of authentication used to access the server. Valid values are `SQL` or `ADPassword`.
    #[builder(into)]
    pub r#authentication_type: String,
    /// The resource id for the storage account used to store BACPAC file. If set, private endpoint connection will be created for the storage account. Must match storage account used for storage_uri parameter.
    #[builder(into)]
    pub r#storage_account_id: Option<String>,
    /// Specifies the access key for the storage account.
    #[builder(into)]
    pub r#storage_key: String,
    /// Specifies the type of access key for the storage account. Valid values are `StorageAccessKey` or `SharedAccessKey`.
    #[builder(into)]
    pub r#storage_key_type: String,
    /// Specifies the blob URI of the .bacpac file.
    #[builder(into)]
    pub r#storage_uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatabaseImport {
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
                    "administratorLogin",
                    &self.r#administrator_login,
                ),
                to_pulumi_object_field(
                    "administratorLoginPassword",
                    &self.r#administrator_login_password,
                ),
                to_pulumi_object_field(
                    "authenticationType",
                    &self.r#authentication_type,
                ),
                to_pulumi_object_field(
                    "storageAccountId",
                    &self.r#storage_account_id,
                ),
                to_pulumi_object_field(
                    "storageKey",
                    &self.r#storage_key,
                ),
                to_pulumi_object_field(
                    "storageKeyType",
                    &self.r#storage_key_type,
                ),
                to_pulumi_object_field(
                    "storageUri",
                    &self.r#storage_uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatabaseImport {
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
                    r#administrator_login: {
                        let field_value = match fields_map.get("administratorLogin") {
                            Some(value) => value,
                            None => bail!("Missing field 'administratorLogin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#administrator_login_password: {
                        let field_value = match fields_map.get("administratorLoginPassword") {
                            Some(value) => value,
                            None => bail!("Missing field 'administratorLoginPassword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authentication_type: {
                        let field_value = match fields_map.get("authenticationType") {
                            Some(value) => value,
                            None => bail!("Missing field 'authenticationType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_id: {
                        let field_value = match fields_map.get("storageAccountId") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageAccountId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_key: {
                        let field_value = match fields_map.get("storageKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_key_type: {
                        let field_value = match fields_map.get("storageKeyType") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageKeyType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_uri: {
                        let field_value = match fields_map.get("storageUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
