#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkSimStaticIpConfiguration {
    /// The ID of attached data network on which the static IP address will be used. The combination of attached data network and slice defines the network scope of the IP address.
    #[builder(into)]
    #[serde(rename = "attachedDataNetworkId")]
    pub r#attached_data_network_id: String,
    /// The ID of network slice on which the static IP address will be used. The combination of attached data network and slice defines the network scope of the IP address.
    #[builder(into)]
    #[serde(rename = "sliceId")]
    pub r#slice_id: String,
    /// The IPv4 address assigned to the SIM at this network scope. This address must be in the userEquipmentStaticAddressPoolPrefix defined in the attached data network.
    #[builder(into)]
    #[serde(rename = "staticIpv4Address")]
    pub r#static_ipv_4_address: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkSimStaticIpConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "attached_data_network_id",
                    &self.r#attached_data_network_id,
                ),
                to_pulumi_object_field(
                    "slice_id",
                    &self.r#slice_id,
                ),
                to_pulumi_object_field(
                    "static_ipv_4_address",
                    &self.r#static_ipv_4_address,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkSimStaticIpConfiguration {
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
