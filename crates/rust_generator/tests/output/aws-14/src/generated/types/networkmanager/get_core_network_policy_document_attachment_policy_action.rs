#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCoreNetworkPolicyDocumentAttachmentPolicyAction {
    /// The name of the network function group to attach to the attachment policy.
    #[builder(into)]
    #[serde(rename = "addToNetworkFunctionGroup")]
    pub r#add_to_network_function_group: Option<String>,
    /// Defines how a segment is mapped. Values can be `constant` or `tag`. `constant` statically defines the segment to associate the attachment to. `tag` uses the value of a tag to dynamically try to map to a segment.reference_policies_elements_condition_operators.html) to evaluate.
    #[builder(into)]
    #[serde(rename = "associationMethod")]
    pub r#association_method: Option<String>,
    /// Determines if this mapping should override the segment value for `require_attachment_acceptance`. You can only set this to `true`, indicating that this setting applies only to segments that have `require_attachment_acceptance` set to `false`. If the segment already has the default `require_attachment_acceptance`, you can set this to inherit segment’s acceptance value.
    #[builder(into)]
    #[serde(rename = "requireAcceptance")]
    pub r#require_acceptance: Option<bool>,
    /// Name of the `segment` to share as defined in the `segments` section. This is used only when the `association_method` is `constant`.
    #[builder(into)]
    #[serde(rename = "segment")]
    pub r#segment: Option<String>,
    /// Maps the attachment to the value of a known key. This is used with the `association_method` is `tag`. For example a `tag` of `stage = “test”`, will map to a segment named `test`. The value must exactly match the name of a segment. This allows you to have many segments, but use only a single rule without having to define multiple nearly identical conditions. This prevents creating many similar conditions that all use the same keys to map to segments.
    #[builder(into)]
    #[serde(rename = "tagValueOfKey")]
    pub r#tag_value_of_key: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCoreNetworkPolicyDocumentAttachmentPolicyAction {
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
                "add_to_network_function_group".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#add_to_network_function_group,
                )
                .await,
            );
            map.insert(
                "association_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#association_method,
                )
                .await,
            );
            map.insert(
                "require_acceptance".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_acceptance,
                )
                .await,
            );
            map.insert(
                "segment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#segment,
                )
                .await,
            );
            map.insert(
                "tag_value_of_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag_value_of_key,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCoreNetworkPolicyDocumentAttachmentPolicyAction {
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
                    r#add_to_network_function_group: {
                        let field_value = match fields_map.get("add_to_network_function_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'add_to_network_function_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#association_method: {
                        let field_value = match fields_map.get("association_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'association_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_acceptance: {
                        let field_value = match fields_map.get("require_acceptance") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_acceptance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#segment: {
                        let field_value = match fields_map.get("segment") {
                            Some(value) => value,
                            None => bail!("Missing field 'segment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_value_of_key: {
                        let field_value = match fields_map.get("tag_value_of_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_value_of_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
