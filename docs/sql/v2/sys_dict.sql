create table sys_dict_type
(
    dict_id     bigint auto_increment comment '字典主键'
        primary key,
    tenant_id   varchar(20)  default '000000'          not null comment '租户编号',
    dict_name   varchar(100) default ''                not null comment '字典名称',
    dict_type   varchar(100) default ''                not null comment '字典类型',
    remark      varchar(500) default ''                not null comment '备注',
    create_dept bigint                                 not null comment '创建部门',
    create_by   bigint                                 not null comment '创建者',
    create_time timestamp    default CURRENT_TIMESTAMP not null comment '创建时间',
    update_by   bigint                                 null comment '更新者',
    update_time datetime                               null on update CURRENT_TIMESTAMP comment '更新时间',
    constraint tenant_id
        unique (tenant_id, dict_type)
)
    comment '字典类型表';

INSERT INTO sys_dict_type (dict_id, tenant_id, dict_name, dict_type, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (1, '000000', '用户性别', 'sys_user_sex', '用户性别列表', 1, 1, sysdate(), null, null);
INSERT INTO sys_dict_type (dict_id, tenant_id, dict_name, dict_type, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (2, '000000', '菜单状态', 'sys_show_hide', '菜单状态列表', 1, 1, sysdate(), null, null);
INSERT INTO sys_dict_type (dict_id, tenant_id, dict_name, dict_type, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (3, '000000', '系统开关', 'sys_normal_disable', '系统开关列表', 1, 1, sysdate(), null, null);
INSERT INTO sys_dict_type (dict_id, tenant_id, dict_name, dict_type, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (6, '000000', '系统是否', 'sys_yes_no', '系统是否列表', 1, 1, sysdate(), null, null);
INSERT INTO sys_dict_type (dict_id, tenant_id, dict_name, dict_type, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (7, '000000', '通知类型', 'sys_notice_type', '通知类型列表', 1, 1, sysdate(), null, null);
INSERT INTO sys_dict_type (dict_id, tenant_id, dict_name, dict_type, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (8, '000000', '通知状态', 'sys_notice_status', '通知状态列表', 1, 1, sysdate(), null, null);
INSERT INTO sys_dict_type (dict_id, tenant_id, dict_name, dict_type, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (9, '000000', '操作类型', 'sys_oper_type', '操作类型列表', 1, 1, sysdate(), null, null);
INSERT INTO sys_dict_type (dict_id, tenant_id, dict_name, dict_type, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (10, '000000', '系统状态', 'sys_common_status', '登录状态列表', 1, 1, sysdate(), null, null);
INSERT INTO sys_dict_type (dict_id, tenant_id, dict_name, dict_type, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (11, '000000', '授权类型', 'sys_grant_type', '认证授权类型', 1, 1, sysdate(), null, null);
INSERT INTO sys_dict_type (dict_id, tenant_id, dict_name, dict_type, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (12, '000000', '设备类型', 'sys_device_type', '客户端设备类型', 1, 1, sysdate(), null, null);



create table sys_dict_data
(
    dict_code   bigint auto_increment comment '字典编码'
        primary key,
    tenant_id   varchar(20)  default '000000'          null comment '租户编号',
    dict_sort   int          default 0                 null comment '字典排序',
    dict_label  varchar(100) default ''                null comment '字典标签',
    dict_value  varchar(100) default ''                null comment '字典键值',
    dict_type   varchar(100) default ''                null comment '字典类型',
    css_class   varchar(100)                           null comment '样式属性（其他样式扩展）',
    list_class  varchar(100)                           null comment '表格回显样式',
    is_default  char         default 'N'               null comment '是否默认（Y是 N否）',
    create_dept bigint                                 not null comment '创建部门',
    create_by   bigint                                 not null comment '创建者',
    create_time timestamp    default CURRENT_TIMESTAMP not null comment '创建时间',
    update_by   bigint                                 null comment '更新者',
    update_time datetime                               null on update CURRENT_TIMESTAMP comment '更新时间',
    remark      varchar(500)                           null comment '备注'
)
    comment '字典数据表';



INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (1, '000000', 1, '男', '0', 'sys_user_sex', '', '', 'Y', 103, 1, sysdate(), null, null, '性别男');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (2, '000000', 2, '女', '1', 'sys_user_sex', '', '', 'N', 103, 1, sysdate(), null, null, '性别女');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (3, '000000', 3, '未知', '2', 'sys_user_sex', '', '', 'N', 103, 1, sysdate(), null, null, '性别未知');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (4, '000000', 1, '显示', '0', 'sys_show_hide', '', 'primary', 'Y', 103, 1, sysdate(), null, null, '显示菜单');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (5, '000000', 2, '隐藏', '1', 'sys_show_hide', '', 'danger', 'N', 103, 1, sysdate(), null, null, '隐藏菜单');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (6, '000000', 1, '正常', '0', 'sys_normal_disable', '', 'primary', 'Y', 103, 1, sysdate(), null, null, '正常状态');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (7, '000000', 2, '停用', '1', 'sys_normal_disable', '', 'danger', 'N', 103, 1, sysdate(), null, null, '停用状态');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (12, '000000', 1, '是', 'Y', 'sys_yes_no', '', 'primary', 'Y', 103, 1, sysdate(), null, null, '系统默认是');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (13, '000000', 2, '否', 'N', 'sys_yes_no', '', 'danger', 'N', 103, 1, sysdate(), null, null, '系统默认否');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (14, '000000', 1, '通知', '1', 'sys_notice_type', '', 'warning', 'Y', 103, 1, sysdate(), null, null, '通知');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (15, '000000', 2, '公告', '2', 'sys_notice_type', '', 'success', 'N', 103, 1, sysdate(), null, null, '公告');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (16, '000000', 1, '正常', '0', 'sys_notice_status', '', 'primary', 'Y', 103, 1, sysdate(), null, null, '正常状态');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (17, '000000', 2, '关闭', '1', 'sys_notice_status', '', 'danger', 'N', 103, 1, sysdate(), null, null, '关闭状态');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (18, '000000', 1, '新增', '1', 'sys_oper_type', '', 'info', 'N', 103, 1, sysdate(), null, null, '新增操作');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (19, '000000', 2, '修改', '2', 'sys_oper_type', '', 'info', 'N', 103, 1, sysdate(), null, null, '修改操作');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (20, '000000', 3, '删除', '3', 'sys_oper_type', '', 'danger', 'N', 103, 1, sysdate(), null, null, '删除操作');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (21, '000000', 4, '授权', '4', 'sys_oper_type', '', 'primary', 'N', 103, 1, sysdate(), null, null, '授权操作');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (22, '000000', 5, '导出', '5', 'sys_oper_type', '', 'warning', 'N', 103, 1, sysdate(), null, null, '导出操作');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (23, '000000', 6, '导入', '6', 'sys_oper_type', '', 'warning', 'N', 103, 1, sysdate(), null, null, '导入操作');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (24, '000000', 7, '强退', '7', 'sys_oper_type', '', 'danger', 'N', 103, 1, sysdate(), null, null, '强退操作');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (25, '000000', 8, '生成代码', '8', 'sys_oper_type', '', 'warning', 'N', 103, 1, sysdate(), null, null, '生成操作');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (26, '000000', 9, '清空数据', '9', 'sys_oper_type', '', 'danger', 'N', 103, 1, sysdate(), null, null, '清空操作');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (27, '000000', 1, '成功', '0', 'sys_common_status', '', 'primary', 'N', 103, 1, sysdate(), null, null, '正常状态');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (28, '000000', 2, '失败', '1', 'sys_common_status', '', 'danger', 'N', 103, 1, sysdate(), null, null, '停用状态');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (29, '000000', 99, '其他', '0', 'sys_oper_type', '', 'info', 'N', 103, 1, sysdate(), null, null, '其他操作');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (30, '000000', 0, '密码认证', 'password', 'sys_grant_type', 'el-check-tag', 'default', 'N', 103, 1, sysdate(), null, null, '密码认证');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (31, '000000', 0, '短信认证', 'sms', 'sys_grant_type', 'el-check-tag', 'default', 'N', 103, 1, sysdate(), null, null, '短信认证');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (32, '000000', 0, '邮件认证', 'email', 'sys_grant_type', 'el-check-tag', 'default', 'N', 103, 1, sysdate(), null, null, '邮件认证');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (33, '000000', 0, '小程序认证', 'xcx', 'sys_grant_type', 'el-check-tag', 'default', 'N', 103, 1, sysdate(), null, null, '小程序认证');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (34, '000000', 0, '三方登录认证', 'social', 'sys_grant_type', 'el-check-tag', 'default', 'N', 103, 1, sysdate(), null, null, '三方登录认证');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (35, '000000', 0, 'PC', 'pc', 'sys_device_type', '', 'default', 'N', 103, 1, sysdate(), null, null, 'PC');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (36, '000000', 0, '安卓', 'android', 'sys_device_type', '', 'default', 'N', 103, 1, sysdate(), null, null, '安卓');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (37, '000000', 0, 'iOS', 'ios', 'sys_device_type', '', 'default', 'N', 103, 1, sysdate(), null, null, 'iOS');
INSERT INTO sys_dict_data (dict_code, tenant_id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, create_dept, create_by, create_time, update_by, update_time, remark) VALUES (38, '000000', 0, '小程序', 'xcx', 'sys_device_type', '', 'default', 'N', 103, 1, sysdate(), null, null, '小程序');


