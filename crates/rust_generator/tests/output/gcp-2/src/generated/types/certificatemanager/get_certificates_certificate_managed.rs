#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCertificatesCertificateManaged {
    /// Detailed state of the latest authorization attempt for each domain
    /// specified for this Managed Certificate.
    #[builder(into)]
    pub r#authorization_attempt_infos: Vec<super::super::types::certificatemanager::GetCertificatesCertificateManagedAuthorizationAttemptInfo>,
    /// Authorizations that will be used for performing domain authorization. Either issuanceConfig or dnsAuthorizations should be specificed, but not both.
    #[builder(into)]
    pub r#dns_authorizations: Vec<String>,
    /// The domains for which a managed SSL certificate will be generated.
    /// Wildcard domains are only supported with DNS challenge resolution
    #[builder(into)]
    pub r#domains: Vec<String>,
    /// The resource name for a CertificateIssuanceConfig used to configure private PKI certificates in the format projects/*/locations/*/certificateIssuanceConfigs/*.
    /// If this field is not set, the certificates will instead be publicly signed as documented at https://cloud.google.com/load-balancing/docs/ssl-certificates/google-managed-certs#caa.
    /// Either issuanceConfig or dnsAuthorizations should be specificed, but not both.
    #[builder(into)]
    pub r#issuance_config: String,
    /// Information about issues with provisioning this Managed Certificate.
    #[builder(into)]
    pub r#provisioning_issues: Vec<super::super::types::certificatemanager::GetCertificatesCertificateManagedProvisioningIssue>,
    /// A state of this Managed Certificate.
    #[builder(into)]
    pub r#state: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCertificatesCertificateManaged {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "authorizationAttemptInfos",
                    &self.r#authorization_attempt_infos,
                ),
                to_pulumi_object_field(
                    "dnsAuthorizations",
                    &self.r#dns_authorizations,
                ),
                to_pulumi_object_field(
                    "domains",
                    &self.r#domains,
                ),
                to_pulumi_object_field(
                    "issuanceConfig",
                    &self.r#issuance_config,
                ),
                to_pulumi_object_field(
                    "provisioningIssues",
                    &self.r#provisioning_issues,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCertificatesCertificateManaged {
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
                        let field_value = match fields_map.get("authorizationAttemptInfos") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorizationAttemptInfos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_authorizations: {
                        let field_value = match fields_map.get("dnsAuthorizations") {
                            Some(value) => value,
                            None => bail!("Missing field 'dnsAuthorizations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("issuanceConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuanceConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioning_issues: {
                        let field_value = match fields_map.get("provisioningIssues") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioningIssues' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
