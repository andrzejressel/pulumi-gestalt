#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorCustomDomainTls {
    /// Resource ID of the Front Door Secret.
    #[builder(into)]
    #[serde(rename = "cdnFrontdoorSecretId")]
    pub r#cdn_frontdoor_secret_id: Option<String>,
    /// Defines the source of the SSL certificate. Possible values include `CustomerCertificate` and `ManagedCertificate`. Defaults to `ManagedCertificate`.
    /// 
    /// ->**NOTE:** It may take up to 15 minutes for the Front Door Service to validate the state and Domain ownership of the Custom Domain.
    #[builder(into)]
    #[serde(rename = "certificateType")]
    pub r#certificate_type: Option<String>,
    /// TLS protocol version that will be used for Https. Possible values include `TLS10` and `TLS12`. Defaults to `TLS12`.
    /// 
    /// > **Note** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more details.
    #[builder(into)]
    #[serde(rename = "minimumTlsVersion")]
    pub r#minimum_tls_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorCustomDomainTls {
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
                    "cdn_frontdoor_secret_id",
                    &self.r#cdn_frontdoor_secret_id,
                ),
                to_pulumi_object_field(
                    "certificate_type",
                    &self.r#certificate_type,
                ),
                to_pulumi_object_field(
                    "minimum_tls_version",
                    &self.r#minimum_tls_version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorCustomDomainTls {
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
                    r#cdn_frontdoor_secret_id: {
                        let field_value = match fields_map.get("cdn_frontdoor_secret_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'cdn_frontdoor_secret_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate_type: {
                        let field_value = match fields_map.get("certificate_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_tls_version: {
                        let field_value = match fields_map.get("minimum_tls_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_tls_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
