#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NextGenerationFirewallVirtualHubPanoramaPanorama {
    #[builder(into)]
    #[serde(rename = "deviceGroupName")]
    pub r#device_group_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[builder(into)]
    #[serde(rename = "panoramaServer1")]
    pub r#panorama_server_1: Option<String>,
    #[builder(into)]
    #[serde(rename = "panoramaServer2")]
    pub r#panorama_server_2: Option<String>,
    #[builder(into)]
    #[serde(rename = "templateName")]
    pub r#template_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "virtualMachineSshKey")]
    pub r#virtual_machine_ssh_key: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NextGenerationFirewallVirtualHubPanoramaPanorama {
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
                "device_group_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_group_name,
                )
                .await,
            );
            map.insert(
                "host_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_name,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "panorama_server_1".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#panorama_server_1,
                )
                .await,
            );
            map.insert(
                "panorama_server_2".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#panorama_server_2,
                )
                .await,
            );
            map.insert(
                "template_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#template_name,
                )
                .await,
            );
            map.insert(
                "virtual_machine_ssh_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_machine_ssh_key,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NextGenerationFirewallVirtualHubPanoramaPanorama {
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
                    r#device_group_name: {
                        let field_value = match fields_map.get("device_group_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_group_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_name: {
                        let field_value = match fields_map.get("host_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#panorama_server_1: {
                        let field_value = match fields_map.get("panorama_server_1") {
                            Some(value) => value,
                            None => bail!("Missing field 'panorama_server_1' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#panorama_server_2: {
                        let field_value = match fields_map.get("panorama_server_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'panorama_server_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#template_name: {
                        let field_value = match fields_map.get("template_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'template_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_machine_ssh_key: {
                        let field_value = match fields_map.get("virtual_machine_ssh_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_machine_ssh_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
