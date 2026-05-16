#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OrganizationConfigurationDatasources {
    /// Enable Kubernetes Audit Logs Monitoring automatically for new member accounts.
    #[builder(into)]
    #[serde(rename = "kubernetes")]
    pub r#kubernetes: Option<Box<super::super::types::guardduty::OrganizationConfigurationDatasourcesKubernetes>>,
    /// Enable Malware Protection automatically for new member accounts.
    #[builder(into)]
    #[serde(rename = "malwareProtection")]
    pub r#malware_protection: Option<Box<super::super::types::guardduty::OrganizationConfigurationDatasourcesMalwareProtection>>,
    /// Enable S3 Protection automatically for new member accounts.
    #[builder(into)]
    #[serde(rename = "s3Logs")]
    pub r#s_3_logs: Option<Box<super::super::types::guardduty::OrganizationConfigurationDatasourcesS3Logs>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OrganizationConfigurationDatasources {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("kubernetes".to_string(), self.r#kubernetes.to_pulumi_value().await);
            map.insert("malware_protection".to_string(), self.r#malware_protection.to_pulumi_value().await);
            map.insert("s_3_logs".to_string(), self.r#s_3_logs.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OrganizationConfigurationDatasources {
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
                    r#kubernetes: {
                        let field_value = match fields_map.get("kubernetes") {
                            Some(value) => value,
                            None => bail!("Missing field 'kubernetes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::guardduty::OrganizationConfigurationDatasourcesKubernetes>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#malware_protection: {
                        let field_value = match fields_map.get("malware_protection") {
                            Some(value) => value,
                            None => bail!("Missing field 'malware_protection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::guardduty::OrganizationConfigurationDatasourcesMalwareProtection>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_logs: {
                        let field_value = match fields_map.get("s_3_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::guardduty::OrganizationConfigurationDatasourcesS3Logs>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
