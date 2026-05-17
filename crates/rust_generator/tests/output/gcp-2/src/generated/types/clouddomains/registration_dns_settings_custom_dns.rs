#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistrationDnsSettingsCustomDns {
    /// The list of DS records for this domain, which are used to enable DNSSEC. The domain's DNS provider can provide
    /// the values to set here. If this field is empty, DNSSEC is disabled.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dsRecords")]
    pub r#ds_records: Option<Vec<super::super::types::clouddomains::RegistrationDnsSettingsCustomDnsDsRecord>>,
    /// Required. A list of name servers that store the DNS zone for this domain. Each name server is a domain
    /// name, with Unicode domain names expressed in Punycode format.
    #[builder(into)]
    #[serde(rename = "nameServers")]
    pub r#name_servers: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegistrationDnsSettingsCustomDns {
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
                "ds_records".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ds_records,
                )
                .await,
            );
            map.insert(
                "name_servers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name_servers,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegistrationDnsSettingsCustomDns {
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
                    r#ds_records: {
                        let field_value = match fields_map.get("ds_records") {
                            Some(value) => value,
                            None => bail!("Missing field 'ds_records' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name_servers: {
                        let field_value = match fields_map.get("name_servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'name_servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
