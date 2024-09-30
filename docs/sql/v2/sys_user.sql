create table sys_user
(
    user_id       bigint auto_increment comment '用户ID'
        primary key,
    tenant_id     varchar(20)  default '000000'          not null comment '租户编号',
    dept_id       bigint                                 not null comment '部门ID',
    user_name     varchar(30)                            not null comment '用户账号',
    nick_name     varchar(30)                            not null comment '用户昵称',
    user_type     varchar(10)  default 'sys_user'        not null comment '用户类型（sys_user系统用户）',
    email         varchar(50)  default ''                not null comment '用户邮箱',
    phone         varchar(11)  default ''                not null comment '手机号码',
    sex           char         default '0'               not null comment '用户性别（0男 1女 2未知）',
    avatar        varchar(200) default ''                not null comment '头像地址',
    password      varchar(100) default ''                not null comment '密码',
    status        char         default '0'               not null comment '帐号状态（0正常 1停用）',
    del_flag      char         default '0'               not null comment '删除标志（0代表存在 1代表删除）',
    login_os      varchar(64)  default ''                not null comment '最后登录操作系统',
    login_browser varchar(64)  default ''                not null comment '最后登录浏览器',
    login_ip      varchar(128) default ''                not null comment '最后登录IP',
    login_time    datetime     default CURRENT_TIMESTAMP not null comment '最后登录时间',
    remark        varchar(200) default ''                not null comment '备注',
    create_dept   bigint                                 not null comment '创建部门',
    create_by     bigint                                 not null comment '创建者',
    create_time   timestamp    default CURRENT_TIMESTAMP not null comment '创建时间',
    update_by     bigint                                 null comment '更新者',
    update_time   datetime                               null on update CURRENT_TIMESTAMP comment '更新时间'
)
    comment '用户信息表';




INSERT INTO sys_user (user_id, tenant_id, dept_id, user_name, nick_name, user_type, email, phone, sex, avatar, password, status, del_flag, login_os, login_browser, login_ip, login_time, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (1, '000000', 1, 'admin', '超级管理员', 'sys_user', '1002219331@qq.com', '18613030352', '0', 'https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png', '123456', '0', '0', 'windows11', 'Chrome', '127.0.0.1', sysdate(), '管理人员', 1, 1, sysdate(), null, null);
INSERT INTO sys_user (user_id, tenant_id, dept_id, user_name, nick_name, user_type, email, phone, sex, avatar, password, status, del_flag, login_os, login_browser, login_ip, login_time, remark, create_dept, create_by, create_time, update_by, update_time) VALUES (2, '000000', 1, 'test', '测试人员', 'sys_user', 'xxx@qq.com', '18613033333', '0', 'https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png', '123456', '0', '0', 'windows11', 'Chrome', '127.0.0.1', sysdate(), '测试人员', 1, 1, sysdate(), null, null);
