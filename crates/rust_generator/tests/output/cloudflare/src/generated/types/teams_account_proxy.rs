#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TeamsAccountProxy {
    /// Sets the time limit in seconds that a user can use an override code to bypass WARP.
    #[builder(into)]
    #[serde(rename = "disableForTime")]
    pub r#disable_for_time: i32,
    /// Whether root ca is enabled account wide for ZT clients.
    #[builder(into)]
    #[serde(rename = "rootCa")]
    pub r#root_ca: bool,
    /// Whether gateway proxy is enabled on gateway devices for TCP traffic.
    #[builder(into)]
    #[serde(rename = "tcp")]
    pub r#tcp: bool,
    /// Whether gateway proxy is enabled on gateway devices for UDP traffic.
    #[builder(into)]
    #[serde(rename = "udp")]
    pub r#udp: bool,
    /// Whether virtual IP (CGNAT) is enabled account wide and will override existing local interface IP for ZT clients.
    #[builder(into)]
    #[serde(rename = "virtualIp")]
    pub r#virtual_ip: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TeamsAccountProxy {
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
                "disable_for_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_for_time,
                )
                .await,
            );
            map.insert(
                "root_ca".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#root_ca,
                )
                .await,
            );
            map.insert(
                "tcp".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tcp,
                )
                .await,
            );
            map.insert(
                "udp".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#udp,
                )
                .await,
            );
            map.insert(
                "virtual_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_ip,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TeamsAccountProxy {
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
                    r#disable_for_time: {
                        let field_value = match fields_map.get("disable_for_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_for_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_ca: {
                        let field_value = match fields_map.get("root_ca") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_ca' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tcp: {
                        let field_value = match fields_map.get("tcp") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#udp: {
                        let field_value = match fields_map.get("udp") {
                            Some(value) => value,
                            None => bail!("Missing field 'udp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_ip: {
                        let field_value = match fields_map.get("virtual_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
