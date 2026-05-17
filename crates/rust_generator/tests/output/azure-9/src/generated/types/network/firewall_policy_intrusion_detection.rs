#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyIntrusionDetection {
    /// In which mode you want to run intrusion detection: `Off`, `Alert` or `Deny`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// A list of Private IP address ranges to identify traffic direction. By default, only ranges defined by IANA RFC 1918 are considered private IP addresses.
    #[builder(into)]
    #[serde(rename = "privateRanges")]
    pub r#private_ranges: Option<Vec<String>>,
    /// One or more `signature_overrides` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "signatureOverrides")]
    pub r#signature_overrides: Option<Vec<super::super::types::network::FirewallPolicyIntrusionDetectionSignatureOverride>>,
    /// One or more `traffic_bypass` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "trafficBypasses")]
    pub r#traffic_bypasses: Option<Vec<super::super::types::network::FirewallPolicyIntrusionDetectionTrafficBypass>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirewallPolicyIntrusionDetection {
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
                "mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mode,
                )
                .await,
            );
            map.insert(
                "private_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_ranges,
                )
                .await,
            );
            map.insert(
                "signature_overrides".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#signature_overrides,
                )
                .await,
            );
            map.insert(
                "traffic_bypasses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#traffic_bypasses,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirewallPolicyIntrusionDetection {
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
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ranges: {
                        let field_value = match fields_map.get("private_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signature_overrides: {
                        let field_value = match fields_map.get("signature_overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'signature_overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#traffic_bypasses: {
                        let field_value = match fields_map.get("traffic_bypasses") {
                            Some(value) => value,
                            None => bail!("Missing field 'traffic_bypasses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
