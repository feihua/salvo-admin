create table interview_type
(
    id             bigint auto_increment comment '主键'
        primary key,
    interview_code varchar(255)                       not null comment '题目类型',
    create_time    datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time    datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间'
)
    comment '题目类型';

