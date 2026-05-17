#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InteractiveQueryClusterStorageAccountGen2 {
    /// The ID of the Gen2 Filesystem. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "filesystemId")]
    pub r#filesystem_id: String,
    /// Is this the Default Storage Account for the HDInsight Hadoop Cluster? Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** One of the `storage_account` or `storage_account_gen2` blocks must be marked as the default.
    #[builder(into)]
    #[serde(rename = "isDefault")]
    pub r#is_default: bool,
    /// The ID of Managed Identity to use for accessing the Gen2 filesystem. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** This can be obtained from the `id` of the `azure.storage.Container` resource.
    #[builder(into)]
    #[serde(rename = "managedIdentityResourceId")]
    pub r#managed_identity_resource_id: String,
    /// The ID of the Storage Account. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageResourceId")]
    pub r#storage_resource_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InteractiveQueryClusterStorageAccountGen2 {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "filesystem_id",
                    &self.r#filesystem_id,
                ),
                to_pulumi_object_field(
                    "is_default",
                    &self.r#is_default,
                ),
                to_pulumi_object_field(
                    "managed_identity_resource_id",
                    &self.r#managed_identity_resource_id,
                ),
                to_pulumi_object_field(
                    "storage_resource_id",
                    &self.r#storage_resource_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InteractiveQueryClusterStorageAccountGen2 {
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
                    r#filesystem_id: {
                        let field_value = match fields_map.get("filesystem_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'filesystem_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_default: {
                        let field_value = match fields_map.get("is_default") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_default' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_identity_resource_id: {
                        let field_value = match fields_map.get("managed_identity_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_identity_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_resource_id: {
                        let field_value = match fields_map.get("storage_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
