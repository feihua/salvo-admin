create table sys_dept
(
    dept_id       bigint auto_increment comment '部门id'
        primary key,
    tenant_id     varchar(20)  default '000000'          not null comment '租户编号',
    parent_id     bigint       default 0                 not null comment '父部门id',
    parent_ids    varchar(500) default ''                not null comment '祖级列表',
    dept_name     varchar(30)  default ''                not null comment '部门名称',
    dept_category varchar(100) default ''                not null comment '部门类别编码',
    order_num     int          default 0                 not null comment '显示顺序',
    leader        bigint                                 not null comment '负责人',
    phone         varchar(11)  default ''                not null comment '联系电话',
    email         varchar(50)  default ''                not null comment '邮箱',
    status        char         default '0'               not null comment '部门状态（0正常 1停用）',
    del_flag      char         default '0'               not null comment '删除标志（0代表存在 2代表删除）',
    create_dept   bigint                                 not null comment '创建部门',
    create_by     bigint                                 not null comment '创建者',
    create_time   timestamp    default CURRENT_TIMESTAMP not null comment '创建时间',
    update_by     bigint                                 null comment '更新者',
    update_time   datetime                               null on update CURRENT_TIMESTAMP comment '更新时间'
)
    comment '部门表';



INSERT INTO sys_dept (dept_id, tenant_id, parent_id, parent_ids, dept_name, dept_category, order_num, leader, phone, email, status, del_flag, create_dept, create_by, create_time, update_by, update_time) VALUES (1, '000000', 0, '0', 'XXX科技', '', 0, 0, '18618888888', 'xxx@qq.com', '0', '0', 1, 1, sysdate(), null, null);
INSERT INTO sys_dept (dept_id, tenant_id, parent_id, parent_ids, dept_name, dept_category, order_num, leader, phone, email, status, del_flag, create_dept, create_by, create_time, update_by, update_time) VALUES (2, '000000', 1, '0,1', '深圳总公司', '', 1, 0, '18618888888', 'xxx@qq.com', '0', '0', 1, 1, sysdate(), null, null);
INSERT INTO sys_dept (dept_id, tenant_id, parent_id, parent_ids, dept_name, dept_category, order_num, leader, phone, email, status, del_flag, create_dept, create_by, create_time, update_by, update_time) VALUES (3, '000000', 1, '0,1', '长沙分公司', '', 2, 0, '18618888888', 'xxx@qq.com', '0', '0', 1, 1, sysdate(), null, null);
INSERT INTO sys_dept (dept_id, tenant_id, parent_id, parent_ids, dept_name, dept_category, order_num, leader, phone, email, status, del_flag, create_dept, create_by, create_time, update_by, update_time) VALUES (4, '000000', 2, '0,1,2', '研发部门', '', 1, 0, '18618888888', 'xxx@qq.com', '0', '0', 1, 1, sysdate(), null, null);
INSERT INTO sys_dept (dept_id, tenant_id, parent_id, parent_ids, dept_name, dept_category, order_num, leader, phone, email, status, del_flag, create_dept, create_by, create_time, update_by, update_time) VALUES (5, '000000', 2, '0,1,2', '市场部门', '', 2, 0, '18618888888', 'xxx@qq.com', '0', '0', 1, 1, sysdate(), null, null);
INSERT INTO sys_dept (dept_id, tenant_id, parent_id, parent_ids, dept_name, dept_category, order_num, leader, phone, email, status, del_flag, create_dept, create_by, create_time, update_by, update_time) VALUES (6, '000000', 2, '0,1,2', '测试部门', '', 3, 0, '18618888888', 'xxx@qq.com', '0', '0', 1, 1, sysdate(), null, null);
INSERT INTO sys_dept (dept_id, tenant_id, parent_id, parent_ids, dept_name, dept_category, order_num, leader, phone, email, status, del_flag, create_dept, create_by, create_time, update_by, update_time) VALUES (7, '000000', 2, '0,1,2', '财务部门', '', 4, 0, '18618888888', 'xxx@qq.com', '0', '0', 1, 1, sysdate(), null, null);
INSERT INTO sys_dept (dept_id, tenant_id, parent_id, parent_ids, dept_name, dept_category, order_num, leader, phone, email, status, del_flag, create_dept, create_by, create_time, update_by, update_time) VALUES (8, '000000', 2, '0,1,2', '运维部门', '', 5, 0, '18618888888', 'xxx@qq.com', '0', '0', 1, 1, sysdate(), null, null);
INSERT INTO sys_dept (dept_id, tenant_id, parent_id, parent_ids, dept_name, dept_category, order_num, leader, phone, email, status, del_flag, create_dept, create_by, create_time, update_by, update_time) VALUES (9, '000000', 3, '0,1,3', '市场部门', '', 1, 0, '18618888888', 'xxx@qq.com', '0', '0', 1, 1, sysdate(), null, null);
INSERT INTO sys_dept (dept_id, tenant_id, parent_id, parent_ids, dept_name, dept_category, order_num, leader, phone, email, status, del_flag, create_dept, create_by, create_time, update_by, update_time) VALUES (10, '000000', 3, '0,1,3', '财务部门', '', 2, 0, '18618888888', 'xxx@qq.com', '0', '0', 1, 1, sysdate(), null, null);


