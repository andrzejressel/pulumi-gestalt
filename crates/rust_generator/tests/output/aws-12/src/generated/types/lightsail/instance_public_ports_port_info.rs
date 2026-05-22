#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstancePublicPortsPortInfo {
    /// Set of CIDR aliases that define access for a preconfigured range of IP addresses.
    #[builder(into)]
    #[serde(rename = "cidrListAliases")]
    pub r#cidr_list_aliases: Option<Vec<String>>,
    /// Set of CIDR blocks.
    #[builder(into)]
    #[serde(rename = "cidrs")]
    pub r#cidrs: Option<Vec<String>>,
    /// First port in a range of open ports on an instance.
    #[builder(into)]
    #[serde(rename = "fromPort")]
    pub r#from_port: i32,
    #[builder(into)]
    #[serde(rename = "ipv6Cidrs")]
    pub r#ipv_6_cidrs: Option<Vec<String>>,
    /// IP protocol name. Valid values are `tcp`, `all`, `udp`, and `icmp`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// Last port in a range of open ports on an instance.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "toPort")]
    pub r#to_port: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstancePublicPortsPortInfo {
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
                    "cidrListAliases",
                    &self.r#cidr_list_aliases,
                ),
                to_pulumi_object_field(
                    "cidrs",
                    &self.r#cidrs,
                ),
                to_pulumi_object_field(
                    "fromPort",
                    &self.r#from_port,
                ),
                to_pulumi_object_field(
                    "ipv6Cidrs",
                    &self.r#ipv_6_cidrs,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstancePublicPortsPortInfo {
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
                    r#cidr_list_aliases: {
                        let field_value = match fields_map.get("cidrListAliases") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidrListAliases' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cidrs: {
                        let field_value = match fields_map.get("cidrs") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidrs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ipv_6_cidrs: {
                        let field_value = match fields_map.get("ipv6Cidrs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv6Cidrs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
