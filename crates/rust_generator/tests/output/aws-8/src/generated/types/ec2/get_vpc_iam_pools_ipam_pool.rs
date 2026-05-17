#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVpcIamPoolsIpamPool {
    /// IP protocol assigned to this pool.
    #[builder(into)]
    #[serde(rename = "addressFamily")]
    pub r#address_family: String,
    /// A default netmask length for allocations added to this pool. If, for example, the CIDR assigned to this pool is `10.0.0.0/8` and you enter 16 here, new allocations will default to `10.0.0.0/16`.
    #[builder(into)]
    #[serde(rename = "allocationDefaultNetmaskLength")]
    pub r#allocation_default_netmask_length: i32,
    /// The maximum netmask length that will be required for CIDR allocations in this pool.
    #[builder(into)]
    #[serde(rename = "allocationMaxNetmaskLength")]
    pub r#allocation_max_netmask_length: i32,
    /// The minimum netmask length that will be required for CIDR allocations in this pool.
    #[builder(into)]
    #[serde(rename = "allocationMinNetmaskLength")]
    pub r#allocation_min_netmask_length: i32,
    /// Tags that are required to create resources in using this pool.
    #[builder(into)]
    #[serde(rename = "allocationResourceTags")]
    pub r#allocation_resource_tags: std::collections::HashMap<String, String>,
    /// ARN of the pool
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// If enabled, IPAM will continuously look for resources within the CIDR range of this pool and automatically import them as allocations into your IPAM.
    #[builder(into)]
    #[serde(rename = "autoImport")]
    pub r#auto_import: bool,
    /// Limits which service in AWS that the pool can be used in. `ec2` for example, allows users to use space for Elastic IP addresses and VPCs.
    #[builder(into)]
    #[serde(rename = "awsService")]
    pub r#aws_service: String,
    /// Description for the IPAM pool.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// ID of the IPAM pool.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// ID of the scope the pool belongs to.
    #[builder(into)]
    #[serde(rename = "ipamScopeId")]
    pub r#ipam_scope_id: String,
    #[builder(into)]
    #[serde(rename = "ipamScopeType")]
    pub r#ipam_scope_type: String,
    /// Locale is the Region where your pool is available for allocations. You can only create pools with locales that match the operating Regions of the IPAM. You can only create VPCs from a pool whose locale matches the VPC's Region.
    #[builder(into)]
    #[serde(rename = "locale")]
    pub r#locale: String,
    #[builder(into)]
    #[serde(rename = "poolDepth")]
    pub r#pool_depth: i32,
    /// Defines whether or not IPv6 pool space is publicly advertisable over the internet.
    #[builder(into)]
    #[serde(rename = "publiclyAdvertisable")]
    pub r#publicly_advertisable: bool,
    /// ID of the source IPAM pool.
    #[builder(into)]
    #[serde(rename = "sourceIpamPoolId")]
    pub r#source_ipam_pool_id: String,
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
    /// Map of tags to assigned to the resource.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: std::collections::HashMap<String, String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVpcIamPoolsIpamPool {
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
                    "address_family",
                    &self.r#address_family,
                ),
                to_pulumi_object_field(
                    "allocation_default_netmask_length",
                    &self.r#allocation_default_netmask_length,
                ),
                to_pulumi_object_field(
                    "allocation_max_netmask_length",
                    &self.r#allocation_max_netmask_length,
                ),
                to_pulumi_object_field(
                    "allocation_min_netmask_length",
                    &self.r#allocation_min_netmask_length,
                ),
                to_pulumi_object_field(
                    "allocation_resource_tags",
                    &self.r#allocation_resource_tags,
                ),
                to_pulumi_object_field(
                    "arn",
                    &self.r#arn,
                ),
                to_pulumi_object_field(
                    "auto_import",
                    &self.r#auto_import,
                ),
                to_pulumi_object_field(
                    "aws_service",
                    &self.r#aws_service,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "ipam_scope_id",
                    &self.r#ipam_scope_id,
                ),
                to_pulumi_object_field(
                    "ipam_scope_type",
                    &self.r#ipam_scope_type,
                ),
                to_pulumi_object_field(
                    "locale",
                    &self.r#locale,
                ),
                to_pulumi_object_field(
                    "pool_depth",
                    &self.r#pool_depth,
                ),
                to_pulumi_object_field(
                    "publicly_advertisable",
                    &self.r#publicly_advertisable,
                ),
                to_pulumi_object_field(
                    "source_ipam_pool_id",
                    &self.r#source_ipam_pool_id,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVpcIamPoolsIpamPool {
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
                    r#address_family: {
                        let field_value = match fields_map.get("address_family") {
                            Some(value) => value,
                            None => bail!("Missing field 'address_family' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allocation_default_netmask_length: {
                        let field_value = match fields_map.get("allocation_default_netmask_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocation_default_netmask_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allocation_max_netmask_length: {
                        let field_value = match fields_map.get("allocation_max_netmask_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocation_max_netmask_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allocation_min_netmask_length: {
                        let field_value = match fields_map.get("allocation_min_netmask_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocation_min_netmask_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allocation_resource_tags: {
                        let field_value = match fields_map.get("allocation_resource_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocation_resource_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#arn: {
                        let field_value = match fields_map.get("arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_import: {
                        let field_value = match fields_map.get("auto_import") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_import' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aws_service: {
                        let field_value = match fields_map.get("aws_service") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipam_scope_id: {
                        let field_value = match fields_map.get("ipam_scope_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipam_scope_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipam_scope_type: {
                        let field_value = match fields_map.get("ipam_scope_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipam_scope_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#locale: {
                        let field_value = match fields_map.get("locale") {
                            Some(value) => value,
                            None => bail!("Missing field 'locale' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pool_depth: {
                        let field_value = match fields_map.get("pool_depth") {
                            Some(value) => value,
                            None => bail!("Missing field 'pool_depth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#publicly_advertisable: {
                        let field_value = match fields_map.get("publicly_advertisable") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicly_advertisable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_ipam_pool_id: {
                        let field_value = match fields_map.get("source_ipam_pool_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_ipam_pool_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
