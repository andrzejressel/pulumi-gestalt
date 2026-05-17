#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LaunchTemplatePlacement {
    /// The affinity setting for an instance on a Dedicated Host.
    #[builder(into)]
    #[serde(rename = "affinity")]
    pub r#affinity: Option<String>,
    /// The Availability Zone for the instance.
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Option<String>,
    /// The name of the placement group for the instance.
    #[builder(into)]
    #[serde(rename = "groupName")]
    pub r#group_name: Option<String>,
    /// The ID of the Dedicated Host for the instance.
    #[builder(into)]
    #[serde(rename = "hostId")]
    pub r#host_id: Option<String>,
    /// The ARN of the Host Resource Group in which to launch instances.
    #[builder(into)]
    #[serde(rename = "hostResourceGroupArn")]
    pub r#host_resource_group_arn: Option<String>,
    /// The number of the partition the instance should launch in. Valid only if the placement group strategy is set to partition.
    #[builder(into)]
    #[serde(rename = "partitionNumber")]
    pub r#partition_number: Option<i32>,
    /// Reserved for future use.
    #[builder(into)]
    #[serde(rename = "spreadDomain")]
    pub r#spread_domain: Option<String>,
    /// The tenancy of the instance (if the instance is running in a VPC). Can be `default`, `dedicated`, or `host`.
    #[builder(into)]
    #[serde(rename = "tenancy")]
    pub r#tenancy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LaunchTemplatePlacement {
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
                    "affinity",
                    &self.r#affinity,
                ),
                to_pulumi_object_field(
                    "availability_zone",
                    &self.r#availability_zone,
                ),
                to_pulumi_object_field(
                    "group_name",
                    &self.r#group_name,
                ),
                to_pulumi_object_field(
                    "host_id",
                    &self.r#host_id,
                ),
                to_pulumi_object_field(
                    "host_resource_group_arn",
                    &self.r#host_resource_group_arn,
                ),
                to_pulumi_object_field(
                    "partition_number",
                    &self.r#partition_number,
                ),
                to_pulumi_object_field(
                    "spread_domain",
                    &self.r#spread_domain,
                ),
                to_pulumi_object_field(
                    "tenancy",
                    &self.r#tenancy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LaunchTemplatePlacement {
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
                    r#affinity: {
                        let field_value = match fields_map.get("affinity") {
                            Some(value) => value,
                            None => bail!("Missing field 'affinity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#availability_zone: {
                        let field_value = match fields_map.get("availability_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'availability_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_name: {
                        let field_value = match fields_map.get("group_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_id: {
                        let field_value = match fields_map.get("host_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_resource_group_arn: {
                        let field_value = match fields_map.get("host_resource_group_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_resource_group_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#partition_number: {
                        let field_value = match fields_map.get("partition_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'partition_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spread_domain: {
                        let field_value = match fields_map.get("spread_domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'spread_domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tenancy: {
                        let field_value = match fields_map.get("tenancy") {
                            Some(value) => value,
                            None => bail!("Missing field 'tenancy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
