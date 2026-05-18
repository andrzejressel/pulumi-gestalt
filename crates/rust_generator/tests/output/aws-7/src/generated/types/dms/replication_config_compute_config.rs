#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicationConfigComputeConfig {
    /// The Availability Zone where the DMS Serverless replication using this configuration will run. The default value is a random.
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Option<String>,
    /// A list of custom DNS name servers supported for the DMS Serverless replication to access your source or target database.
    #[builder(into)]
    #[serde(rename = "dnsNameServers")]
    pub r#dns_name_servers: Option<String>,
    /// An Key Management Service (KMS) key Amazon Resource Name (ARN) that is used to encrypt the data during DMS Serverless replication. If you don't specify a value for the KmsKeyId parameter, DMS uses your default encryption key.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Specifies the maximum value of the DMS capacity units (DCUs) for which a given DMS Serverless replication can be provisioned. A single DCU is 2GB of RAM, with 2 DCUs as the minimum value allowed. The list of valid DCU values includes 2, 4, 8, 16, 32, 64, 128, 192, 256, and 384.
    #[builder(into)]
    #[serde(rename = "maxCapacityUnits")]
    pub r#max_capacity_units: Option<i32>,
    /// Specifies the minimum value of the DMS capacity units (DCUs) for which a given DMS Serverless replication can be provisioned. The list of valid DCU values includes 2, 4, 8, 16, 32, 64, 128, 192, 256, and 384. If this value isn't set DMS scans the current activity of available source tables to identify an optimum setting for this parameter.
    #[builder(into)]
    #[serde(rename = "minCapacityUnits")]
    pub r#min_capacity_units: Option<i32>,
    /// Specifies if the replication instance is a multi-az deployment. You cannot set the `availability_zone` parameter if the `multi_az` parameter is set to `true`.
    #[builder(into)]
    #[serde(rename = "multiAz")]
    pub r#multi_az: Option<bool>,
    /// The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).
    /// 
    /// - Default: A 30-minute window selected at random from an 8-hour block of time per region, occurring on a random day of the week.
    /// - Format: `ddd:hh24:mi-ddd:hh24:mi`
    /// - Valid Days: `mon, tue, wed, thu, fri, sat, sun`
    /// - Constraints: Minimum 30-minute window.
    #[builder(into)]
    #[serde(rename = "preferredMaintenanceWindow")]
    pub r#preferred_maintenance_window: Option<String>,
    /// Specifies a subnet group identifier to associate with the DMS Serverless replication.
    #[builder(into)]
    #[serde(rename = "replicationSubnetGroupId")]
    pub r#replication_subnet_group_id: String,
    /// Specifies the virtual private cloud (VPC) security group to use with the DMS Serverless replication. The VPC security group must work with the VPC containing the replication.
    #[builder(into)]
    #[serde(rename = "vpcSecurityGroupIds")]
    pub r#vpc_security_group_ids: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReplicationConfigComputeConfig {
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
                    "availability_zone",
                    &self.r#availability_zone,
                ),
                to_pulumi_object_field(
                    "dns_name_servers",
                    &self.r#dns_name_servers,
                ),
                to_pulumi_object_field(
                    "kms_key_id",
                    &self.r#kms_key_id,
                ),
                to_pulumi_object_field(
                    "max_capacity_units",
                    &self.r#max_capacity_units,
                ),
                to_pulumi_object_field(
                    "min_capacity_units",
                    &self.r#min_capacity_units,
                ),
                to_pulumi_object_field(
                    "multi_az",
                    &self.r#multi_az,
                ),
                to_pulumi_object_field(
                    "preferred_maintenance_window",
                    &self.r#preferred_maintenance_window,
                ),
                to_pulumi_object_field(
                    "replication_subnet_group_id",
                    &self.r#replication_subnet_group_id,
                ),
                to_pulumi_object_field(
                    "vpc_security_group_ids",
                    &self.r#vpc_security_group_ids,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReplicationConfigComputeConfig {
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
                    r#availability_zone: {
                        let field_value = match fields_map.get("availability_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'availability_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_name_servers: {
                        let field_value = match fields_map.get("dns_name_servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_name_servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_id: {
                        let field_value = match fields_map.get("kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_capacity_units: {
                        let field_value = match fields_map.get("max_capacity_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_capacity_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_capacity_units: {
                        let field_value = match fields_map.get("min_capacity_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_capacity_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multi_az: {
                        let field_value = match fields_map.get("multi_az") {
                            Some(value) => value,
                            None => bail!("Missing field 'multi_az' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preferred_maintenance_window: {
                        let field_value = match fields_map.get("preferred_maintenance_window") {
                            Some(value) => value,
                            None => bail!("Missing field 'preferred_maintenance_window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replication_subnet_group_id: {
                        let field_value = match fields_map.get("replication_subnet_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'replication_subnet_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_security_group_ids: {
                        let field_value = match fields_map.get("vpc_security_group_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_security_group_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
