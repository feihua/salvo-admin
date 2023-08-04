create table interview_banner
(
    id            bigint auto_increment comment '主键'
        primary key,
    title         varchar(50)                        not null comment '标题',
    image_url     varchar(50)                        not null comment '图片url',
    webview_url   varchar(50)                        not null comment '轮播图详情url',
    banner_sort   int      default 1                 not null comment '排序',
    banner_status int      default 1                 not null comment '状态',
    remark        varchar(255)                       not null comment '备注',
    create_time   datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time   datetime default CURRENT_TIMESTAMP not null comment '修改时间'
)
    comment 'app轮播图';

