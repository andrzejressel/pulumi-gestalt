#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainAssociationSubDomain {
    /// Branch name setting for the subdomain.
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: String,
    /// DNS record for the subdomain in a space-prefixed and space-delimited format (` CNAME <target>`).
    #[builder(into)]
    #[serde(rename = "dnsRecord")]
    pub r#dns_record: Option<String>,
    /// Prefix setting for the subdomain.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: String,
    /// Verified status of the subdomain.
    #[builder(into)]
    #[serde(rename = "verified")]
    pub r#verified: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainAssociationSubDomain {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("branch_name".to_string(), self.r#branch_name.to_pulumi_value().await);
            map.insert("dns_record".to_string(), self.r#dns_record.to_pulumi_value().await);
            map.insert("prefix".to_string(), self.r#prefix.to_pulumi_value().await);
            map.insert("verified".to_string(), self.r#verified.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainAssociationSubDomain {
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
                    r#branch_name: {
                        let field_value = match fields_map.get("branch_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'branch_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dns_record: {
                        let field_value = match fields_map.get("dns_record") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_record' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#prefix: {
                        let field_value = match fields_map.get("prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#verified: {
                        let field_value = match fields_map.get("verified") {
                            Some(value) => value,
                            None => bail!("Missing field 'verified' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
