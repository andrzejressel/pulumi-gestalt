#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FluxConfigurationBlobStorage {
    /// Specifies the account key (shared key) to access the storage account.
    #[builder(into)]
    pub r#account_key: Option<String>,
    /// Specifies the Azure Blob container ID.
    #[builder(into)]
    pub r#container_id: String,
    /// Specifies the name of a local secret on the Kubernetes cluster to use as the authentication secret rather than the managed or user-provided configuration secrets.
    #[builder(into)]
    pub r#local_auth_reference: Option<String>,
    /// A `managed_identity` block as defined below.
    #[builder(into)]
    pub r#managed_identity: Option<Box<super::super::types::containerservice::FluxConfigurationBlobStorageManagedIdentity>>,
    /// Specifies the shared access token to access the storage container.
    #[builder(into)]
    pub r#sas_token: Option<String>,
    /// A `service_principal` block as defined below.
    #[builder(into)]
    pub r#service_principal: Option<Box<super::super::types::containerservice::FluxConfigurationBlobStorageServicePrincipal>>,
    /// Specifies the interval at which to re-reconcile the cluster Azure Blob source with the remote.
    #[builder(into)]
    pub r#sync_interval_in_seconds: Option<i32>,
    /// Specifies the maximum time to attempt to reconcile the cluster Azure Blob source with the remote.
    #[builder(into)]
    pub r#timeout_in_seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FluxConfigurationBlobStorage {
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
                    "accountKey",
                    &self.r#account_key,
                ),
                to_pulumi_object_field(
                    "containerId",
                    &self.r#container_id,
                ),
                to_pulumi_object_field(
                    "localAuthReference",
                    &self.r#local_auth_reference,
                ),
                to_pulumi_object_field(
                    "managedIdentity",
                    &self.r#managed_identity,
                ),
                to_pulumi_object_field(
                    "sasToken",
                    &self.r#sas_token,
                ),
                to_pulumi_object_field(
                    "servicePrincipal",
                    &self.r#service_principal,
                ),
                to_pulumi_object_field(
                    "syncIntervalInSeconds",
                    &self.r#sync_interval_in_seconds,
                ),
                to_pulumi_object_field(
                    "timeoutInSeconds",
                    &self.r#timeout_in_seconds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("accountKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'accountKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_id: {
                        let field_value = match fields_map.get("containerId") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_auth_reference: {
                        let field_value = match fields_map.get("localAuthReference") {
                            Some(value) => value,
                            None => bail!("Missing field 'localAuthReference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_identity: {
                        let field_value = match fields_map.get("managedIdentity") {
                            Some(value) => value,
                            None => bail!("Missing field 'managedIdentity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sas_token: {
                        let field_value = match fields_map.get("sasToken") {
                            Some(value) => value,
                            None => bail!("Missing field 'sasToken' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_principal: {
                        let field_value = match fields_map.get("servicePrincipal") {
                            Some(value) => value,
                            None => bail!("Missing field 'servicePrincipal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_interval_in_seconds: {
                        let field_value = match fields_map.get("syncIntervalInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'syncIntervalInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_in_seconds: {
                        let field_value = match fields_map.get("timeoutInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeoutInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
