#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateTemplatePredefinedValuesKeyUsageExtendedKeyUsage {
    /// Corresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as "TLS WWW client authentication", though regularly used for non-WWW TLS.
    #[builder(into)]
    pub r#client_auth: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as "Signing of downloadable executable code client authentication".
    #[builder(into)]
    pub r#code_signing: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as "Email protection".
    #[builder(into)]
    pub r#email_protection: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as "Signing OCSP responses".
    #[builder(into)]
    pub r#ocsp_signing: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as "TLS WWW server authentication", though regularly used for non-WWW TLS.
    #[builder(into)]
    pub r#server_auth: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as "Binding the hash of an object to a time".
    #[builder(into)]
    pub r#time_stamping: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateTemplatePredefinedValuesKeyUsageExtendedKeyUsage {
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
                    "clientAuth",
                    &self.r#client_auth,
                ),
                to_pulumi_object_field(
                    "codeSigning",
                    &self.r#code_signing,
                ),
                to_pulumi_object_field(
                    "emailProtection",
                    &self.r#email_protection,
                ),
                to_pulumi_object_field(
                    "ocspSigning",
                    &self.r#ocsp_signing,
                ),
                to_pulumi_object_field(
                    "serverAuth",
                    &self.r#server_auth,
                ),
                to_pulumi_object_field(
                    "timeStamping",
                    &self.r#time_stamping,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateTemplatePredefinedValuesKeyUsageExtendedKeyUsage {
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
                    r#client_auth: {
                        let field_value = match fields_map.get("clientAuth") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientAuth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#code_signing: {
                        let field_value = match fields_map.get("codeSigning") {
                            Some(value) => value,
                            None => bail!("Missing field 'codeSigning' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_protection: {
                        let field_value = match fields_map.get("emailProtection") {
                            Some(value) => value,
                            None => bail!("Missing field 'emailProtection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ocsp_signing: {
                        let field_value = match fields_map.get("ocspSigning") {
                            Some(value) => value,
                            None => bail!("Missing field 'ocspSigning' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_auth: {
                        let field_value = match fields_map.get("serverAuth") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverAuth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_stamping: {
                        let field_value = match fields_map.get("timeStamping") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeStamping' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
