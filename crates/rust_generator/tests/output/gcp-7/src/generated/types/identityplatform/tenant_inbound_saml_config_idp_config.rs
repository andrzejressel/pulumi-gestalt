#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TenantInboundSamlConfigIdpConfig {
    /// The IDP's certificate data to verify the signature in the SAMLResponse issued by the IDP.
    /// Structure is documented below.
    #[builder(into)]
    pub r#idp_certificates: Vec<super::super::types::identityplatform::TenantInboundSamlConfigIdpConfigIdpCertificate>,
    /// Unique identifier for all SAML entities
    #[builder(into)]
    pub r#idp_entity_id: String,
    /// Indicates if outbounding SAMLRequest should be signed.
    #[builder(into)]
    pub r#sign_request: Option<bool>,
    /// URL to send Authentication request to.
    #[builder(into)]
    pub r#sso_url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TenantInboundSamlConfigIdpConfig {
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
                    "idpCertificates",
                    &self.r#idp_certificates,
                ),
                to_pulumi_object_field(
                    "idpEntityId",
                    &self.r#idp_entity_id,
                ),
                to_pulumi_object_field(
                    "signRequest",
                    &self.r#sign_request,
                ),
                to_pulumi_object_field(
                    "ssoUrl",
                    &self.r#sso_url,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TenantInboundSamlConfigIdpConfig {
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
                    r#idp_certificates: {
                        let field_value = match fields_map.get("idpCertificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'idpCertificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idp_entity_id: {
                        let field_value = match fields_map.get("idpEntityId") {
                            Some(value) => value,
                            None => bail!("Missing field 'idpEntityId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sign_request: {
                        let field_value = match fields_map.get("signRequest") {
                            Some(value) => value,
                            None => bail!("Missing field 'signRequest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sso_url: {
                        let field_value = match fields_map.get("ssoUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssoUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
