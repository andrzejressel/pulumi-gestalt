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
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "mode",
                    &self.r#mode,
                ),
                to_pulumi_object_field(
                    "private_ranges",
                    &self.r#private_ranges,
                ),
                to_pulumi_object_field(
                    "signature_overrides",
                    &self.r#signature_overrides,
                ),
                to_pulumi_object_field(
                    "traffic_bypasses",
                    &self.r#traffic_bypasses,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
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
