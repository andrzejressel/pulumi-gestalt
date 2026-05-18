#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceEndpointSpecPort {
    /// A random name for the port
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Rrepresents the protocol of a port: `tcp`, `udp` or `sctp`. Defaults to `tcp`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    /// Represents the mode in which the port is to be published: 'ingress' or 'host'. Defaults to `ingress`.
    #[builder(into)]
    #[serde(rename = "publishMode")]
    pub r#publish_mode: Option<String>,
    /// The port on the swarm hosts
    #[builder(into)]
    #[serde(rename = "publishedPort")]
    pub r#published_port: Option<i32>,
    /// The port inside the container
    #[builder(into)]
    #[serde(rename = "targetPort")]
    pub r#target_port: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceEndpointSpecPort {
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
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "publish_mode",
                    &self.r#publish_mode,
                ),
                to_pulumi_object_field(
                    "published_port",
                    &self.r#published_port,
                ),
                to_pulumi_object_field(
                    "target_port",
                    &self.r#target_port,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceEndpointSpecPort {
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
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#publish_mode: {
                        let field_value = match fields_map.get("publish_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'publish_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#published_port: {
                        let field_value = match fields_map.get("published_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'published_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_port: {
                        let field_value = match fields_map.get("target_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
