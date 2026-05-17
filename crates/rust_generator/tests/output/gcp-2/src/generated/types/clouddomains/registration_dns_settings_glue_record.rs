#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistrationDnsSettingsGlueRecord {
    /// Required. Domain name of the host in Punycode format.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: String,
    /// List of IPv4 addresses corresponding to this host in the standard decimal format (e.g. 198.51.100.1).
    /// At least one of ipv4_address and ipv6_address must be set.
    #[builder(into)]
    #[serde(rename = "ipv4Addresses")]
    pub r#ipv_4_addresses: Option<Vec<String>>,
    /// List of IPv4 addresses corresponding to this host in the standard decimal format (e.g. 198.51.100.1).
    /// At least one of ipv4_address and ipv6_address must be set.
    #[builder(into)]
    #[serde(rename = "ipv6Addresses")]
    pub r#ipv_6_addresses: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegistrationDnsSettingsGlueRecord {
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
                "host_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_name,
                )
                .await,
            );
            map.insert(
                "ipv_4_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_4_addresses,
                )
                .await,
            );
            map.insert(
                "ipv_6_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_6_addresses,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegistrationDnsSettingsGlueRecord {
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
                    r#host_name: {
                        let field_value = match fields_map.get("host_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_4_addresses: {
                        let field_value = match fields_map.get("ipv_4_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_4_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_addresses: {
                        let field_value = match fields_map.get("ipv_6_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
