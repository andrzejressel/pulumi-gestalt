#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateManaged {
    /// (Output)
    /// Detailed state of the latest authorization attempt for each domain
    /// specified for this Managed Certificate.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_provisioning_issue"></a>The `provisioning_issue` block contains:
    #[builder(into)]
    #[serde(rename = "authorizationAttemptInfos")]
    pub r#authorization_attempt_infos: Option<Vec<super::super::types::certificatemanager::CertificateManagedAuthorizationAttemptInfo>>,
    /// Authorizations that will be used for performing domain authorization. Either issuanceConfig or dnsAuthorizations should be specificed, but not both.
    #[builder(into)]
    #[serde(rename = "dnsAuthorizations")]
    pub r#dns_authorizations: Option<Vec<String>>,
    /// The domains for which a managed SSL certificate will be generated.
    /// Wildcard domains are only supported with DNS challenge resolution
    #[builder(into)]
    #[serde(rename = "domains")]
    pub r#domains: Option<Vec<String>>,
    /// The resource name for a CertificateIssuanceConfig used to configure private PKI certificates in the format projects/*/locations/*/certificateIssuanceConfigs/*.
    /// If this field is not set, the certificates will instead be publicly signed as documented at https://cloud.google.com/load-balancing/docs/ssl-certificates/google-managed-certs#caa.
    /// Either issuanceConfig or dnsAuthorizations should be specificed, but not both.
    #[builder(into)]
    #[serde(rename = "issuanceConfig")]
    pub r#issuance_config: Option<String>,
    /// (Output)
    /// Information about issues with provisioning this Managed Certificate.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "provisioningIssues")]
    pub r#provisioning_issues: Option<Vec<super::super::types::certificatemanager::CertificateManagedProvisioningIssue>>,
    /// (Output)
    /// State of the domain for managed certificate issuance.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateManaged {
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
                "authorization_attempt_infos".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authorization_attempt_infos,
                )
                .await,
            );
            map.insert(
                "dns_authorizations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dns_authorizations,
                )
                .await,
            );
            map.insert(
                "domains".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domains,
                )
                .await,
            );
            map.insert(
                "issuance_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#issuance_config,
                )
                .await,
            );
            map.insert(
                "provisioning_issues".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#provisioning_issues,
                )
                .await,
            );
            map.insert(
                "state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateManaged {
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
                    r#authorization_attempt_infos: {
                        let field_value = match fields_map.get("authorization_attempt_infos") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorization_attempt_infos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_authorizations: {
                        let field_value = match fields_map.get("dns_authorizations") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_authorizations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domains: {
                        let field_value = match fields_map.get("domains") {
                            Some(value) => value,
                            None => bail!("Missing field 'domains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#issuance_config: {
                        let field_value = match fields_map.get("issuance_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuance_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioning_issues: {
                        let field_value = match fields_map.get("provisioning_issues") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioning_issues' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
