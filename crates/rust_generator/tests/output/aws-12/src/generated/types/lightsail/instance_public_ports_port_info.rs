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
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "cidr_list_aliases".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cidr_list_aliases,
                )
                .await,
            );
            map.insert(
                "cidrs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cidrs,
                )
                .await,
            );
            map.insert(
                "from_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#from_port,
                )
                .await,
            );
            map.insert(
                "ipv_6_cidrs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_6_cidrs,
                )
                .await,
            );
            map.insert(
                "protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocol,
                )
                .await,
            );
            map.insert(
                "to_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#to_port,
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
                        let field_value = match fields_map.get("cidr_list_aliases") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidr_list_aliases' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("from_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'from_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_cidrs: {
                        let field_value = match fields_map.get("ipv_6_cidrs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_cidrs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
