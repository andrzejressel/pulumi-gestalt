#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkSimStaticIpConfiguration {
    /// The ID of attached data network on which the static.
    #[builder(into)]
    #[serde(rename = "attachedDataNetworkId")]
    pub r#attached_data_network_id: String,
    #[builder(into)]
    #[serde(rename = "sliceId")]
    pub r#slice_id: String,
    /// The IPv4 address assigned to the SIM at this network scope.
    #[builder(into)]
    #[serde(rename = "staticIpv4Address")]
    pub r#static_ipv_4_address: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkSimStaticIpConfiguration {
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
                "attached_data_network_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#attached_data_network_id,
                )
                .await,
            );
            map.insert(
                "slice_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#slice_id,
                )
                .await,
            );
            map.insert(
                "static_ipv_4_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#static_ipv_4_address,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNetworkSimStaticIpConfiguration {
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
                    r#attached_data_network_id: {
                        let field_value = match fields_map.get("attached_data_network_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'attached_data_network_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slice_id: {
                        let field_value = match fields_map.get("slice_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'slice_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#static_ipv_4_address: {
                        let field_value = match fields_map.get("static_ipv_4_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'static_ipv_4_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
