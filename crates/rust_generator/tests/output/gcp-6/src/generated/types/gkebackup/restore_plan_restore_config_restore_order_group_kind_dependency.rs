#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RestorePlanRestoreConfigRestoreOrderGroupKindDependency {
    /// The requiring group kind requires that the satisfying
    /// group kind be restored first.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requiring")]
    pub r#requiring: Box<super::super::types::gkebackup::RestorePlanRestoreConfigRestoreOrderGroupKindDependencyRequiring>,
    /// The satisfying group kind must be restored first
    /// in order to satisfy the dependency.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "satisfying")]
    pub r#satisfying: Box<super::super::types::gkebackup::RestorePlanRestoreConfigRestoreOrderGroupKindDependencySatisfying>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RestorePlanRestoreConfigRestoreOrderGroupKindDependency {
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
                "requiring".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#requiring,
                )
                .await,
            );
            map.insert(
                "satisfying".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#satisfying,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RestorePlanRestoreConfigRestoreOrderGroupKindDependency {
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
                    r#requiring: {
                        let field_value = match fields_map.get("requiring") {
                            Some(value) => value,
                            None => bail!("Missing field 'requiring' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#satisfying: {
                        let field_value = match fields_map.get("satisfying") {
                            Some(value) => value,
                            None => bail!("Missing field 'satisfying' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
