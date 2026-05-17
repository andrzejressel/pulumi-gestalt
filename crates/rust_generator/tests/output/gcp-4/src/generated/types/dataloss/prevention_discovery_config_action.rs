#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigAction {
    /// Export data profiles into a provided location
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "exportData")]
    pub r#export_data: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigActionExportData>>,
    /// Publish a message into the Pub/Sub topic.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pubSubNotification")]
    pub r#pub_sub_notification: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigActionPubSubNotification>>,
    /// Publish a message into the Pub/Sub topic.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tagResources")]
    pub r#tag_resources: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigActionTagResources>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDiscoveryConfigAction {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "export_data",
                    &self.r#export_data,
                ),
                to_pulumi_object_field(
                    "pub_sub_notification",
                    &self.r#pub_sub_notification,
                ),
                to_pulumi_object_field(
                    "tag_resources",
                    &self.r#tag_resources,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDiscoveryConfigAction {
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
                    r#export_data: {
                        let field_value = match fields_map.get("export_data") {
                            Some(value) => value,
                            None => bail!("Missing field 'export_data' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pub_sub_notification: {
                        let field_value = match fields_map.get("pub_sub_notification") {
                            Some(value) => value,
                            None => bail!("Missing field 'pub_sub_notification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_resources: {
                        let field_value = match fields_map.get("tag_resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
