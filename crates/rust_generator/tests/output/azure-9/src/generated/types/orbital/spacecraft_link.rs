#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpacecraftLink {
    /// Bandwidth in Mhz.
    #[builder(into)]
    #[serde(rename = "bandwidthMhz")]
    pub r#bandwidth_mhz: f64,
    /// Center frequency in Mhz.
    /// 
    /// > **Note:** The value of `center_frequency_mhz +/- bandwidth_mhz / 2` should fall in one of these ranges: `Uplink/LHCP`: [2025, 2120]; `Uplink/Linear`: [399, 403],[435, 438],[449, 451]; `Uplink/RHCP`: [399, 403],[435, 438],[449, 451],[2025, 2120]; `Downlink/LHCP`: [2200, 2300], [7500, 8400]; `Downlink/Linear`: [399, 403], [435, 438], [449, 451]; Downlink/Linear`: [399, 403], [435, 438], [449, 451], [2200, 2300], [7500, 8400]
    #[builder(into)]
    #[serde(rename = "centerFrequencyMhz")]
    pub r#center_frequency_mhz: f64,
    /// Direction if the communication. Possible values are `Uplink` and `Downlink`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: String,
    /// Name of the link.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Polarization. Possible values are `RHCP`, `LHCP`, `linearVertical` and `linearHorizontal`.
    #[builder(into)]
    #[serde(rename = "polarization")]
    pub r#polarization: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpacecraftLink {
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
                "direction".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#direction,
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
                "polarization".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#polarization,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpacecraftLink {
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
                    r#direction: {
                        let field_value = match fields_map.get("direction") {
                            Some(value) => value,
                            None => bail!("Missing field 'direction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#polarization: {
                        let field_value = match fields_map.get("polarization") {
                            Some(value) => value,
                            None => bail!("Missing field 'polarization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
