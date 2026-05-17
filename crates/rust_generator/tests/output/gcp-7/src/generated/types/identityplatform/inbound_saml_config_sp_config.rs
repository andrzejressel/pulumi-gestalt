#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InboundSamlConfigSpConfig {
    /// Callback URI where responses from IDP are handled. Must start with `https://`.
    #[builder(into)]
    #[serde(rename = "callbackUri")]
    pub r#callback_uri: Option<String>,
    /// (Output)
    /// The IDP's certificate data to verify the signature in the SAMLResponse issued by the IDP.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_sp_certificates"></a>The `sp_certificates` block contains:
    #[builder(into)]
    #[serde(rename = "spCertificates")]
    pub r#sp_certificates: Option<Vec<super::super::types::identityplatform::InboundSamlConfigSpConfigSpCertificate>>,
    /// Unique identifier for all SAML entities.
    #[builder(into)]
    #[serde(rename = "spEntityId")]
    pub r#sp_entity_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InboundSamlConfigSpConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "callback_uri",
                    &self.r#callback_uri,
                ),
                to_pulumi_object_field(
                    "sp_certificates",
                    &self.r#sp_certificates,
                ),
                to_pulumi_object_field(
                    "sp_entity_id",
                    &self.r#sp_entity_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InboundSamlConfigSpConfig {
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
                    r#callback_uri: {
                        let field_value = match fields_map.get("callback_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'callback_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sp_certificates: {
                        let field_value = match fields_map.get("sp_certificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'sp_certificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sp_entity_id: {
                        let field_value = match fields_map.get("sp_entity_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'sp_entity_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
