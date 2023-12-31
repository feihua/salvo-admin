create table interview_title
(
    id             bigint auto_increment comment '主键'
        primary key,
    title          varchar(255)                       not null comment '标题',
    content        varchar(255)                       not null comment '答案',
    interview_type varchar(255)                       not null comment '类型',
    create_time    datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time    datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间'
)
    comment '面试题目';

INSERT INTO interview_title (id, title, content, interview_type, create_time, update_time) VALUES (1, '测试', '测试', 'java', '2023-08-03 16:28:57', '2023-08-03 16:30:06');
