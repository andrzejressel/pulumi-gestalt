#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyBasedRouteFilter {
    /// The destination IP range of outgoing packets that this policy-based route applies to. Default is "0.0.0.0/0" if protocol version is IPv4.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "destRange")]
    pub r#dest_range: Option<String>,
    /// The IP protocol that this policy-based route applies to. Valid values are 'TCP', 'UDP', and 'ALL'. Default is 'ALL'.
    #[builder(into)]
    #[serde(rename = "ipProtocol")]
    pub r#ip_protocol: Option<String>,
    /// Internet protocol versions this policy-based route applies to.
    /// Possible values are: `IPV4`.
    #[builder(into)]
    #[serde(rename = "protocolVersion")]
    pub r#protocol_version: String,
    /// The source IP range of outgoing packets that this policy-based route applies to. Default is "0.0.0.0/0" if protocol version is IPv4.
    #[builder(into)]
    #[serde(rename = "srcRange")]
    pub r#src_range: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyBasedRouteFilter {
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
                "dest_range".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dest_range,
                )
                .await,
            );
            map.insert(
                "ip_protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_protocol,
                )
                .await,
            );
            map.insert(
                "protocol_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocol_version,
                )
                .await,
            );
            map.insert(
                "src_range".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#src_range,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyBasedRouteFilter {
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
                    r#dest_range: {
                        let field_value = match fields_map.get("dest_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'dest_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_protocol: {
                        let field_value = match fields_map.get("ip_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol_version: {
                        let field_value = match fields_map.get("protocol_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#src_range: {
                        let field_value = match fields_map.get("src_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'src_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
