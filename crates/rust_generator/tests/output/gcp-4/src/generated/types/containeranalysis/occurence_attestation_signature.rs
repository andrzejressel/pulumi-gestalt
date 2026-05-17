#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OccurenceAttestationSignature {
    /// The identifier for the public key that verifies this
    /// signature. MUST be an RFC3986 conformant
    /// URI. * When possible, the key id should be an
    /// immutable reference, such as a cryptographic digest.
    /// Examples of valid values:
    /// * OpenPGP V4 public key fingerprint. See https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr
    /// for more details on this scheme.
    /// * `openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA`
    /// * RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER serialization):
    /// * "ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU"
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "publicKeyId")]
    pub r#public_key_id: String,
    /// The content of the signature, an opaque bytestring.
    /// The payload that this signature verifies MUST be
    /// unambiguously provided with the Signature during
    /// verification. A wrapper message might provide the
    /// payload explicitly. Alternatively, a message might
    /// have a canonical serialization that can always be
    /// unambiguously computed to derive the payload.
    #[builder(into)]
    #[serde(rename = "signature")]
    pub r#signature: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OccurenceAttestationSignature {
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
                "public_key_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_key_id,
                )
                .await,
            );
            map.insert(
                "signature".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#signature,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OccurenceAttestationSignature {
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
                    r#public_key_id: {
                        let field_value = match fields_map.get("public_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signature: {
                        let field_value = match fields_map.get("signature") {
                            Some(value) => value,
                            None => bail!("Missing field 'signature' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
