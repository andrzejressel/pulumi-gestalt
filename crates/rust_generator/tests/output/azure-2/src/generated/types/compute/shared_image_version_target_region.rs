#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SharedImageVersionTargetRegion {
    /// The ID of the Disk Encryption Set to encrypt the Image Version in the target region. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#disk_encryption_set_id: Option<String>,
    /// Specifies whether this Shared Image Version should be excluded when querying for the `latest` version. Defaults to `false`.
    #[builder(into)]
    pub r#exclude_from_latest_enabled: Option<bool>,
    /// The Azure Region in which this Image Version should exist.
    #[builder(into)]
    pub r#name: String,
    /// The number of replicas of the Image Version to be created per region.
    #[builder(into)]
    pub r#regional_replica_count: i32,
    /// The storage account type for the image version. Possible values are `Standard_LRS`, `Premium_LRS` and `Standard_ZRS`. Defaults to `Standard_LRS`. You can store all of your image version replicas in Zone Redundant Storage by specifying `Standard_ZRS`.
    #[builder(into)]
    pub r#storage_account_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SharedImageVersionTargetRegion {
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
                    "diskEncryptionSetId",
                    &self.r#disk_encryption_set_id,
                ),
                to_pulumi_object_field(
                    "excludeFromLatestEnabled",
                    &self.r#exclude_from_latest_enabled,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "regionalReplicaCount",
                    &self.r#regional_replica_count,
                ),
                to_pulumi_object_field(
                    "storageAccountType",
                    &self.r#storage_account_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SharedImageVersionTargetRegion {
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
                    r#disk_encryption_set_id: {
                        let field_value = match fields_map.get("diskEncryptionSetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskEncryptionSetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclude_from_latest_enabled: {
                        let field_value = match fields_map.get("excludeFromLatestEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'excludeFromLatestEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regional_replica_count: {
                        let field_value = match fields_map.get("regionalReplicaCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'regionalReplicaCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_type: {
                        let field_value = match fields_map.get("storageAccountType") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageAccountType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
