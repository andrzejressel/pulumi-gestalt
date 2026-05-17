#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionNetwork {
    /// List of ports, or port pairs, to forward from the virtual machine to the application container.
    #[builder(into)]
    #[serde(rename = "forwardedPorts")]
    pub r#forwarded_ports: Option<Vec<String>>,
    /// Prevent instances from receiving an ephemeral external IP address.
    /// Possible values are: `EXTERNAL`, `INTERNAL`.
    #[builder(into)]
    #[serde(rename = "instanceIpMode")]
    pub r#instance_ip_mode: Option<String>,
    /// Tag to apply to the instance during creation.
    #[builder(into)]
    #[serde(rename = "instanceTag")]
    pub r#instance_tag: Option<String>,
    /// Google Compute Engine network where the virtual machines are created. Specify the short name, not the resource path.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Enable session affinity.
    #[builder(into)]
    #[serde(rename = "sessionAffinity")]
    pub r#session_affinity: Option<bool>,
    /// Google Cloud Platform sub-network where the virtual machines are created. Specify the short name, not the resource path.
    /// If the network that the instance is being created in is a Legacy network, then the IP address is allocated from the IPv4Range.
    /// If the network that the instance is being created in is an auto Subnet Mode Network, then only network name should be specified (not the subnetworkName) and the IP address is created from the IPCidrRange of the subnetwork that exists in that zone for that network.
    /// If the network that the instance is being created in is a custom Subnet Mode Network, then the subnetworkName must be specified and the IP address is created from the IPCidrRange of the subnetwork.
    /// If specified, the subnetwork must exist in the same region as the App Engine flexible environment application.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlexibleAppVersionNetwork {
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
                "forwarded_ports".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#forwarded_ports,
                )
                .await,
            );
            map.insert(
                "instance_ip_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_ip_mode,
                )
                .await,
            );
            map.insert(
                "instance_tag".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_tag,
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
                "session_affinity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#session_affinity,
                )
                .await,
            );
            map.insert(
                "subnetwork".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnetwork,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlexibleAppVersionNetwork {
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
                    r#forwarded_ports: {
                        let field_value = match fields_map.get("forwarded_ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwarded_ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_ip_mode: {
                        let field_value = match fields_map.get("instance_ip_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_ip_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_tag: {
                        let field_value = match fields_map.get("instance_tag") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_tag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#session_affinity: {
                        let field_value = match fields_map.get("session_affinity") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_affinity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnetwork: {
                        let field_value = match fields_map.get("subnetwork") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetwork' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
