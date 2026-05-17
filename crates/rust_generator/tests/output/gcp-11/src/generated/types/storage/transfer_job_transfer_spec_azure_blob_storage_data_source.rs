#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TransferJobTransferSpecAzureBlobStorageDataSource {
    /// Credentials used to authenticate API requests to Azure block.
    #[builder(into)]
    #[serde(rename = "azureCredentials")]
    pub r#azure_credentials: Option<Box<super::super::types::storage::TransferJobTransferSpecAzureBlobStorageDataSourceAzureCredentials>>,
    /// The container to transfer from the Azure Storage account.`
    #[builder(into)]
    #[serde(rename = "container")]
    pub r#container: String,
    /// Full Resource name of a secret in Secret Manager containing [SAS Credentials in JSON form](https://cloud.google.com/storage-transfer/docs/reference/rest/v1/TransferSpec#azureblobstoragedata:~:text=begin%20with%20a%20%27/%27.-,credentialsSecret,-string). Service Agent for Storage Transfer must have permissions to access secret. If credentials_secret is specified, do not specify azure_credentials.`,
    #[builder(into)]
    #[serde(rename = "credentialsSecret")]
    pub r#credentials_secret: Option<String>,
    /// Root path to transfer objects. Must be an empty string or full path name that ends with a '/'. This field is treated as an object prefix. As such, it should generally not begin with a '/'.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// The name of the Azure Storage account.
    #[builder(into)]
    #[serde(rename = "storageAccount")]
    pub r#storage_account: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TransferJobTransferSpecAzureBlobStorageDataSource {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "azure_credentials",
                    &self.r#azure_credentials,
                ),
                to_pulumi_object_field(
                    "container",
                    &self.r#container,
                ),
                to_pulumi_object_field(
                    "credentials_secret",
                    &self.r#credentials_secret,
                ),
                to_pulumi_object_field(
                    "path",
                    &self.r#path,
                ),
                to_pulumi_object_field(
                    "storage_account",
                    &self.r#storage_account,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TransferJobTransferSpecAzureBlobStorageDataSource {
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
                    r#azure_credentials: {
                        let field_value = match fields_map.get("azure_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container: {
                        let field_value = match fields_map.get("container") {
                            Some(value) => value,
                            None => bail!("Missing field 'container' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#credentials_secret: {
                        let field_value = match fields_map.get("credentials_secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'credentials_secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account: {
                        let field_value = match fields_map.get("storage_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
