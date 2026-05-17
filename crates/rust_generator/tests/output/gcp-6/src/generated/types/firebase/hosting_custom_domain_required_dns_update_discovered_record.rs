#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HostingCustomDomainRequiredDnsUpdateDiscoveredRecord {
    /// The domain name the record pertains to, e.g. `foo.bar.com.`.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Option<String>,
    /// The data of the record. The meaning of the value depends on record type:
    /// - A and AAAA: IP addresses for the domain name.
    /// - CNAME: Another domain to check for records.
    /// - TXT: Arbitrary text strings associated with the domain name. Hosting
    /// uses TXT records to determine a which Firebase Projects have
    /// permission to act on the domain name's behalf.
    /// - CAA: The record's flags, tag, and value, e.g. `0 issue "pki.goog"`.
    #[builder(into)]
    #[serde(rename = "rdata")]
    pub r#rdata: Option<String>,
    /// Indicates the a required action for this record.
    #[builder(into)]
    #[serde(rename = "requiredAction")]
    pub r#required_action: Option<String>,
    /// The record's type, which determines what data the record contains.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HostingCustomDomainRequiredDnsUpdateDiscoveredRecord {
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
                "domain_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domain_name,
                )
                .await,
            );
            map.insert(
                "rdata".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rdata,
                )
                .await,
            );
            map.insert(
                "required_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#required_action,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HostingCustomDomainRequiredDnsUpdateDiscoveredRecord {
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
                    r#domain_name: {
                        let field_value = match fields_map.get("domain_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rdata: {
                        let field_value = match fields_map.get("rdata") {
                            Some(value) => value,
                            None => bail!("Missing field 'rdata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#required_action: {
                        let field_value = match fields_map.get("required_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'required_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
