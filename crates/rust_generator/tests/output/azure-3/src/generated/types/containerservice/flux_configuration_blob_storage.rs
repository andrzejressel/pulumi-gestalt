#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FluxConfigurationBlobStorage {
    /// Specifies the account key (shared key) to access the storage account.
    #[builder(into)]
    #[serde(rename = "accountKey")]
    pub r#account_key: Option<String>,
    /// Specifies the Azure Blob container ID.
    #[builder(into)]
    #[serde(rename = "containerId")]
    pub r#container_id: String,
    /// Specifies the name of a local secret on the Kubernetes cluster to use as the authentication secret rather than the managed or user-provided configuration secrets.
    #[builder(into)]
    #[serde(rename = "localAuthReference")]
    pub r#local_auth_reference: Option<String>,
    /// A `managed_identity` block as defined below.
    #[builder(into)]
    #[serde(rename = "managedIdentity")]
    pub r#managed_identity: Option<Box<super::super::types::containerservice::FluxConfigurationBlobStorageManagedIdentity>>,
    /// Specifies the shared access token to access the storage container.
    #[builder(into)]
    #[serde(rename = "sasToken")]
    pub r#sas_token: Option<String>,
    /// A `service_principal` block as defined below.
    #[builder(into)]
    #[serde(rename = "servicePrincipal")]
    pub r#service_principal: Option<Box<super::super::types::containerservice::FluxConfigurationBlobStorageServicePrincipal>>,
    /// Specifies the interval at which to re-reconcile the cluster Azure Blob source with the remote.
    #[builder(into)]
    #[serde(rename = "syncIntervalInSeconds")]
    pub r#sync_interval_in_seconds: Option<i32>,
    /// Specifies the maximum time to attempt to reconcile the cluster Azure Blob source with the remote.
    #[builder(into)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FluxConfigurationBlobStorage {
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
                    "account_key",
                    &self.r#account_key,
                ),
                to_pulumi_object_field(
                    "container_id",
                    &self.r#container_id,
                ),
                to_pulumi_object_field(
                    "local_auth_reference",
                    &self.r#local_auth_reference,
                ),
                to_pulumi_object_field(
                    "managed_identity",
                    &self.r#managed_identity,
                ),
                to_pulumi_object_field(
                    "sas_token",
                    &self.r#sas_token,
                ),
                to_pulumi_object_field(
                    "service_principal",
                    &self.r#service_principal,
                ),
                to_pulumi_object_field(
                    "sync_interval_in_seconds",
                    &self.r#sync_interval_in_seconds,
                ),
                to_pulumi_object_field(
                    "timeout_in_seconds",
                    &self.r#timeout_in_seconds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FluxConfigurationBlobStorage {
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
                    r#account_key: {
                        let field_value = match fields_map.get("account_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_id: {
                        let field_value = match fields_map.get("container_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_auth_reference: {
                        let field_value = match fields_map.get("local_auth_reference") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_auth_reference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_identity: {
                        let field_value = match fields_map.get("managed_identity") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_identity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sas_token: {
                        let field_value = match fields_map.get("sas_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'sas_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_principal: {
                        let field_value = match fields_map.get("service_principal") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_principal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_interval_in_seconds: {
                        let field_value = match fields_map.get("sync_interval_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'sync_interval_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_in_seconds: {
                        let field_value = match fields_map.get("timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
