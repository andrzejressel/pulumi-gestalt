#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateConfigX509ConfigKeyUsage {
    /// Describes high-level ways in which a key may be used.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "baseKeyUsage")]
    pub r#base_key_usage: Box<super::super::types::certificateauthority::CertificateConfigX509ConfigKeyUsageBaseKeyUsage>,
    /// Describes high-level ways in which a key may be used.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "extendedKeyUsage")]
    pub r#extended_key_usage: Box<super::super::types::certificateauthority::CertificateConfigX509ConfigKeyUsageExtendedKeyUsage>,
    /// An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "unknownExtendedKeyUsages")]
    pub r#unknown_extended_key_usages: Option<Vec<super::super::types::certificateauthority::CertificateConfigX509ConfigKeyUsageUnknownExtendedKeyUsage>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateConfigX509ConfigKeyUsage {
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
                    "base_key_usage",
                    &self.r#base_key_usage,
                ),
                to_pulumi_object_field(
                    "extended_key_usage",
                    &self.r#extended_key_usage,
                ),
                to_pulumi_object_field(
                    "unknown_extended_key_usages",
                    &self.r#unknown_extended_key_usages,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateConfigX509ConfigKeyUsage {
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
                    r#base_key_usage: {
                        let field_value = match fields_map.get("base_key_usage") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_key_usage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extended_key_usage: {
                        let field_value = match fields_map.get("extended_key_usage") {
                            Some(value) => value,
                            None => bail!("Missing field 'extended_key_usage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unknown_extended_key_usages: {
                        let field_value = match fields_map.get("unknown_extended_key_usages") {
                            Some(value) => value,
                            None => bail!("Missing field 'unknown_extended_key_usages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
