#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkloadIdentityPoolProviderX509TrustStore {
    /// Set of intermediate CA certificates used for building the trust chain to
    /// trust anchor.
    /// IMPORTANT: Intermediate CAs are only supported when configuring x509 federation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "intermediateCas")]
    pub r#intermediate_cas: Option<Vec<super::super::types::iam::WorkloadIdentityPoolProviderX509TrustStoreIntermediateCa>>,
    /// List of Trust Anchors to be used while performing validation
    /// against a given TrustStore. The incoming end entity's certificate
    /// must be chained up to one of the trust anchors here.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "trustAnchors")]
    pub r#trust_anchors: Vec<super::super::types::iam::WorkloadIdentityPoolProviderX509TrustStoreTrustAnchor>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkloadIdentityPoolProviderX509TrustStore {
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
                    "intermediate_cas",
                    &self.r#intermediate_cas,
                ),
                to_pulumi_object_field(
                    "trust_anchors",
                    &self.r#trust_anchors,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkloadIdentityPoolProviderX509TrustStore {
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
                    r#intermediate_cas: {
                        let field_value = match fields_map.get("intermediate_cas") {
                            Some(value) => value,
                            None => bail!("Missing field 'intermediate_cas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trust_anchors: {
                        let field_value = match fields_map.get("trust_anchors") {
                            Some(value) => value,
                            None => bail!("Missing field 'trust_anchors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
