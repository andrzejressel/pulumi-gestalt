#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IntegrationRuntimeSsisVnetIntegration {
    /// Static public IP addresses for the Azure-SSIS Integration Runtime. The size must be 2.
    #[builder(into)]
    #[serde(rename = "publicIps")]
    pub r#public_ips: Option<Vec<String>>,
    /// id of the subnet to which the nodes of the Azure-SSIS Integration Runtime will be added.
    /// 
    /// > **NOTE** Only one of `subnet_id` and `subnet_name` can be specified. If `subnet_name` is specified, `vnet_id` must be provided.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
    /// Name of the subnet to which the nodes of the Azure-SSIS Integration Runtime will be added.
    #[builder(into)]
    #[serde(rename = "subnetName")]
    pub r#subnet_name: Option<String>,
    /// ID of the virtual network to which the nodes of the Azure-SSIS Integration Runtime will be added.
    #[builder(into)]
    #[serde(rename = "vnetId")]
    pub r#vnet_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IntegrationRuntimeSsisVnetIntegration {
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
                "public_ips".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_ips,
                )
                .await,
            );
            map.insert(
                "subnet_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnet_id,
                )
                .await,
            );
            map.insert(
                "subnet_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnet_name,
                )
                .await,
            );
            map.insert(
                "vnet_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vnet_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IntegrationRuntimeSsisVnetIntegration {
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
                    r#public_ips: {
                        let field_value = match fields_map.get("public_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_name: {
                        let field_value = match fields_map.get("subnet_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnet_id: {
                        let field_value = match fields_map.get("vnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
