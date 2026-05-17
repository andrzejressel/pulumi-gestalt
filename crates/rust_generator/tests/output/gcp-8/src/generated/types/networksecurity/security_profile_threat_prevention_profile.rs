#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecurityProfileThreatPreventionProfile {
    /// The configuration for overriding threats actions by severity match.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "severityOverrides")]
    pub r#severity_overrides: Option<Vec<super::super::types::networksecurity::SecurityProfileThreatPreventionProfileSeverityOverride>>,
    /// The configuration for overriding threats actions by threat id match.
    /// If a threat is matched both by configuration provided in severity overrides
    /// and threat overrides, the threat overrides action is applied.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "threatOverrides")]
    pub r#threat_overrides: Option<Vec<super::super::types::networksecurity::SecurityProfileThreatPreventionProfileThreatOverride>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SecurityProfileThreatPreventionProfile {
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
                    "severity_overrides",
                    &self.r#severity_overrides,
                ),
                to_pulumi_object_field(
                    "threat_overrides",
                    &self.r#threat_overrides,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SecurityProfileThreatPreventionProfile {
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
                    r#severity_overrides: {
                        let field_value = match fields_map.get("severity_overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'severity_overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threat_overrides: {
                        let field_value = match fields_map.get("threat_overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'threat_overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
