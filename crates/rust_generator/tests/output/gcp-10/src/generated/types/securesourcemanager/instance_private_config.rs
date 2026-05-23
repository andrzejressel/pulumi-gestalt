#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstancePrivateConfig {
    /// CA pool resource, resource must in the format of `projects/{project}/locations/{location}/caPools/{ca_pool}`.
    #[builder(into)]
    pub r#ca_pool: String,
    /// (Output)
    /// Service Attachment for HTTP, resource is in the format of `projects/{project}/regions/{region}/serviceAttachments/{service_attachment}`.
    #[builder(into)]
    pub r#http_service_attachment: Option<String>,
    /// 'Indicate if it's private instance.'
    #[builder(into)]
    pub r#is_private: bool,
    /// (Output)
    /// Service Attachment for SSH, resource is in the format of `projects/{project}/regions/{region}/serviceAttachments/{service_attachment}`.
    #[builder(into)]
    pub r#ssh_service_attachment: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstancePrivateConfig {
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
                    "caPool",
                    &self.r#ca_pool,
                ),
                to_pulumi_object_field(
                    "httpServiceAttachment",
                    &self.r#http_service_attachment,
                ),
                to_pulumi_object_field(
                    "isPrivate",
                    &self.r#is_private,
                ),
                to_pulumi_object_field(
                    "sshServiceAttachment",
                    &self.r#ssh_service_attachment,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstancePrivateConfig {
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
                    r#ca_pool: {
                        let field_value = match fields_map.get("caPool") {
                            Some(value) => value,
                            None => bail!("Missing field 'caPool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_service_attachment: {
                        let field_value = match fields_map.get("httpServiceAttachment") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpServiceAttachment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_private: {
                        let field_value = match fields_map.get("isPrivate") {
                            Some(value) => value,
                            None => bail!("Missing field 'isPrivate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_service_attachment: {
                        let field_value = match fields_map.get("sshServiceAttachment") {
                            Some(value) => value,
                            None => bail!("Missing field 'sshServiceAttachment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
