#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AttestorAttestationAuthorityNote {
    /// (Output)
    /// This field will contain the service account email address that
    /// this Attestor will use as the principal when querying Container
    /// Analysis. Attestor administrators must grant this service account
    /// the IAM role needed to read attestations from the noteReference in
    /// Container Analysis (containeranalysis.notes.occurrences.viewer).
    /// This email address is fixed for the lifetime of the Attestor, but
    /// callers should not make any other assumptions about the service
    /// account email; future versions may use an email based on a
    /// different naming pattern.
    #[builder(into)]
    #[serde(rename = "delegationServiceAccountEmail")]
    pub r#delegation_service_account_email: Option<String>,
    /// The resource name of a ATTESTATION_AUTHORITY Note, created by the
    /// user. If the Note is in a different project from the Attestor, it
    /// should be specified in the format `projects/*/notes/*` (or the legacy
    /// `providers/*/notes/*`). This field may not be updated.
    /// An attestation by this attestor is stored as a Container Analysis
    /// ATTESTATION_AUTHORITY Occurrence that names a container image
    /// and that links to this Note.
    #[builder(into)]
    #[serde(rename = "noteReference")]
    pub r#note_reference: String,
    /// Public keys that verify attestations signed by this attestor. This
    /// field may be updated.
    /// If this field is non-empty, one of the specified public keys must
    /// verify that an attestation was signed by this attestor for the
    /// image specified in the admission request.
    /// If this field is empty, this attestor always returns that no valid
    /// attestations exist.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "publicKeys")]
    pub r#public_keys: Option<Vec<super::super::types::binaryauthorization::AttestorAttestationAuthorityNotePublicKey>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AttestorAttestationAuthorityNote {
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
                    "delegation_service_account_email",
                    &self.r#delegation_service_account_email,
                ),
                to_pulumi_object_field(
                    "note_reference",
                    &self.r#note_reference,
                ),
                to_pulumi_object_field(
                    "public_keys",
                    &self.r#public_keys,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AttestorAttestationAuthorityNote {
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
                    r#delegation_service_account_email: {
                        let field_value = match fields_map.get("delegation_service_account_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'delegation_service_account_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#note_reference: {
                        let field_value = match fields_map.get("note_reference") {
                            Some(value) => value,
                            None => bail!("Missing field 'note_reference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_keys: {
                        let field_value = match fields_map.get("public_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
