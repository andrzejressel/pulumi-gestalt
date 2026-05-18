#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewaySslProfileSslPolicy {
    #[builder(into)]
    #[serde(rename = "cipherSuites")]
    pub r#cipher_suites: Option<Vec<String>>,
    /// A list of SSL Protocols which should be disabled on this Application Gateway. Possible values are `TLSv1_0`, `TLSv1_1`, `TLSv1_2` and `TLSv1_3`.
    /// 
    /// > **NOTE:** `disabled_protocols` cannot be set when `policy_name` or `policy_type` are set.
    #[builder(into)]
    #[serde(rename = "disabledProtocols")]
    pub r#disabled_protocols: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "minProtocolVersion")]
    pub r#min_protocol_version: Option<String>,
    #[builder(into)]
    #[serde(rename = "policyName")]
    pub r#policy_name: Option<String>,
    /// The Type of the Policy. Possible values are `Predefined`, `Custom` and `CustomV2`.
    /// 
    /// > **NOTE:** `policy_type` is Required when `policy_name` is set - cannot be set if `disabled_protocols` is set.
    #[builder(into)]
    #[serde(rename = "policyType")]
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
                    "cipher_suites",
                    &self.r#cipher_suites,
                ),
                to_pulumi_object_field(
                    "disabled_protocols",
                    &self.r#disabled_protocols,
                ),
                to_pulumi_object_field(
                    "min_protocol_version",
                    &self.r#min_protocol_version,
                ),
                to_pulumi_object_field(
                    "policy_name",
                    &self.r#policy_name,
                ),
                to_pulumi_object_field(
                    "policy_type",
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
                        let field_value = match fields_map.get("cipher_suites") {
                            Some(value) => value,
                            None => bail!("Missing field 'cipher_suites' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disabled_protocols: {
                        let field_value = match fields_map.get("disabled_protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabled_protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_protocol_version: {
                        let field_value = match fields_map.get("min_protocol_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_protocol_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_name: {
                        let field_value = match fields_map.get("policy_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_type: {
                        let field_value = match fields_map.get("policy_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
