#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceNetworkConfig {
    /// A list of external networks authorized to access this instance. This
    /// field is only allowed to be set when `enable_public_ip` is set to
    /// true.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "authorizedExternalNetworks")]
    pub r#authorized_external_networks: Option<Vec<super::super::types::alloydb::InstanceNetworkConfigAuthorizedExternalNetwork>>,
    /// Enabling outbound public ip for the instance.
    #[builder(into)]
    #[serde(rename = "enableOutboundPublicIp")]
    pub r#enable_outbound_public_ip: Option<bool>,
    /// Enabling public ip for the instance. If a user wishes to disable this,
    /// please also clear the list of the authorized external networks set on
    /// the same instance.
    #[builder(into)]
    #[serde(rename = "enablePublicIp")]
    pub r#enable_public_ip: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceNetworkConfig {
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
                "authorized_external_networks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authorized_external_networks,
                )
                .await,
            );
            map.insert(
                "enable_outbound_public_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_outbound_public_ip,
                )
                .await,
            );
            map.insert(
                "enable_public_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_public_ip,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceNetworkConfig {
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
                    r#authorized_external_networks: {
                        let field_value = match fields_map.get("authorized_external_networks") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorized_external_networks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_outbound_public_ip: {
                        let field_value = match fields_map.get("enable_outbound_public_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_outbound_public_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_public_ip: {
                        let field_value = match fields_map.get("enable_public_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_public_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
