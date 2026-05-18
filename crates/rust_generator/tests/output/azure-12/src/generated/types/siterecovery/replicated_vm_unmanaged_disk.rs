#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicatedVmUnmanagedDisk {
    /// Id of disk that should be replicated. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "diskUri")]
    pub r#disk_uri: String,
    /// Storage account that should be used for caching. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "stagingStorageAccountId")]
    pub r#staging_storage_account_id: String,
    /// Storage account disk should belong to when a failover is done. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "targetStorageAccountId")]
    pub r#target_storage_account_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReplicatedVmUnmanagedDisk {
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
                    "disk_uri",
                    &self.r#disk_uri,
                ),
                to_pulumi_object_field(
                    "staging_storage_account_id",
                    &self.r#staging_storage_account_id,
                ),
                to_pulumi_object_field(
                    "target_storage_account_id",
                    &self.r#target_storage_account_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReplicatedVmUnmanagedDisk {
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
                    r#disk_uri: {
                        let field_value = match fields_map.get("disk_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#staging_storage_account_id: {
                        let field_value = match fields_map.get("staging_storage_account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'staging_storage_account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_storage_account_id: {
                        let field_value = match fields_map.get("target_storage_account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_storage_account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
