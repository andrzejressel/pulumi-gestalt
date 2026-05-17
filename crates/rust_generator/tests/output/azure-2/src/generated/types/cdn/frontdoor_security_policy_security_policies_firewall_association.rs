#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorSecurityPolicySecurityPoliciesFirewallAssociation {
    /// One or more `domain` blocks as defined below. Changing this forces a new Front Door Security Policy to be created.
    #[builder(into)]
    #[serde(rename = "domains")]
    pub r#domains: Vec<super::super::types::cdn::FrontdoorSecurityPolicySecurityPoliciesFirewallAssociationDomain>,
    /// The list of paths to match for this firewall policy. Possible value includes `/*`. Changing this forces a new Front Door Security Policy to be created.
    #[builder(into)]
    #[serde(rename = "patternsToMatch")]
    pub r#patterns_to_match: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorSecurityPolicySecurityPoliciesFirewallAssociation {
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
                "domains".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domains,
                )
                .await,
            );
            map.insert(
                "patterns_to_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#patterns_to_match,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorSecurityPolicySecurityPoliciesFirewallAssociation {
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
                    r#domains: {
                        let field_value = match fields_map.get("domains") {
                            Some(value) => value,
                            None => bail!("Missing field 'domains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#patterns_to_match: {
                        let field_value = match fields_map.get("patterns_to_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'patterns_to_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
