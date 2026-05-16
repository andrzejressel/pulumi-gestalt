#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDedicatedIpPoolDedicatedIp {
    /// IPv4 address.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: String,
    /// Indicates how complete the dedicated IP warm-up process is. When this value equals `1`, the address has completed the warm-up process and is ready for use.
    #[builder(into)]
    #[serde(rename = "warmupPercentage")]
    pub r#warmup_percentage: i32,
    /// The warm-up status of a dedicated IP address. Valid values: `IN_PROGRESS`, `DONE`.
    #[builder(into)]
    #[serde(rename = "warmupStatus")]
    pub r#warmup_status: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDedicatedIpPoolDedicatedIp {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("ip".to_string(), self.r#ip.to_pulumi_value().await);
            map.insert("warmup_percentage".to_string(), self.r#warmup_percentage.to_pulumi_value().await);
            map.insert("warmup_status".to_string(), self.r#warmup_status.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDedicatedIpPoolDedicatedIp {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#ip: {
                        let field_value = match fields_map.get("ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#warmup_percentage: {
                        let field_value = match fields_map.get("warmup_percentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'warmup_percentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#warmup_status: {
                        let field_value = match fields_map.get("warmup_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'warmup_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
