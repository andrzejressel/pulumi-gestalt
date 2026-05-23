#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewaySslProfileSslPolicy {
    #[builder(into)]
    pub r#cipher_suites: Option<Vec<String>>,
    /// A list of SSL Protocols which should be disabled on this Application Gateway. Possible values are `TLSv1_0`, `TLSv1_1`, `TLSv1_2` and `TLSv1_3`.
    /// 
    /// > **NOTE:** `disabled_protocols` cannot be set when `policy_name` or `policy_type` are set.
    #[builder(into)]
    pub r#disabled_protocols: Option<Vec<String>>,
    #[builder(into)]
    pub r#min_protocol_version: Option<String>,
    #[builder(into)]
    pub r#policy_name: Option<String>,
    /// The Type of the Policy. Possible values are `Predefined`, `Custom` and `CustomV2`.
    /// 
    /// > **NOTE:** `policy_type` is Required when `policy_name` is set - cannot be set if `disabled_protocols` is set.
    #[builder(into)]
    pub r#policy_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationGatewaySslProfileSslPolicy {
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
                    "cipherSuites",
                    &self.r#cipher_suites,
                ),
                to_pulumi_object_field(
                    "disabledProtocols",
                    &self.r#disabled_protocols,
                ),
                to_pulumi_object_field(
                    "minProtocolVersion",
                    &self.r#min_protocol_version,
                ),
                to_pulumi_object_field(
                    "policyName",
                    &self.r#policy_name,
                ),
                to_pulumi_object_field(
                    "policyType",
                    &self.r#policy_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationGatewaySslProfileSslPolicy {
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
                    r#cipher_suites: {
                        let field_value = match fields_map.get("cipherSuites") {
                            Some(value) => value,
                            None => bail!("Missing field 'cipherSuites' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disabled_protocols: {
                        let field_value = match fields_map.get("disabledProtocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabledProtocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_protocol_version: {
                        let field_value = match fields_map.get("minProtocolVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'minProtocolVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_name: {
                        let field_value = match fields_map.get("policyName") {
                            Some(value) => value,
                            None => bail!("Missing field 'policyName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_type: {
                        let field_value = match fields_map.get("policyType") {
                            Some(value) => value,
                            None => bail!("Missing field 'policyType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
