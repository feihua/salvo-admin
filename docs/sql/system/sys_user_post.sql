create table sys_user_post
(
    id          bigint auto_increment comment '主键'
        primary key,
    user_id bigint not null comment '用户id',
    post_id bigint not null comment '岗位id'
) comment = '用户与岗位关联表';


insert into sys_user_post(user_id, post_id) values ('1', '1');
insert into sys_user_post(user_id, post_id) values ('2', '2');
