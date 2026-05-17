#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobBinaryAuthorization {
    /// If present, indicates to use Breakglass using this justification. If useDefault is False, then it must be empty. For more information on breakglass, see https://cloud.google.com/binary-authorization/docs/using-breakglass
    #[builder(into)]
    #[serde(rename = "breakglassJustification")]
    pub r#breakglass_justification: Option<String>,
    /// The path to a binary authorization policy. Format: projects/{project}/platforms/cloudRun/{policy-name}
    #[builder(into)]
    #[serde(rename = "policy")]
    pub r#policy: Option<String>,
    /// If True, indicates to use the default project's binary authorization policy. If False, binary authorization will be disabled.
    #[builder(into)]
    #[serde(rename = "useDefault")]
    pub r#use_default: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobBinaryAuthorization {
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
                "breakglass_justification".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#breakglass_justification,
                )
                .await,
            );
            map.insert(
                "policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#policy,
                )
                .await,
            );
            map.insert(
                "use_default".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_default,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobBinaryAuthorization {
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
                    r#breakglass_justification: {
                        let field_value = match fields_map.get("breakglass_justification") {
                            Some(value) => value,
                            None => bail!("Missing field 'breakglass_justification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy: {
                        let field_value = match fields_map.get("policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_default: {
                        let field_value = match fields_map.get("use_default") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_default' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
