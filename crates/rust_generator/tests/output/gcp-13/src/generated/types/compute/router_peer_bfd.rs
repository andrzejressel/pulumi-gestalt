#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RouterPeerBfd {
    /// The minimum interval, in milliseconds, between BFD control packets
    /// received from the peer router. The actual value is negotiated
    /// between the two routers and is equal to the greater of this value
    /// and the transmit interval of the other router. If set, this value
    /// must be between 1000 and 30000.
    #[builder(into)]
    #[serde(rename = "minReceiveInterval")]
    pub r#min_receive_interval: Option<i32>,
    /// The minimum interval, in milliseconds, between BFD control packets
    /// transmitted to the peer router. The actual value is negotiated
    /// between the two routers and is equal to the greater of this value
    /// and the corresponding receive interval of the other router. If set,
    /// this value must be between 1000 and 30000.
    #[builder(into)]
    #[serde(rename = "minTransmitInterval")]
    pub r#min_transmit_interval: Option<i32>,
    /// The number of consecutive BFD packets that must be missed before
    /// BFD declares that a peer is unavailable. If set, the value must
    /// be a value between 5 and 16.
    /// 
    /// <a name="nested_md5_authentication_key"></a>The `md5_authentication_key` block supports:
    #[builder(into)]
    #[serde(rename = "multiplier")]
    pub r#multiplier: Option<i32>,
    /// The BFD session initialization mode for this BGP peer.
    /// If set to `ACTIVE`, the Cloud Router will initiate the BFD session
    /// for this BGP peer. If set to `PASSIVE`, the Cloud Router will wait
    /// for the peer router to initiate the BFD session for this BGP peer.
    /// If set to `DISABLED`, BFD is disabled for this BGP peer.
    /// Possible values are: `ACTIVE`, `DISABLED`, `PASSIVE`.
    #[builder(into)]
    #[serde(rename = "sessionInitializationMode")]
    pub r#session_initialization_mode: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RouterPeerBfd {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "min_receive_interval",
                    &self.r#min_receive_interval,
                ),
                to_pulumi_object_field(
                    "min_transmit_interval",
                    &self.r#min_transmit_interval,
                ),
                to_pulumi_object_field(
                    "multiplier",
                    &self.r#multiplier,
                ),
                to_pulumi_object_field(
                    "session_initialization_mode",
                    &self.r#session_initialization_mode,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RouterPeerBfd {
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
                    r#min_receive_interval: {
                        let field_value = match fields_map.get("min_receive_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_receive_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_transmit_interval: {
                        let field_value = match fields_map.get("min_transmit_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_transmit_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multiplier: {
                        let field_value = match fields_map.get("multiplier") {
                            Some(value) => value,
                            None => bail!("Missing field 'multiplier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_initialization_mode: {
                        let field_value = match fields_map.get("session_initialization_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_initialization_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
