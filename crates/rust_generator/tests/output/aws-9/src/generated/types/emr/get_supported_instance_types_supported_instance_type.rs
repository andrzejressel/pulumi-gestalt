#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSupportedInstanceTypesSupportedInstanceType {
    /// CPU architecture.
    #[builder(into)]
    #[serde(rename = "architecture")]
    pub r#architecture: String,
    /// Indicates whether the instance type supports Amazon EBS optimization.
    #[builder(into)]
    #[serde(rename = "ebsOptimizedAvailable")]
    pub r#ebs_optimized_available: bool,
    /// Indicates whether the instance type uses Amazon EBS optimization by default.
    #[builder(into)]
    #[serde(rename = "ebsOptimizedByDefault")]
    pub r#ebs_optimized_by_default: bool,
    /// Indicates whether the instance type only supports Amazon EBS.
    #[builder(into)]
    #[serde(rename = "ebsStorageOnly")]
    pub r#ebs_storage_only: bool,
    /// The Amazon EC2 family and generation for the instance type.
    #[builder(into)]
    #[serde(rename = "instanceFamilyId")]
    pub r#instance_family_id: String,
    /// Indicates whether the instance type only supports 64-bit architecture.
    #[builder(into)]
    #[serde(rename = "is64BitsOnly")]
    pub r#is_64_bits_only: bool,
    /// Memory that is available to Amazon EMR from the instance type.
    #[builder(into)]
    #[serde(rename = "memoryGb")]
    pub r#memory_gb: f64,
    /// Number of disks for the instance type.
    #[builder(into)]
    #[serde(rename = "numberOfDisks")]
    pub r#number_of_disks: i32,
    /// Storage capacity of the instance type.
    #[builder(into)]
    #[serde(rename = "storageGb")]
    pub r#storage_gb: i32,
    /// Amazon EC2 instance type. For example, `m5.xlarge`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// The number of vCPUs available for the instance type.
    #[builder(into)]
    #[serde(rename = "vcpu")]
    pub r#vcpu: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSupportedInstanceTypesSupportedInstanceType {
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
                "architecture".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#architecture,
                )
                .await,
            );
            map.insert(
                "ebs_optimized_available".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ebs_optimized_available,
                )
                .await,
            );
            map.insert(
                "ebs_optimized_by_default".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ebs_optimized_by_default,
                )
                .await,
            );
            map.insert(
                "ebs_storage_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ebs_storage_only,
                )
                .await,
            );
            map.insert(
                "instance_family_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_family_id,
                )
                .await,
            );
            map.insert(
                "is_64_bits_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_64_bits_only,
                )
                .await,
            );
            map.insert(
                "memory_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#memory_gb,
                )
                .await,
            );
            map.insert(
                "number_of_disks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#number_of_disks,
                )
                .await,
            );
            map.insert(
                "storage_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_gb,
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
            map.insert(
                "vcpu".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vcpu,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSupportedInstanceTypesSupportedInstanceType {
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
                    r#architecture: {
                        let field_value = match fields_map.get("architecture") {
                            Some(value) => value,
                            None => bail!("Missing field 'architecture' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebs_optimized_available: {
                        let field_value = match fields_map.get("ebs_optimized_available") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebs_optimized_available' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebs_optimized_by_default: {
                        let field_value = match fields_map.get("ebs_optimized_by_default") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebs_optimized_by_default' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebs_storage_only: {
                        let field_value = match fields_map.get("ebs_storage_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebs_storage_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_family_id: {
                        let field_value = match fields_map.get("instance_family_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_family_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_64_bits_only: {
                        let field_value = match fields_map.get("is_64_bits_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_64_bits_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_gb: {
                        let field_value = match fields_map.get("memory_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_of_disks: {
                        let field_value = match fields_map.get("number_of_disks") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_of_disks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_gb: {
                        let field_value = match fields_map.get("storage_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#vcpu: {
                        let field_value = match fields_map.get("vcpu") {
                            Some(value) => value,
                            None => bail!("Missing field 'vcpu' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
