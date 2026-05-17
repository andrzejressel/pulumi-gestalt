#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyClusterAdmissionRule {
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "cluster")]
    pub r#cluster: String,
    /// The action when a pod creation is denied by the admission rule.
    /// Possible values are: `ENFORCED_BLOCK_AND_AUDIT_LOG`, `DRYRUN_AUDIT_LOG_ONLY`.
    #[builder(into)]
    #[serde(rename = "enforcementMode")]
    pub r#enforcement_mode: String,
    /// How this admission rule will be evaluated.
    /// Possible values are: `ALWAYS_ALLOW`, `REQUIRE_ATTESTATION`, `ALWAYS_DENY`.
    #[builder(into)]
    #[serde(rename = "evaluationMode")]
    pub r#evaluation_mode: String,
    /// The resource names of the attestors that must attest to a
    /// container image. If the attestor is in a different project from the
    /// policy, it should be specified in the format `projects/*/attestors/*`.
    /// Each attestor must exist before a policy can reference it. To add an
    /// attestor to a policy the principal issuing the policy change
    /// request must be able to read the attestor resource.
    /// Note: this field must be non-empty when the evaluation_mode field
    /// specifies REQUIRE_ATTESTATION, otherwise it must be empty.
    #[builder(into)]
    #[serde(rename = "requireAttestationsBies")]
    pub r#require_attestations_bies: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyClusterAdmissionRule {
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
                "cluster".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster,
                )
                .await,
            );
            map.insert(
                "enforcement_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enforcement_mode,
                )
                .await,
            );
            map.insert(
                "evaluation_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#evaluation_mode,
                )
                .await,
            );
            map.insert(
                "require_attestations_bies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_attestations_bies,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyClusterAdmissionRule {
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
                    r#cluster: {
                        let field_value = match fields_map.get("cluster") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforcement_mode: {
                        let field_value = match fields_map.get("enforcement_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforcement_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#evaluation_mode: {
                        let field_value = match fields_map.get("evaluation_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluation_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_attestations_bies: {
                        let field_value = match fields_map.get("require_attestations_bies") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_attestations_bies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
