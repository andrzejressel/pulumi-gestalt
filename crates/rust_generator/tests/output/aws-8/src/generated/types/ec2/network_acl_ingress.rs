#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkAclIngress {
    /// The action to take.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// The CIDR block to match. This must be a
    /// valid network mask.
    #[builder(into)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: Option<String>,
    /// The from port to match.
    #[builder(into)]
    #[serde(rename = "fromPort")]
    pub r#from_port: i32,
    /// The ICMP type code to be used. Default 0.
    /// 
    /// > Note: For more information on ICMP types and codes, see here: https://www.iana.org/assignments/icmp-parameters/icmp-parameters.xhtml
    #[builder(into)]
    #[serde(rename = "icmpCode")]
    pub r#icmp_code: Option<i32>,
    /// The ICMP type to be used. Default 0.
    #[builder(into)]
    #[serde(rename = "icmpType")]
    pub r#icmp_type: Option<i32>,
    /// The IPv6 CIDR block.
    #[builder(into)]
    #[serde(rename = "ipv6CidrBlock")]
    pub r#ipv_6_cidr_block: Option<String>,
    /// The protocol to match. If using the -1 'all'
    /// protocol, you must specify a from and to port of 0.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// The rule number. Used for ordering.
    #[builder(into)]
    #[serde(rename = "ruleNo")]
    pub r#rule_no: i32,
    /// The to port to match.
    #[builder(into)]
    #[serde(rename = "toPort")]
    pub r#to_port: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkAclIngress {
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
                    "action",
                    &self.r#action,
                ),
                to_pulumi_object_field(
                    "cidrBlock",
                    &self.r#cidr_block,
                ),
                to_pulumi_object_field(
                    "fromPort",
                    &self.r#from_port,
                ),
                to_pulumi_object_field(
                    "icmpCode",
                    &self.r#icmp_code,
                ),
                to_pulumi_object_field(
                    "icmpType",
                    &self.r#icmp_type,
                ),
                to_pulumi_object_field(
                    "ipv6CidrBlock",
                    &self.r#ipv_6_cidr_block,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "ruleNo",
                    &self.r#rule_no,
                ),
                to_pulumi_object_field(
                    "toPort",
                    &self.r#to_port,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkAclIngress {
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
                    r#action: {
                        let field_value = match fields_map.get("action") {
                            Some(value) => value,
                            None => bail!("Missing field 'action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cidr_block: {
                        let field_value = match fields_map.get("cidrBlock") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidrBlock' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from_port: {
                        let field_value = match fields_map.get("fromPort") {
                            Some(value) => value,
                            None => bail!("Missing field 'fromPort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#icmp_code: {
                        let field_value = match fields_map.get("icmpCode") {
                            Some(value) => value,
                            None => bail!("Missing field 'icmpCode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#icmp_type: {
                        let field_value = match fields_map.get("icmpType") {
                            Some(value) => value,
                            None => bail!("Missing field 'icmpType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_cidr_block: {
                        let field_value = match fields_map.get("ipv6CidrBlock") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv6CidrBlock' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#rule_no: {
                        let field_value = match fields_map.get("ruleNo") {
                            Some(value) => value,
                            None => bail!("Missing field 'ruleNo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#to_port: {
                        let field_value = match fields_map.get("toPort") {
                            Some(value) => value,
                            None => bail!("Missing field 'toPort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
