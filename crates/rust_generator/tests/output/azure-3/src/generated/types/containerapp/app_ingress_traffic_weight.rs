#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppIngressTrafficWeight {
    /// The label to apply to the revision as a name prefix for routing traffic.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// This traffic Weight applies to the latest stable Container Revision. At most only one `traffic_weight` block can have the `latest_revision` set to `true`.
    #[builder(into)]
    #[serde(rename = "latestRevision")]
    pub r#latest_revision: Option<bool>,
    /// The percentage of traffic which should be sent this revision.
    /// 
    /// > **Note:** The cumulative values for `weight` must equal 100 exactly and explicitly, no default weights are assumed.
    #[builder(into)]
    #[serde(rename = "percentage")]
    pub r#percentage: i32,
    /// The suffix string to which this `traffic_weight` applies.
    /// 
    /// > **Note:** If `latest_revision` is `false`, the `revision_suffix` shall be specified.
    #[builder(into)]
    #[serde(rename = "revisionSuffix")]
    pub r#revision_suffix: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AppIngressTrafficWeight {
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
                "label".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#label,
                )
                .await,
            );
            map.insert(
                "latest_revision".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#latest_revision,
                )
                .await,
            );
            map.insert(
                "percentage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#percentage,
                )
                .await,
            );
            map.insert(
                "revision_suffix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#revision_suffix,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AppIngressTrafficWeight {
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
                    r#label: {
                        let field_value = match fields_map.get("label") {
                            Some(value) => value,
                            None => bail!("Missing field 'label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#latest_revision: {
                        let field_value = match fields_map.get("latest_revision") {
                            Some(value) => value,
                            None => bail!("Missing field 'latest_revision' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#percentage: {
                        let field_value = match fields_map.get("percentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'percentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#revision_suffix: {
                        let field_value = match fields_map.get("revision_suffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'revision_suffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
