#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecurityGroupEgress {
    /// List of CIDR blocks.
    #[builder(into)]
    #[serde(rename = "cidrBlocks")]
    pub r#cidr_blocks: Option<Vec<String>>,
    /// Description of this egress rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Start port (or ICMP type number if protocol is `icmp`)
    #[builder(into)]
    #[serde(rename = "fromPort")]
    pub r#from_port: i32,
    /// List of IPv6 CIDR blocks.
    #[builder(into)]
    #[serde(rename = "ipv6CidrBlocks")]
    pub r#ipv_6_cidr_blocks: Option<Vec<String>>,
    /// List of Prefix List IDs.
    #[builder(into)]
    #[serde(rename = "prefixListIds")]
    pub r#prefix_list_ids: Option<Vec<String>>,
    /// Protocol. If you select a protocol of `-1` (semantically equivalent to `all`, which is not a valid value here), you must specify a `from_port` and `to_port` equal to 0. The supported values are defined in the `IpProtocol` argument in the [IpPermission](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_IpPermission.html) API reference.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// List of security groups. A group name can be used relative to the default VPC. Otherwise, group ID.
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Option<Vec<String>>,
    /// Whether the security group itself will be added as a source to this egress rule.
    #[builder(into)]
    #[serde(rename = "self")]
    pub r#self_: Option<bool>,
    /// End range port (or ICMP code if protocol is `icmp`).
    /// 
    /// The following arguments are optional:
    /// 
    /// > **Note** Although `cidr_blocks`, `ipv6_cidr_blocks`, `prefix_list_ids`, and `security_groups` are all marked as optional, you _must_ provide one of them in order to configure the destination of the traffic.
    #[builder(into)]
    #[serde(rename = "toPort")]
    pub r#to_port: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SecurityGroupEgress {
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
                    "cidr_blocks",
                    &self.r#cidr_blocks,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "from_port",
                    &self.r#from_port,
                ),
                to_pulumi_object_field(
                    "ipv_6_cidr_blocks",
                    &self.r#ipv_6_cidr_blocks,
                ),
                to_pulumi_object_field(
                    "prefix_list_ids",
                    &self.r#prefix_list_ids,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "security_groups",
                    &self.r#security_groups,
                ),
                to_pulumi_object_field(
                    "self_",
                    &self.r#self_,
                ),
                to_pulumi_object_field(
                    "to_port",
                    &self.r#to_port,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SecurityGroupEgress {
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
                    r#cidr_blocks: {
                        let field_value = match fields_map.get("cidr_blocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidr_blocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#from_port: {
                        let field_value = match fields_map.get("from_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'from_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_cidr_blocks: {
                        let field_value = match fields_map.get("ipv_6_cidr_blocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_cidr_blocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_list_ids: {
                        let field_value = match fields_map.get("prefix_list_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix_list_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_groups: {
                        let field_value = match fields_map.get("security_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#self_: {
                        let field_value = match fields_map.get("self_") {
                            Some(value) => value,
                            None => bail!("Missing field 'self_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#to_port: {
                        let field_value = match fields_map.get("to_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'to_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
