#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInstanceBootDiskInitializeParam {
    /// A flag to enable confidential compute mode on boot disk
    #[builder(into)]
    #[serde(rename = "enableConfidentialCompute")]
    pub r#enable_confidential_compute: bool,
    /// The image from which this disk was initialised.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// A set of key/value label pairs assigned to the disk.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// Indicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle.
    #[builder(into)]
    #[serde(rename = "provisionedIops")]
    pub r#provisioned_iops: i32,
    /// Indicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle.
    #[builder(into)]
    #[serde(rename = "provisionedThroughput")]
    pub r#provisioned_throughput: i32,
    /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty.
    #[builder(into)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: std::collections::HashMap<String, String>,
    /// A list of self_links to resource policies attached to the selected `boot_disk`
    #[builder(into)]
    #[serde(rename = "resourcePolicies")]
    pub r#resource_policies: Vec<String>,
    /// The size of the image in gigabytes.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: i32,
    /// The URL of the storage pool in which the new disk is created
    #[builder(into)]
    #[serde(rename = "storagePool")]
    pub r#storage_pool: String,
    /// The accelerator type resource exposed to this instance. E.g. `nvidia-tesla-k80`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetInstanceBootDiskInitializeParam {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "enable_confidential_compute".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_confidential_compute,
                )
                .await,
            );
            map.insert(
                "image".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image,
                )
                .await,
            );
            map.insert(
                "labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#labels,
                )
                .await,
            );
            map.insert(
                "provisioned_iops".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#provisioned_iops,
                )
                .await,
            );
            map.insert(
                "provisioned_throughput".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#provisioned_throughput,
                )
                .await,
            );
            map.insert(
                "resource_manager_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_manager_tags,
                )
                .await,
            );
            map.insert(
                "resource_policies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_policies,
                )
                .await,
            );
            map.insert(
                "size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#size,
                )
                .await,
            );
            map.insert(
                "storage_pool".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_pool,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetInstanceBootDiskInitializeParam {
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
                    r#enable_confidential_compute: {
                        let field_value = match fields_map.get("enable_confidential_compute") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_confidential_compute' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image: {
                        let field_value = match fields_map.get("image") {
                            Some(value) => value,
                            None => bail!("Missing field 'image' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioned_iops: {
                        let field_value = match fields_map.get("provisioned_iops") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioned_iops' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioned_throughput: {
                        let field_value = match fields_map.get("provisioned_throughput") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioned_throughput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_manager_tags: {
                        let field_value = match fields_map.get("resource_manager_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_manager_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_policies: {
                        let field_value = match fields_map.get("resource_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#size: {
                        let field_value = match fields_map.get("size") {
                            Some(value) => value,
                            None => bail!("Missing field 'size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_pool: {
                        let field_value = match fields_map.get("storage_pool") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_pool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
