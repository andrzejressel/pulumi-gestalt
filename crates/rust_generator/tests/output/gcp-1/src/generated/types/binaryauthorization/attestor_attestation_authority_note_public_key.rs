#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AttestorAttestationAuthorityNotePublicKey {
    /// ASCII-armored representation of a PGP public key, as the
    /// entire output by the command
    /// `gpg --export --armor foo@example.com` (either LF or CRLF
    /// line endings). When using this field, id should be left
    /// blank. The BinAuthz API handlers will calculate the ID
    /// and fill it in automatically. BinAuthz computes this ID
    /// as the OpenPGP RFC4880 V4 fingerprint, represented as
    /// upper-case hex. If id is provided by the caller, it will
    /// be overwritten by the API-calculated ID.
    #[builder(into)]
    #[serde(rename = "asciiArmoredPgpPublicKey")]
    pub r#ascii_armored_pgp_public_key: Option<String>,
    /// A descriptive comment. This field may be updated.
    #[builder(into)]
    #[serde(rename = "comment")]
    pub r#comment: Option<String>,
    /// The ID of this public key. Signatures verified by BinAuthz
    /// must include the ID of the public key that can be used to
    /// verify them, and that ID must match the contents of this
    /// field exactly. Additional restrictions on this field can
    /// be imposed based on which public key type is encapsulated.
    /// See the documentation on publicKey cases below for details.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// A raw PKIX SubjectPublicKeyInfo format public key.
    /// NOTE: id may be explicitly provided by the caller when using this
    /// type of public key, but it MUST be a valid RFC3986 URI. If id is left
    /// blank, a default one will be computed based on the digest of the DER
    /// encoding of the public key.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pkixPublicKey")]
    pub r#pkix_public_key: Option<Box<super::super::types::binaryauthorization::AttestorAttestationAuthorityNotePublicKeyPkixPublicKey>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AttestorAttestationAuthorityNotePublicKey {
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
                    "ascii_armored_pgp_public_key",
                    &self.r#ascii_armored_pgp_public_key,
                ),
                to_pulumi_object_field(
                    "comment",
                    &self.r#comment,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "pkix_public_key",
                    &self.r#pkix_public_key,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AttestorAttestationAuthorityNotePublicKey {
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
                    r#ascii_armored_pgp_public_key: {
                        let field_value = match fields_map.get("ascii_armored_pgp_public_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'ascii_armored_pgp_public_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#comment: {
                        let field_value = match fields_map.get("comment") {
                            Some(value) => value,
                            None => bail!("Missing field 'comment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pkix_public_key: {
                        let field_value = match fields_map.get("pkix_public_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'pkix_public_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
