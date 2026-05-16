#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateRenewalSummary {
    /// The status of ACM's managed renewal of the certificate
    #[builder(into)]
    #[serde(rename = "renewalStatus")]
    pub r#renewal_status: Option<String>,
    /// The reason that a renewal request was unsuccessful or is pending
    #[builder(into)]
    #[serde(rename = "renewalStatusReason")]
    pub r#renewal_status_reason: Option<String>,
    #[builder(into)]
    #[serde(rename = "updatedAt")]
    pub r#updated_at: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateRenewalSummary {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("renewal_status".to_string(), self.r#renewal_status.to_pulumi_value().await);
            map.insert("renewal_status_reason".to_string(), self.r#renewal_status_reason.to_pulumi_value().await);
            map.insert("updated_at".to_string(), self.r#updated_at.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateRenewalSummary {
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
                    r#renewal_status: {
                        let field_value = match fields_map.get("renewal_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'renewal_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#renewal_status_reason: {
                        let field_value = match fields_map.get("renewal_status_reason") {
                            Some(value) => value,
                            None => bail!("Missing field 'renewal_status_reason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#updated_at: {
                        let field_value = match fields_map.get("updated_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'updated_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
