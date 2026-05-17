#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstancePrivateConfig {
    /// CA pool resource, resource must in the format of `projects/{project}/locations/{location}/caPools/{ca_pool}`.
    #[builder(into)]
    #[serde(rename = "caPool")]
    pub r#ca_pool: String,
    /// (Output)
    /// Service Attachment for HTTP, resource is in the format of `projects/{project}/regions/{region}/serviceAttachments/{service_attachment}`.
    #[builder(into)]
    #[serde(rename = "httpServiceAttachment")]
    pub r#http_service_attachment: Option<String>,
    /// 'Indicate if it's private instance.'
    #[builder(into)]
    #[serde(rename = "isPrivate")]
    pub r#is_private: bool,
    /// (Output)
    /// Service Attachment for SSH, resource is in the format of `projects/{project}/regions/{region}/serviceAttachments/{service_attachment}`.
    #[builder(into)]
    #[serde(rename = "sshServiceAttachment")]
    pub r#ssh_service_attachment: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstancePrivateConfig {
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
                "ca_pool".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ca_pool,
                )
                .await,
            );
            map.insert(
                "http_service_attachment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_service_attachment,
                )
                .await,
            );
            map.insert(
                "is_private".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_private,
                )
                .await,
            );
            map.insert(
                "ssh_service_attachment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssh_service_attachment,
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
                        let field_value = match fields_map.get("ca_pool") {
                            Some(value) => value,
                            None => bail!("Missing field 'ca_pool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_service_attachment: {
                        let field_value = match fields_map.get("http_service_attachment") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_service_attachment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_private: {
                        let field_value = match fields_map.get("is_private") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_private' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_service_attachment: {
                        let field_value = match fields_map.get("ssh_service_attachment") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_service_attachment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
