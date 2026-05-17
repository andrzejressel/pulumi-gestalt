#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCoreNetworkPolicyDocumentSegmentActionVia {
    /// A list of strings. The network function group to use for the service insertion action.
    #[builder(into)]
    #[serde(rename = "networkFunctionGroups")]
    pub r#network_function_groups: Option<Vec<String>>,
    /// Any edge overrides and the preferred edge to use.
    #[builder(into)]
    #[serde(rename = "withEdgeOverrides")]
    pub r#with_edge_overrides: Option<Vec<super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegmentActionViaWithEdgeOverride>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCoreNetworkPolicyDocumentSegmentActionVia {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "network_function_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_function_groups,
                )
                .await,
            );
            map.insert(
                "with_edge_overrides".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#with_edge_overrides,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCoreNetworkPolicyDocumentSegmentActionVia {
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
                    r#network_function_groups: {
                        let field_value = match fields_map.get("network_function_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_function_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#with_edge_overrides: {
                        let field_value = match fields_map.get("with_edge_overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'with_edge_overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
