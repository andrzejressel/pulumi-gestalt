#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallVirtualHub {
    /// The private IP address associated with the Firewall.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Option<String>,
    /// The list of public IP addresses associated with the Firewall.
    #[builder(into)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Option<Vec<String>>,
    /// Specifies the number of public IPs to assign to the Firewall. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "publicIpCount")]
    pub r#public_ip_count: Option<i32>,
    /// Specifies the ID of the Virtual Hub where the Firewall resides in.
    #[builder(into)]
    #[serde(rename = "virtualHubId")]
    pub r#virtual_hub_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirewallVirtualHub {
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
                    "privateIpAddress",
                    &self.r#private_ip_address,
                ),
                to_pulumi_object_field(
                    "publicIpAddresses",
                    &self.r#public_ip_addresses,
                ),
                to_pulumi_object_field(
                    "publicIpCount",
                    &self.r#public_ip_count,
                ),
                to_pulumi_object_field(
                    "virtualHubId",
                    &self.r#virtual_hub_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirewallVirtualHub {
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
                    r#private_ip_address: {
                        let field_value = match fields_map.get("privateIpAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateIpAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_addresses: {
                        let field_value = match fields_map.get("publicIpAddresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicIpAddresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_count: {
                        let field_value = match fields_map.get("publicIpCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicIpCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_hub_id: {
                        let field_value = match fields_map.get("virtualHubId") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtualHubId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
