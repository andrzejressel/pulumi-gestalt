#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ActivityLogAlertCriteriaResourceHealth {
    /// The current resource health statuses that will log an alert. Possible values are `Available`, `Degraded`, `Unavailable` and `Unknown`.
    #[builder(into)]
    #[serde(rename = "currents")]
    pub r#currents: Option<Vec<String>>,
    /// The previous resource health statuses that will log an alert. Possible values are `Available`, `Degraded`, `Unavailable` and `Unknown`.
    #[builder(into)]
    #[serde(rename = "previouses")]
    pub r#previouses: Option<Vec<String>>,
    /// The reason that will log an alert. Possible values are `PlatformInitiated` (such as a problem with the resource in an affected region of an Azure incident), `UserInitiated` (such as a shutdown request of a VM) and `Unknown`.
    #[builder(into)]
    #[serde(rename = "reasons")]
    pub r#reasons: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ActivityLogAlertCriteriaResourceHealth {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "currents",
                    &self.r#currents,
                ),
                to_pulumi_object_field(
                    "previouses",
                    &self.r#previouses,
                ),
                to_pulumi_object_field(
                    "reasons",
                    &self.r#reasons,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ActivityLogAlertCriteriaResourceHealth {
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
                    r#currents: {
                        let field_value = match fields_map.get("currents") {
                            Some(value) => value,
                            None => bail!("Missing field 'currents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#previouses: {
                        let field_value = match fields_map.get("previouses") {
                            Some(value) => value,
                            None => bail!("Missing field 'previouses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reasons: {
                        let field_value = match fields_map.get("reasons") {
                            Some(value) => value,
                            None => bail!("Missing field 'reasons' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
