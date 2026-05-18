#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagedZoneDnssecConfigDefaultKeySpec {
    /// String mnemonic specifying the DNSSEC algorithm of this key
    /// Possible values are: `ecdsap256sha256`, `ecdsap384sha384`, `rsasha1`, `rsasha256`, `rsasha512`.
    #[builder(into)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Option<String>,
    /// Length of the keys in bits
    #[builder(into)]
    #[serde(rename = "keyLength")]
    pub r#key_length: Option<i32>,
    /// Specifies whether this is a key signing key (KSK) or a zone
    /// signing key (ZSK). Key signing keys have the Secure Entry
    /// Point flag set and, when active, will only be used to sign
    /// resource record sets of type DNSKEY. Zone signing keys do
    /// not have the Secure Entry Point flag set and will be used
    /// to sign all other types of resource record sets.
    /// Possible values are: `keySigning`, `zoneSigning`.
    #[builder(into)]
    #[serde(rename = "keyType")]
    pub r#key_type: Option<String>,
    /// Identifies what kind of resource this is
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ManagedZoneDnssecConfigDefaultKeySpec {
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
                    "algorithm",
                    &self.r#algorithm,
                ),
                to_pulumi_object_field(
                    "key_length",
                    &self.r#key_length,
                ),
                to_pulumi_object_field(
                    "key_type",
                    &self.r#key_type,
                ),
                to_pulumi_object_field(
                    "kind",
                    &self.r#kind,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ManagedZoneDnssecConfigDefaultKeySpec {
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
                    r#algorithm: {
                        let field_value = match fields_map.get("algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_length: {
                        let field_value = match fields_map.get("key_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_type: {
                        let field_value = match fields_map.get("key_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kind: {
                        let field_value = match fields_map.get("kind") {
                            Some(value) => value,
                            None => bail!("Missing field 'kind' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
