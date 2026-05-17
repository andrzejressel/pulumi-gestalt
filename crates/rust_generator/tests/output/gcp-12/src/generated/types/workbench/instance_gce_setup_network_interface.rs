#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceGceSetupNetworkInterface {
    /// Optional. An array of configurations for this interface. Currently, only one access
    /// config, ONE_TO_ONE_NAT, is supported. If no accessConfigs specified, the
    /// instance will have an external internet access through an ephemeral
    /// external IP address.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "accessConfigs")]
    pub r#access_configs: Option<Vec<super::super::types::workbench::InstanceGceSetupNetworkInterfaceAccessConfig>>,
    /// Optional. The name of the VPC that this VM instance is in.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// Optional. The type of vNIC to be used on this interface. This
    /// may be gVNIC or VirtioNet.
    /// Possible values are: `VIRTIO_NET`, `GVNIC`.
    #[builder(into)]
    #[serde(rename = "nicType")]
    pub r#nic_type: Option<String>,
    /// Optional. The name of the subnet that this VM instance is in.
    #[builder(into)]
    #[serde(rename = "subnet")]
    pub r#subnet: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceGceSetupNetworkInterface {
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
                "access_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#access_configs,
                )
                .await,
            );
            map.insert(
                "network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network,
                )
                .await,
            );
            map.insert(
                "nic_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nic_type,
                )
                .await,
            );
            map.insert(
                "subnet".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnet,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceGceSetupNetworkInterface {
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
                    r#access_configs: {
                        let field_value = match fields_map.get("access_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nic_type: {
                        let field_value = match fields_map.get("nic_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'nic_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet: {
                        let field_value = match fields_map.get("subnet") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
