#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CaPoolIssuancePolicyAllowedKeyType {
    /// Represents an allowed Elliptic Curve key type.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ellipticCurve")]
    pub r#elliptic_curve: Option<Box<super::super::types::certificateauthority::CaPoolIssuancePolicyAllowedKeyTypeEllipticCurve>>,
    /// Describes an RSA key that may be used in a Certificate issued from a CaPool.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rsa")]
    pub r#rsa: Option<Box<super::super::types::certificateauthority::CaPoolIssuancePolicyAllowedKeyTypeRsa>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CaPoolIssuancePolicyAllowedKeyType {
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
                    "elliptic_curve",
                    &self.r#elliptic_curve,
                ),
                to_pulumi_object_field(
                    "rsa",
                    &self.r#rsa,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CaPoolIssuancePolicyAllowedKeyType {
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
                    r#elliptic_curve: {
                        let field_value = match fields_map.get("elliptic_curve") {
                            Some(value) => value,
                            None => bail!("Missing field 'elliptic_curve' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rsa: {
                        let field_value = match fields_map.get("rsa") {
                            Some(value) => value,
                            None => bail!("Missing field 'rsa' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
