create table sys_role
(
    role_id             bigint                                 not null comment '角色ID'
        primary key,
    tenant_id           varchar(20)  default '000000'          not null comment '租户编号',
    role_name           varchar(30)                            not null comment '角色名称',
    role_key            varchar(100)                           not null comment '角色权限字符串',
    role_sort           int                                    not null comment '显示顺序',
    data_scope          char         default '1'               not null comment '数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）',
    menu_check_strictly tinyint(1)   default 1                 null comment '菜单树选择项是否关联显示',
    dept_check_strictly tinyint(1)   default 1                 null comment '部门树选择项是否关联显示',
    status              char                                   not null comment '角色状态（0正常 1停用）',
    del_flag            char         default '0'               not null comment '删除标志（0代表存在 2代表删除）',
    is_admin            tinyint      default 0                 not null comment '是否超级管理员:  0：否  1：是',
    remark              varchar(500) default ''                not null comment '备注',
    create_dept         bigint                                 not null comment '创建部门',
    create_by           bigint                                 not null comment '创建者',
    create_time         timestamp    default CURRENT_TIMESTAMP not null comment '创建时间',
    update_by           bigint                                 null comment '更新者',
    update_time         datetime                               null on update CURRENT_TIMESTAMP comment '更新时间'
)
    comment '角色信息表';



INSERT INTO sys_role (role_id, tenant_id, role_name, role_key, role_sort, data_scope, menu_check_strictly, dept_check_strictly, status, del_flag, is_admin, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (1, '000000', '超级管理员', 'admin', 1, '1', 1, 1, '0', '0', 1, '演示角色', 100, 1, sysdate(), null, null);
INSERT INTO sys_role (role_id, tenant_id, role_name, role_key, role_sort, data_scope, menu_check_strictly, dept_check_strictly, status, del_flag, is_admin, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (2, '000000', '开发人员', 'dev', 1, '1', 1, 1, '0', '0', 1, '演示角色', 100, 1, sysdate(), null, null);
INSERT INTO sys_role (role_id, tenant_id, role_name, role_key, role_sort, data_scope, menu_check_strictly, dept_check_strictly, status, del_flag, is_admin, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (3, '000000', '测试员', 'test', 1, '1', 1, 1, '0', '0', 1, '演示角色', 100, 1, sysdate(), null, null);
