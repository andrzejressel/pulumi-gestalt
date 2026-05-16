#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DashboardDashboardPublishOptions {
    /// Ad hoc (one-time) filtering option. See ad_hoc_filtering_option.
    #[builder(into)]
    #[serde(rename = "adHocFilteringOption")]
    pub r#ad_hoc_filtering_option: Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsAdHocFilteringOption>>,
    /// The drill-down options of data points in a dashboard. See data_point_drill_up_down_option.
    #[builder(into)]
    #[serde(rename = "dataPointDrillUpDownOption")]
    pub r#data_point_drill_up_down_option: Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsDataPointDrillUpDownOption>>,
    /// The data point menu label options of a dashboard. See data_point_menu_label_option.
    #[builder(into)]
    #[serde(rename = "dataPointMenuLabelOption")]
    pub r#data_point_menu_label_option: Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsDataPointMenuLabelOption>>,
    /// The data point tool tip options of a dashboard. See data_point_tooltip_option.
    #[builder(into)]
    #[serde(rename = "dataPointTooltipOption")]
    pub r#data_point_tooltip_option: Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsDataPointTooltipOption>>,
    /// Export to .csv option. See export_to_csv_option.
    #[builder(into)]
    #[serde(rename = "exportToCsvOption")]
    pub r#export_to_csv_option: Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsExportToCsvOption>>,
    /// Determines if hidden fields are exported with a dashboard. See export_with_hidden_fields_option.
    #[builder(into)]
    #[serde(rename = "exportWithHiddenFieldsOption")]
    pub r#export_with_hidden_fields_option: Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsExportWithHiddenFieldsOption>>,
    /// Sheet controls option. See sheet_controls_option.
    #[builder(into)]
    #[serde(rename = "sheetControlsOption")]
    pub r#sheet_controls_option: Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsSheetControlsOption>>,
    /// The sheet layout maximization options of a dashboard. See sheet_layout_element_maximization_option.
    #[builder(into)]
    #[serde(rename = "sheetLayoutElementMaximizationOption")]
    pub r#sheet_layout_element_maximization_option: Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsSheetLayoutElementMaximizationOption>>,
    /// The axis sort options of a dashboard. See visual_axis_sort_option.
    #[builder(into)]
    #[serde(rename = "visualAxisSortOption")]
    pub r#visual_axis_sort_option: Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsVisualAxisSortOption>>,
    /// The menu options of a visual in a dashboard. See visual_menu_option.
    #[builder(into)]
    #[serde(rename = "visualMenuOption")]
    pub r#visual_menu_option: Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsVisualMenuOption>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DashboardDashboardPublishOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("ad_hoc_filtering_option".to_string(), self.r#ad_hoc_filtering_option.to_pulumi_value().await);
            map.insert("data_point_drill_up_down_option".to_string(), self.r#data_point_drill_up_down_option.to_pulumi_value().await);
            map.insert("data_point_menu_label_option".to_string(), self.r#data_point_menu_label_option.to_pulumi_value().await);
            map.insert("data_point_tooltip_option".to_string(), self.r#data_point_tooltip_option.to_pulumi_value().await);
            map.insert("export_to_csv_option".to_string(), self.r#export_to_csv_option.to_pulumi_value().await);
            map.insert("export_with_hidden_fields_option".to_string(), self.r#export_with_hidden_fields_option.to_pulumi_value().await);
            map.insert("sheet_controls_option".to_string(), self.r#sheet_controls_option.to_pulumi_value().await);
            map.insert("sheet_layout_element_maximization_option".to_string(), self.r#sheet_layout_element_maximization_option.to_pulumi_value().await);
            map.insert("visual_axis_sort_option".to_string(), self.r#visual_axis_sort_option.to_pulumi_value().await);
            map.insert("visual_menu_option".to_string(), self.r#visual_menu_option.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DashboardDashboardPublishOptions {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#ad_hoc_filtering_option: {
                        let field_value = match fields_map.get("ad_hoc_filtering_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'ad_hoc_filtering_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsAdHocFilteringOption>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#data_point_drill_up_down_option: {
                        let field_value = match fields_map.get("data_point_drill_up_down_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_point_drill_up_down_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsDataPointDrillUpDownOption>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#data_point_menu_label_option: {
                        let field_value = match fields_map.get("data_point_menu_label_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_point_menu_label_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsDataPointMenuLabelOption>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#data_point_tooltip_option: {
                        let field_value = match fields_map.get("data_point_tooltip_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_point_tooltip_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsDataPointTooltipOption>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#export_to_csv_option: {
                        let field_value = match fields_map.get("export_to_csv_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'export_to_csv_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsExportToCsvOption>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#export_with_hidden_fields_option: {
                        let field_value = match fields_map.get("export_with_hidden_fields_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'export_with_hidden_fields_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsExportWithHiddenFieldsOption>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sheet_controls_option: {
                        let field_value = match fields_map.get("sheet_controls_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'sheet_controls_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsSheetControlsOption>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sheet_layout_element_maximization_option: {
                        let field_value = match fields_map.get("sheet_layout_element_maximization_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'sheet_layout_element_maximization_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsSheetLayoutElementMaximizationOption>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#visual_axis_sort_option: {
                        let field_value = match fields_map.get("visual_axis_sort_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'visual_axis_sort_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsVisualAxisSortOption>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#visual_menu_option: {
                        let field_value = match fields_map.get("visual_menu_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'visual_menu_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::DashboardDashboardPublishOptionsVisualMenuOption>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
