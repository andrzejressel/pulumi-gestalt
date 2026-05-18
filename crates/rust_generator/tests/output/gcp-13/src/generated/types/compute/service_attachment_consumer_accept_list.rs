#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceAttachmentConsumerAcceptList {
    /// The number of consumer forwarding rules the consumer project can
    /// create.
    #[builder(into)]
    #[serde(rename = "connectionLimit")]
    pub r#connection_limit: i32,
    /// The network that is allowed to connect to this service attachment.
    /// Only one of project_id_or_num and network_url may be set.
    #[builder(into)]
    #[serde(rename = "networkUrl")]
    pub r#network_url: Option<String>,
    /// A project that is allowed to connect to this service attachment.
    /// Only one of project_id_or_num and network_url may be set.
    #[builder(into)]
    #[serde(rename = "projectIdOrNum")]
    pub r#project_id_or_num: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceAttachmentConsumerAcceptList {
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
                    "connection_limit",
                    &self.r#connection_limit,
                ),
                to_pulumi_object_field(
                    "network_url",
                    &self.r#network_url,
                ),
                to_pulumi_object_field(
                    "project_id_or_num",
                    &self.r#project_id_or_num,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceAttachmentConsumerAcceptList {
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
                    r#connection_limit: {
                        let field_value = match fields_map.get("connection_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_url: {
                        let field_value = match fields_map.get("network_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#project_id_or_num: {
                        let field_value = match fields_map.get("project_id_or_num") {
                            Some(value) => value,
                            None => bail!("Missing field 'project_id_or_num' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
