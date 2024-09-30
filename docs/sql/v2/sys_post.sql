create table sys_post
(
    post_id       bigint auto_increment comment '岗位ID'
        primary key,
    tenant_id     varchar(20)  default '000000'          not null comment '租户编号',
    dept_id       bigint                                 not null comment '部门id',
    post_code     varchar(64)                            not null comment '岗位编码',
    post_category varchar(100) default ''                not null comment '岗位类别编码',
    post_name     varchar(50)                            not null comment '岗位名称',
    post_sort     int                                    not null comment '显示顺序',
    status        char                                   not null comment '状态（0正常 1停用）',
    remark        varchar(500) default ''                not null comment '备注',
    create_dept   bigint                                 not null comment '创建部门',
    create_by     bigint                                 not null comment '创建者',
    create_time   timestamp    default CURRENT_TIMESTAMP not null comment '创建时间',
    update_by     bigint                                 null comment '更新者',
    update_time   datetime                               null on update CURRENT_TIMESTAMP comment '修改时间'
)
    comment '岗位信息表';



INSERT INTO sys_post (post_id, tenant_id, dept_id, post_code, post_category, post_name, post_sort, status, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (1, '000000', 100, 'ceo', '', '董事长', 1, '0', '', 100, 1, sysdate(), null, null);
INSERT INTO sys_post (post_id, tenant_id, dept_id, post_code, post_category, post_name, post_sort, status, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (2, '000000', 100, 'se', '', '项目经理', 2, '0', '', 100, 1, sysdate(), null, null);
INSERT INTO sys_post (post_id, tenant_id, dept_id, post_code, post_category, post_name, post_sort, status, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (3, '000000', 100, 'hr', '', '人力资源', 3, '0', '', 100, 1, sysdate(), null, null);
INSERT INTO sys_post (post_id, tenant_id, dept_id, post_code, post_category, post_name, post_sort, status, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (4, '000000', 100, 'user', '', '普通员工', 4, '0', '', 100, 1, sysdate(), null, null);
