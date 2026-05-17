#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RecordSetRoutingPolicyWrr {
    /// The list of targets to be health checked. Note that if DNSSEC is enabled for this zone, only one of `rrdatas` or `health_checked_targets` can be set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "healthCheckedTargets")]
    pub r#health_checked_targets: Option<Box<super::super::types::dns::RecordSetRoutingPolicyWrrHealthCheckedTargets>>,
    /// Same as `rrdatas` above.
    #[builder(into)]
    #[serde(rename = "rrdatas")]
    pub r#rrdatas: Option<Vec<String>>,
    /// The ratio of traffic routed to the target.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RecordSetRoutingPolicyWrr {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "health_checked_targets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#health_checked_targets,
                )
                .await,
            );
            map.insert(
                "rrdatas".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rrdatas,
                )
                .await,
            );
            map.insert(
                "weight".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weight,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RecordSetRoutingPolicyWrr {
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
                    r#health_checked_targets: {
                        let field_value = match fields_map.get("health_checked_targets") {
                            Some(value) => value,
                            None => bail!("Missing field 'health_checked_targets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rrdatas: {
                        let field_value = match fields_map.get("rrdatas") {
                            Some(value) => value,
                            None => bail!("Missing field 'rrdatas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weight: {
                        let field_value = match fields_map.get("weight") {
                            Some(value) => value,
                            None => bail!("Missing field 'weight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
