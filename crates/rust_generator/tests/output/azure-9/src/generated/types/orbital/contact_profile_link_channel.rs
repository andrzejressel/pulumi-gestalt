#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContactProfileLinkChannel {
    /// Bandwidth in MHz.
    #[builder(into)]
    #[serde(rename = "bandwidthMhz")]
    pub r#bandwidth_mhz: f64,
    /// Center frequency in MHz.
    #[builder(into)]
    #[serde(rename = "centerFrequencyMhz")]
    pub r#center_frequency_mhz: f64,
    /// Copy of the modem configuration file such as Kratos QRadio or Kratos QuantumRx. Only valid for downlink directions. If provided, the modem connects to the customer endpoint and sends demodulated data instead of a VITA.49 stream.
    #[builder(into)]
    #[serde(rename = "demodulationConfiguration")]
    pub r#demodulation_configuration: Option<String>,
    /// Customer End point to store/retrieve data during a contact. An `end_point` block as defined below.
    #[builder(into)]
    #[serde(rename = "endPoints")]
    pub r#end_points: Vec<super::super::types::orbital::ContactProfileLinkChannelEndPoint>,
    /// Copy of the modem configuration file such as Kratos QRadio. Only valid for uplink directions. If provided, the modem connects to the customer endpoint and accepts commands from the customer instead of a VITA.49 stream.
    #[builder(into)]
    #[serde(rename = "modulationConfiguration")]
    pub r#modulation_configuration: Option<String>,
    /// Name of the channel.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContactProfileLinkChannel {
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
                "bandwidth_mhz".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bandwidth_mhz,
                )
                .await,
            );
            map.insert(
                "center_frequency_mhz".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#center_frequency_mhz,
                )
                .await,
            );
            map.insert(
                "demodulation_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#demodulation_configuration,
                )
                .await,
            );
            map.insert(
                "end_points".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#end_points,
                )
                .await,
            );
            map.insert(
                "modulation_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#modulation_configuration,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ContactProfileLinkChannel {
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
                    r#bandwidth_mhz: {
                        let field_value = match fields_map.get("bandwidth_mhz") {
                            Some(value) => value,
                            None => bail!("Missing field 'bandwidth_mhz' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#center_frequency_mhz: {
                        let field_value = match fields_map.get("center_frequency_mhz") {
                            Some(value) => value,
                            None => bail!("Missing field 'center_frequency_mhz' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#demodulation_configuration: {
                        let field_value = match fields_map.get("demodulation_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'demodulation_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#end_points: {
                        let field_value = match fields_map.get("end_points") {
                            Some(value) => value,
                            None => bail!("Missing field 'end_points' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#modulation_configuration: {
                        let field_value = match fields_map.get("modulation_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'modulation_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
