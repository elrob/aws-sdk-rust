// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn serialize_structure_crate_model_tag(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Key");
    if let Some(var_2) = &input.key {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Value");
    if let Some(var_4) = &input.value {
        scope_3.string(var_4);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_scaling_configuration(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::ScalingConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("MinCapacity");
    if let Some(var_6) = &input.min_capacity {
        scope_5.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("MaxCapacity");
    if let Some(var_8) = &input.max_capacity {
        scope_7.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("AutoPause");
    if let Some(var_10) = &input.auto_pause {
        scope_9.boolean(*var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("SecondsUntilAutoPause");
    if let Some(var_12) = &input.seconds_until_auto_pause {
        scope_11.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("TimeoutAction");
    if let Some(var_14) = &input.timeout_action {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("SecondsBeforeTimeout");
    if let Some(var_16) = &input.seconds_before_timeout {
        scope_15.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_processor_feature(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::ProcessorFeature,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("Name");
    if let Some(var_18) = &input.name {
        scope_17.string(var_18);
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("Value");
    if let Some(var_20) = &input.value {
        scope_19.string(var_20);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_user_auth_config(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::UserAuthConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("Description");
    if let Some(var_22) = &input.description {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("UserName");
    if let Some(var_24) = &input.user_name {
        scope_23.string(var_24);
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("AuthScheme");
    if let Some(var_26) = &input.auth_scheme {
        scope_25.string(var_26.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("SecretArn");
    if let Some(var_28) = &input.secret_arn {
        scope_27.string(var_28);
    }
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("IAMAuth");
    if let Some(var_30) = &input.iam_auth {
        scope_29.string(var_30.as_str());
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_filter(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::Filter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("Name");
    if let Some(var_32) = &input.name {
        scope_31.string(var_32);
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("Values");
    if let Some(var_34) = &input.values {
        let mut list_36 = scope_33.start_list(false, Some("Value"));
        for item_35 in var_34 {
            #[allow(unused_mut)]
            let mut entry_37 = list_36.entry();
            entry_37.string(item_35);
        }
        list_36.finish();
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_cloudwatch_logs_export_configuration(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::CloudwatchLogsExportConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_38 = writer.prefix("EnableLogTypes");
    if let Some(var_39) = &input.enable_log_types {
        let mut list_41 = scope_38.start_list(false, None);
        for item_40 in var_39 {
            #[allow(unused_mut)]
            let mut entry_42 = list_41.entry();
            entry_42.string(item_40);
        }
        list_41.finish();
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("DisableLogTypes");
    if let Some(var_44) = &input.disable_log_types {
        let mut list_46 = scope_43.start_list(false, None);
        for item_45 in var_44 {
            #[allow(unused_mut)]
            let mut entry_47 = list_46.entry();
            entry_47.string(item_45);
        }
        list_46.finish();
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_parameter(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::Parameter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_48 = writer.prefix("ParameterName");
    if let Some(var_49) = &input.parameter_name {
        scope_48.string(var_49);
    }
    #[allow(unused_mut)]
    let mut scope_50 = writer.prefix("ParameterValue");
    if let Some(var_51) = &input.parameter_value {
        scope_50.string(var_51);
    }
    #[allow(unused_mut)]
    let mut scope_52 = writer.prefix("Description");
    if let Some(var_53) = &input.description {
        scope_52.string(var_53);
    }
    #[allow(unused_mut)]
    let mut scope_54 = writer.prefix("Source");
    if let Some(var_55) = &input.source {
        scope_54.string(var_55);
    }
    #[allow(unused_mut)]
    let mut scope_56 = writer.prefix("ApplyType");
    if let Some(var_57) = &input.apply_type {
        scope_56.string(var_57);
    }
    #[allow(unused_mut)]
    let mut scope_58 = writer.prefix("DataType");
    if let Some(var_59) = &input.data_type {
        scope_58.string(var_59);
    }
    #[allow(unused_mut)]
    let mut scope_60 = writer.prefix("AllowedValues");
    if let Some(var_61) = &input.allowed_values {
        scope_60.string(var_61);
    }
    #[allow(unused_mut)]
    let mut scope_62 = writer.prefix("IsModifiable");
    if input.is_modifiable {
        scope_62.boolean(input.is_modifiable);
    }
    #[allow(unused_mut)]
    let mut scope_63 = writer.prefix("MinimumEngineVersion");
    if let Some(var_64) = &input.minimum_engine_version {
        scope_63.string(var_64);
    }
    #[allow(unused_mut)]
    let mut scope_65 = writer.prefix("ApplyMethod");
    if let Some(var_66) = &input.apply_method {
        scope_65.string(var_66.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_67 = writer.prefix("SupportedEngineModes");
    if let Some(var_68) = &input.supported_engine_modes {
        let mut list_70 = scope_67.start_list(false, None);
        for item_69 in var_68 {
            #[allow(unused_mut)]
            let mut entry_71 = list_70.entry();
            entry_71.string(item_69);
        }
        list_70.finish();
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_connection_pool_configuration(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::ConnectionPoolConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_72 = writer.prefix("MaxConnectionsPercent");
    if let Some(var_73) = &input.max_connections_percent {
        scope_72.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_73).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_74 = writer.prefix("MaxIdleConnectionsPercent");
    if let Some(var_75) = &input.max_idle_connections_percent {
        scope_74.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_75).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_76 = writer.prefix("ConnectionBorrowTimeout");
    if let Some(var_77) = &input.connection_borrow_timeout {
        scope_76.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_77).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_78 = writer.prefix("SessionPinningFilters");
    if let Some(var_79) = &input.session_pinning_filters {
        let mut list_81 = scope_78.start_list(false, None);
        for item_80 in var_79 {
            #[allow(unused_mut)]
            let mut entry_82 = list_81.entry();
            entry_82.string(item_80);
        }
        list_81.finish();
    }
    #[allow(unused_mut)]
    let mut scope_83 = writer.prefix("InitQuery");
    if let Some(var_84) = &input.init_query {
        scope_83.string(var_84);
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_option_configuration(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::OptionConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_85 = writer.prefix("OptionName");
    if let Some(var_86) = &input.option_name {
        scope_85.string(var_86);
    }
    #[allow(unused_mut)]
    let mut scope_87 = writer.prefix("Port");
    if let Some(var_88) = &input.port {
        scope_87.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_88).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_89 = writer.prefix("OptionVersion");
    if let Some(var_90) = &input.option_version {
        scope_89.string(var_90);
    }
    #[allow(unused_mut)]
    let mut scope_91 = writer.prefix("DBSecurityGroupMemberships");
    if let Some(var_92) = &input.db_security_group_memberships {
        let mut list_94 = scope_91.start_list(false, Some("DBSecurityGroupName"));
        for item_93 in var_92 {
            #[allow(unused_mut)]
            let mut entry_95 = list_94.entry();
            entry_95.string(item_93);
        }
        list_94.finish();
    }
    #[allow(unused_mut)]
    let mut scope_96 = writer.prefix("VpcSecurityGroupMemberships");
    if let Some(var_97) = &input.vpc_security_group_memberships {
        let mut list_99 = scope_96.start_list(false, Some("VpcSecurityGroupId"));
        for item_98 in var_97 {
            #[allow(unused_mut)]
            let mut entry_100 = list_99.entry();
            entry_100.string(item_98);
        }
        list_99.finish();
    }
    #[allow(unused_mut)]
    let mut scope_101 = writer.prefix("OptionSettings");
    if let Some(var_102) = &input.option_settings {
        let mut list_104 = scope_101.start_list(false, Some("OptionSetting"));
        for item_103 in var_102 {
            #[allow(unused_mut)]
            let mut entry_105 = list_104.entry();
            crate::query_ser::serialize_structure_crate_model_option_setting(entry_105, item_103)?;
        }
        list_104.finish();
    }
    Ok(())
}

#[allow(unused_mut)]
pub fn serialize_structure_crate_model_option_setting(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::model::OptionSetting,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_106 = writer.prefix("Name");
    if let Some(var_107) = &input.name {
        scope_106.string(var_107);
    }
    #[allow(unused_mut)]
    let mut scope_108 = writer.prefix("Value");
    if let Some(var_109) = &input.value {
        scope_108.string(var_109);
    }
    #[allow(unused_mut)]
    let mut scope_110 = writer.prefix("DefaultValue");
    if let Some(var_111) = &input.default_value {
        scope_110.string(var_111);
    }
    #[allow(unused_mut)]
    let mut scope_112 = writer.prefix("Description");
    if let Some(var_113) = &input.description {
        scope_112.string(var_113);
    }
    #[allow(unused_mut)]
    let mut scope_114 = writer.prefix("ApplyType");
    if let Some(var_115) = &input.apply_type {
        scope_114.string(var_115);
    }
    #[allow(unused_mut)]
    let mut scope_116 = writer.prefix("DataType");
    if let Some(var_117) = &input.data_type {
        scope_116.string(var_117);
    }
    #[allow(unused_mut)]
    let mut scope_118 = writer.prefix("AllowedValues");
    if let Some(var_119) = &input.allowed_values {
        scope_118.string(var_119);
    }
    #[allow(unused_mut)]
    let mut scope_120 = writer.prefix("IsModifiable");
    if input.is_modifiable {
        scope_120.boolean(input.is_modifiable);
    }
    #[allow(unused_mut)]
    let mut scope_121 = writer.prefix("IsCollection");
    if input.is_collection {
        scope_121.boolean(input.is_collection);
    }
    Ok(())
}
