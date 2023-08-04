create table interview_member
(
    id          bigint auto_increment comment '主键'
        primary key,
    phone       varchar(255)                       not null comment '手机号',
    name        varchar(255)                       not null comment '会员姓名',
    password    varchar(255)                       not null comment '密码',
    level       varchar(255)                       not null comment '会员等级',
    create_time datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间'
)
    comment '会员信息';

INSERT INTO rustdb.interview_member (id, phone, name, password, level, create_time, update_time) VALUES (1, '18613030352', 'koobe', '123456', '1', '2023-08-10 16:14:12', '2023-08-03 16:15:30');
