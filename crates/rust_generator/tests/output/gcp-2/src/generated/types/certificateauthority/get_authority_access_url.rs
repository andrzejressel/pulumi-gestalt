#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAuthorityAccessUrl {
    /// The URL where this CertificateAuthority's CA certificate is published. This will only be
    /// set for CAs that have been activated.
    #[builder(into)]
    #[serde(rename = "caCertificateAccessUrl")]
    pub r#ca_certificate_access_url: String,
    /// The URL where this CertificateAuthority's CRLs are published. This will only be set for
    /// CAs that have been activated.
    #[builder(into)]
    #[serde(rename = "crlAccessUrls")]
    pub r#crl_access_urls: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAuthorityAccessUrl {
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
                "ca_certificate_access_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ca_certificate_access_url,
                )
                .await,
            );
            map.insert(
                "crl_access_urls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#crl_access_urls,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAuthorityAccessUrl {
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
                    r#ca_certificate_access_url: {
                        let field_value = match fields_map.get("ca_certificate_access_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'ca_certificate_access_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#crl_access_urls: {
                        let field_value = match fields_map.get("crl_access_urls") {
                            Some(value) => value,
                            None => bail!("Missing field 'crl_access_urls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
