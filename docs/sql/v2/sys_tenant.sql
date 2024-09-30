create table sys_tenant
(
    id                bigint auto_increment comment 'id'
        primary key,
    tenant_id         varchar(20)                            not null comment '租户编号',
    contact_user_name varchar(20)                            not null comment '联系人',
    contact_phone     varchar(20)                            not null comment '联系电话',
    company_name      varchar(50)                            not null comment '企业名称',
    license_number    varchar(30)                            not null comment '统一社会信用代码',
    address           varchar(200)                           not null comment '地址',
    intro             varchar(200)                           not null comment '企业简介',
    domain            varchar(200)                           not null comment '域名',
    package_id        bigint                                 not null comment '租户套餐编号',
    expire_time       datetime                               not null comment '过期时间',
    account_count     int          default -1                not null comment '用户数量（-1不限制）',
    status            char         default '0'               not null comment '租户状态（0正常 1停用）',
    del_flag          char         default '0'               not null comment '删除标志（0代表存在 2代表删除）',
    remark            varchar(500) default ''                not null comment '备注',
    create_dept       bigint                                 not null comment '创建部门',
    create_by         bigint                                 not null comment '创建者',
    create_time       timestamp    default CURRENT_TIMESTAMP not null comment '创建时间',
    update_by         bigint                                 null comment '更新者',
    update_time       datetime                               null on update CURRENT_TIMESTAMP comment '更新时间'
)
    comment '租户表';




INSERT INTO sys_tenant (id, tenant_id, contact_user_name, contact_phone, company_name, license_number, address, intro, domain, package_id, expire_time, account_count, status, del_flag, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (1, '000000', '管理组', '18613030352', 'XXX有限公司', '123456', '深圳科技园', '多租户通用后台管理管理系统', 'https://www.baidu.com', 1,  sysdate(), -1, '0', '0', '', 1, 1, sysdate(), null, null);


create table sys_tenant_package
(
    package_id          bigint auto_increment comment '租户套餐id'
        primary key,
    package_name        varchar(20)                            not null comment '套餐名称',
    menu_ids            varchar(3000)                          not null comment '关联菜单id',
    menu_check_strictly tinyint(1)   default 1                 not null comment '菜单树选择项是否关联显示',
    status              char         default '0'               not null comment '状态（0正常 1停用）',
    del_flag            char         default '0'               not null comment '删除标志（0代表存在 2代表删除）',
    remark              varchar(500) default ''                not null comment '备注',
    create_dept         bigint                                 not null comment '创建部门',
    create_by           bigint                                 not null comment '创建者',
    create_time         timestamp    default CURRENT_TIMESTAMP not null comment '创建时间',
    update_by           bigint                                 null comment '更新者',
    update_time         datetime                               null on update CURRENT_TIMESTAMP comment '更新时间'
)
    comment '租户套餐表';

