#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateTemplatePredefinedValuesKeyUsage {
    /// Describes high-level ways in which a key may be used.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "baseKeyUsage")]
    pub r#base_key_usage: Option<Box<super::super::types::certificateauthority::CertificateTemplatePredefinedValuesKeyUsageBaseKeyUsage>>,
    /// Detailed scenarios in which a key may be used.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "extendedKeyUsage")]
    pub r#extended_key_usage: Option<Box<super::super::types::certificateauthority::CertificateTemplatePredefinedValuesKeyUsageExtendedKeyUsage>>,
    /// Used to describe extended key usages that are not listed in the KeyUsage.ExtendedKeyUsageOptions message.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "unknownExtendedKeyUsages")]
    pub r#unknown_extended_key_usages: Option<Vec<super::super::types::certificateauthority::CertificateTemplatePredefinedValuesKeyUsageUnknownExtendedKeyUsage>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateTemplatePredefinedValuesKeyUsage {
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
                    "baseKeyUsage",
                    &self.r#base_key_usage,
                ),
                to_pulumi_object_field(
                    "extendedKeyUsage",
                    &self.r#extended_key_usage,
                ),
                to_pulumi_object_field(
                    "unknownExtendedKeyUsages",
                    &self.r#unknown_extended_key_usages,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateTemplatePredefinedValuesKeyUsage {
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
                        let field_value = match fields_map.get("baseKeyUsage") {
                            Some(value) => value,
                            None => bail!("Missing field 'baseKeyUsage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extended_key_usage: {
                        let field_value = match fields_map.get("extendedKeyUsage") {
                            Some(value) => value,
                            None => bail!("Missing field 'extendedKeyUsage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unknown_extended_key_usages: {
                        let field_value = match fields_map.get("unknownExtendedKeyUsages") {
                            Some(value) => value,
                            None => bail!("Missing field 'unknownExtendedKeyUsages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
