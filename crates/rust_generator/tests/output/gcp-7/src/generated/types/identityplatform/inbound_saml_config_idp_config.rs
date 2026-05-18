#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InboundSamlConfigIdpConfig {
    /// The IdP's certificate data to verify the signature in the SAMLResponse issued by the IDP.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "idpCertificates")]
    pub r#idp_certificates: Vec<super::super::types::identityplatform::InboundSamlConfigIdpConfigIdpCertificate>,
    /// Unique identifier for all SAML entities
    #[builder(into)]
    #[serde(rename = "idpEntityId")]
    pub r#idp_entity_id: String,
    /// Indicates if outbounding SAMLRequest should be signed.
    #[builder(into)]
    #[serde(rename = "signRequest")]
    pub r#sign_request: Option<bool>,
    /// URL to send Authentication request to.
    #[builder(into)]
    #[serde(rename = "ssoUrl")]
    pub r#sso_url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InboundSamlConfigIdpConfig {
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
                    "idp_certificates",
                    &self.r#idp_certificates,
                ),
                to_pulumi_object_field(
                    "idp_entity_id",
                    &self.r#idp_entity_id,
                ),
                to_pulumi_object_field(
                    "sign_request",
                    &self.r#sign_request,
                ),
                to_pulumi_object_field(
                    "sso_url",
                    &self.r#sso_url,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InboundSamlConfigIdpConfig {
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
                        let field_value = match fields_map.get("idp_certificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'idp_certificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idp_entity_id: {
                        let field_value = match fields_map.get("idp_entity_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'idp_entity_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sign_request: {
                        let field_value = match fields_map.get("sign_request") {
                            Some(value) => value,
                            None => bail!("Missing field 'sign_request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sso_url: {
                        let field_value = match fields_map.get("sso_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'sso_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
