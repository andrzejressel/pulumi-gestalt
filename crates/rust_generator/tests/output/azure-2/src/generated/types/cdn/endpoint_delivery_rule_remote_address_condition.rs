#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointDeliveryRuleRemoteAddressCondition {
    /// List of string values. For `GeoMatch` `operator` this should be a list of country codes (e.g. `US` or `DE`). List of IP address if `operator` equals to `IPMatch`. This is required if `operator` is not `Any`.
    #[builder(into)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Option<Vec<String>>,
    /// Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "negateCondition")]
    pub r#negate_condition: Option<bool>,
    /// Valid values are `Any`, `GeoMatch` and `IPMatch`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointDeliveryRuleRemoteAddressCondition {
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
                    "match_values",
                    &self.r#match_values,
                ),
                to_pulumi_object_field(
                    "negate_condition",
                    &self.r#negate_condition,
                ),
                to_pulumi_object_field(
                    "operator",
                    &self.r#operator,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointDeliveryRuleRemoteAddressCondition {
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
                    r#match_values: {
                        let field_value = match fields_map.get("match_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#negate_condition: {
                        let field_value = match fields_map.get("negate_condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'negate_condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operator: {
                        let field_value = match fields_map.get("operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
