#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AttestorAttestationAuthorityNotePublicKeyPkixPublicKey {
    /// A PEM-encoded public key, as described in
    /// `https://tools.ietf.org/html/rfc7468#section-13`
    #[builder(into)]
    #[serde(rename = "publicKeyPem")]
    pub r#public_key_pem: Option<String>,
    /// The signature algorithm used to verify a message against
    /// a signature using this key. These signature algorithm must
    /// match the structure and any object identifiers encoded in
    /// publicKeyPem (i.e. this algorithm must match that of the
    /// public key).
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "signatureAlgorithm")]
    pub r#signature_algorithm: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AttestorAttestationAuthorityNotePublicKeyPkixPublicKey {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "public_key_pem",
                    &self.r#public_key_pem,
                ),
                to_pulumi_object_field(
                    "signature_algorithm",
                    &self.r#signature_algorithm,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AttestorAttestationAuthorityNotePublicKeyPkixPublicKey {
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
                    r#public_key_pem: {
                        let field_value = match fields_map.get("public_key_pem") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_key_pem' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signature_algorithm: {
                        let field_value = match fields_map.get("signature_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'signature_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
