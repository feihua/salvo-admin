create table interview_member
(
    id          bigint auto_increment comment '主键'
        primary key,
    phone       varchar(255)                       not null comment '手机号',
    name        varchar(255)                       not null comment '会员姓名',
    password    varchar(255)                       not null comment '密码',
    level       varchar(255)                       not null comment '会员等级',
    create_time datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime default CURRENT_TIMESTAMP not null comment '修改时间'
)
    comment '会员信息';

