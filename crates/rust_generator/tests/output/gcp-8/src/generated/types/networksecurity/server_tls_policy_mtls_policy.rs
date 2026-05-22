#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServerTlsPolicyMtlsPolicy {
    /// Required if the policy is to be used with Traffic Director. For external HTTPS load balancers it must be empty.
    /// Defines the mechanism to obtain the Certificate Authority certificate to validate the client certificate.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "clientValidationCas")]
    pub r#client_validation_cas: Option<Vec<super::super::types::networksecurity::ServerTlsPolicyMtlsPolicyClientValidationCa>>,
    /// When the client presents an invalid certificate or no certificate to the load balancer, the clientValidationMode specifies how the client connection is handled.
    /// Required if the policy is to be used with the external HTTPS load balancing. For Traffic Director it must be empty.
    /// Possible values are: `CLIENT_VALIDATION_MODE_UNSPECIFIED`, `ALLOW_INVALID_OR_MISSING_CLIENT_CERT`, `REJECT_INVALID`.
    #[builder(into)]
    #[serde(rename = "clientValidationMode")]
    pub r#client_validation_mode: Option<String>,
    /// Reference to the TrustConfig from certificatemanager.googleapis.com namespace.
    /// If specified, the chain validation will be performed against certificates configured in the given TrustConfig.
    /// Allowed only if the policy is to be used with external HTTPS load balancers.
    #[builder(into)]
    #[serde(rename = "clientValidationTrustConfig")]
    pub r#client_validation_trust_config: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServerTlsPolicyMtlsPolicy {
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
                    "clientValidationCas",
                    &self.r#client_validation_cas,
                ),
                to_pulumi_object_field(
                    "clientValidationMode",
                    &self.r#client_validation_mode,
                ),
                to_pulumi_object_field(
                    "clientValidationTrustConfig",
                    &self.r#client_validation_trust_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServerTlsPolicyMtlsPolicy {
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
                    r#client_validation_cas: {
                        let field_value = match fields_map.get("clientValidationCas") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientValidationCas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_validation_mode: {
                        let field_value = match fields_map.get("clientValidationMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientValidationMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_validation_trust_config: {
                        let field_value = match fields_map.get("clientValidationTrustConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientValidationTrustConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
