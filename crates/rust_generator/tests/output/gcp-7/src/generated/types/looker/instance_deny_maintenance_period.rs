#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceDenyMaintenancePeriod {
    /// Required. Start date of the deny maintenance period
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "endDate")]
    pub r#end_date: Box<super::super::types::looker::InstanceDenyMaintenancePeriodEndDate>,
    /// Required. Start date of the deny maintenance period
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "startDate")]
    pub r#start_date: Box<super::super::types::looker::InstanceDenyMaintenancePeriodStartDate>,
    /// Required. Start time of the window in UTC time.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: Box<super::super::types::looker::InstanceDenyMaintenancePeriodTime>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceDenyMaintenancePeriod {
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
                    "end_date",
                    &self.r#end_date,
                ),
                to_pulumi_object_field(
                    "start_date",
                    &self.r#start_date,
                ),
                to_pulumi_object_field(
                    "time",
                    &self.r#time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceDenyMaintenancePeriod {
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
                    r#end_date: {
                        let field_value = match fields_map.get("end_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'end_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_date: {
                        let field_value = match fields_map.get("start_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time: {
                        let field_value = match fields_map.get("time") {
                            Some(value) => value,
                            None => bail!("Missing field 'time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
