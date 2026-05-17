#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateConfigOverlayAnimationAnimationFade {
    /// The time to end the fade animation, in seconds.
    #[builder(into)]
    #[serde(rename = "endTimeOffset")]
    pub r#end_time_offset: Option<String>,
    /// Required. Type of fade animation: `FADE_IN` or `FADE_OUT`.
    /// The possible values are:
    /// * `FADE_TYPE_UNSPECIFIED`: The fade type is not specified.
    /// * `FADE_IN`: Fade the overlay object into view.
    /// * `FADE_OUT`: Fade the overlay object out of view.
    /// Possible values are: `FADE_TYPE_UNSPECIFIED`, `FADE_IN`, `FADE_OUT`.
    #[builder(into)]
    #[serde(rename = "fadeType")]
    pub r#fade_type: String,
    /// The time to start the fade animation, in seconds.
    #[builder(into)]
    #[serde(rename = "startTimeOffset")]
    pub r#start_time_offset: Option<String>,
    /// Normalized coordinates based on output video resolution.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "xy")]
    pub r#xy: Option<Box<super::super::types::transcoder::JobTemplateConfigOverlayAnimationAnimationFadeXy>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateConfigOverlayAnimationAnimationFade {
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
                "end_time_offset".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#end_time_offset,
                )
                .await,
            );
            map.insert(
                "fade_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fade_type,
                )
                .await,
            );
            map.insert(
                "start_time_offset".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_time_offset,
                )
                .await,
            );
            map.insert(
                "xy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#xy,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobTemplateConfigOverlayAnimationAnimationFade {
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
                    r#end_time_offset: {
                        let field_value = match fields_map.get("end_time_offset") {
                            Some(value) => value,
                            None => bail!("Missing field 'end_time_offset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fade_type: {
                        let field_value = match fields_map.get("fade_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'fade_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_time_offset: {
                        let field_value = match fields_map.get("start_time_offset") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_time_offset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#xy: {
                        let field_value = match fields_map.get("xy") {
                            Some(value) => value,
                            None => bail!("Missing field 'xy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
