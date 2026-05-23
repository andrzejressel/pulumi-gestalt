#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectPeerConfigurationBgpConfiguration {
    /// A Connect peer core network address.
    #[builder(into)]
    pub r#core_network_address: Option<String>,
    #[builder(into)]
    pub r#core_network_asn: Option<i32>,
    /// The Connect peer address.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    pub r#peer_address: Option<String>,
    #[builder(into)]
    pub r#peer_asn: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectPeerConfigurationBgpConfiguration {
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
                    "coreNetworkAddress",
                    &self.r#core_network_address,
                ),
                to_pulumi_object_field(
                    "coreNetworkAsn",
                    &self.r#core_network_asn,
                ),
                to_pulumi_object_field(
                    "peerAddress",
                    &self.r#peer_address,
                ),
                to_pulumi_object_field(
                    "peerAsn",
                    &self.r#peer_asn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectPeerConfigurationBgpConfiguration {
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
                    r#core_network_address: {
                        let field_value = match fields_map.get("coreNetworkAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'coreNetworkAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#core_network_asn: {
                        let field_value = match fields_map.get("coreNetworkAsn") {
                            Some(value) => value,
                            None => bail!("Missing field 'coreNetworkAsn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peer_address: {
                        let field_value = match fields_map.get("peerAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'peerAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peer_asn: {
                        let field_value = match fields_map.get("peerAsn") {
                            Some(value) => value,
                            None => bail!("Missing field 'peerAsn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
