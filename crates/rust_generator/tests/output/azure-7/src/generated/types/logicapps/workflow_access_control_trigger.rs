#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkflowAccessControlTrigger {
    /// A list of the allowed caller IP address ranges.
    #[builder(into)]
    #[serde(rename = "allowedCallerIpAddressRanges")]
    pub r#allowed_caller_ip_address_ranges: Vec<String>,
    /// A `open_authentication_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "openAuthenticationPolicies")]
    pub r#open_authentication_policies: Option<Vec<super::super::types::logicapps::WorkflowAccessControlTriggerOpenAuthenticationPolicy>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkflowAccessControlTrigger {
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
                "allowed_caller_ip_address_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_caller_ip_address_ranges,
                )
                .await,
            );
            map.insert(
                "open_authentication_policies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#open_authentication_policies,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkflowAccessControlTrigger {
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
                    r#allowed_caller_ip_address_ranges: {
                        let field_value = match fields_map.get("allowed_caller_ip_address_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_caller_ip_address_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#open_authentication_policies: {
                        let field_value = match fields_map.get("open_authentication_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'open_authentication_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
