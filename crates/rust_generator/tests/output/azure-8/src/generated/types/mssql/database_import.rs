#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatabaseImport {
    /// Specifies the name of the SQL administrator.
    #[builder(into)]
    #[serde(rename = "administratorLogin")]
    pub r#administrator_login: String,
    /// Specifies the password of the SQL administrator.
    #[builder(into)]
    #[serde(rename = "administratorLoginPassword")]
    pub r#administrator_login_password: String,
    /// Specifies the type of authentication used to access the server. Valid values are `SQL` or `ADPassword`.
    #[builder(into)]
    #[serde(rename = "authenticationType")]
    pub r#authentication_type: String,
    /// The resource id for the storage account used to store BACPAC file. If set, private endpoint connection will be created for the storage account. Must match storage account used for storage_uri parameter.
    #[builder(into)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: Option<String>,
    /// Specifies the access key for the storage account.
    #[builder(into)]
    #[serde(rename = "storageKey")]
    pub r#storage_key: String,
    /// Specifies the type of access key for the storage account. Valid values are `StorageAccessKey` or `SharedAccessKey`.
    #[builder(into)]
    #[serde(rename = "storageKeyType")]
    pub r#storage_key_type: String,
    /// Specifies the blob URI of the .bacpac file.
    #[builder(into)]
    #[serde(rename = "storageUri")]
    pub r#storage_uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatabaseImport {
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
                "administrator_login".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#administrator_login,
                )
                .await,
            );
            map.insert(
                "administrator_login_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#administrator_login_password,
                )
                .await,
            );
            map.insert(
                "authentication_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authentication_type,
                )
                .await,
            );
            map.insert(
                "storage_account_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_account_id,
                )
                .await,
            );
            map.insert(
                "storage_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_key,
                )
                .await,
            );
            map.insert(
                "storage_key_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_key_type,
                )
                .await,
            );
            map.insert(
                "storage_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_uri,
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
                        let field_value = match fields_map.get("administrator_login") {
                            Some(value) => value,
                            None => bail!("Missing field 'administrator_login' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#administrator_login_password: {
                        let field_value = match fields_map.get("administrator_login_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'administrator_login_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authentication_type: {
                        let field_value = match fields_map.get("authentication_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_id: {
                        let field_value = match fields_map.get("storage_account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_key: {
                        let field_value = match fields_map.get("storage_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_key_type: {
                        let field_value = match fields_map.get("storage_key_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_key_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_uri: {
                        let field_value = match fields_map.get("storage_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
