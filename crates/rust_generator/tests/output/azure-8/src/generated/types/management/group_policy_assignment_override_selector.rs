#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupPolicyAssignmentOverrideSelector {
    #[builder(into)]
    #[serde(rename = "ins")]
    pub r#ins: Option<Vec<String>>,
    /// Specifies which characteristic will narrow down the set of evaluated resources. Possible values are `resourceLocation`, `resourceType` and `resourceWithoutLocation`.
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: Option<String>,
    #[builder(into)]
    #[serde(rename = "notIns")]
    pub r#not_ins: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GroupPolicyAssignmentOverrideSelector {
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
                "ins".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ins,
                )
                .await,
            );
            map.insert(
                "kind".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kind,
                )
                .await,
            );
            map.insert(
                "not_ins".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#not_ins,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GroupPolicyAssignmentOverrideSelector {
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
                    r#ins: {
                        let field_value = match fields_map.get("ins") {
                            Some(value) => value,
                            None => bail!("Missing field 'ins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kind: {
                        let field_value = match fields_map.get("kind") {
                            Some(value) => value,
                            None => bail!("Missing field 'kind' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#not_ins: {
                        let field_value = match fields_map.get("not_ins") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_ins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
