create table interview_banner
(
    id            bigint auto_increment comment '主键'
        primary key,
    title         varchar(50)                        not null comment '标题',
    image_url     varchar(50)                        not null comment '图片url',
    webview_url   varchar(50)                        not null comment '轮播图详情url',
    banner_sort   int      default 1                 not null comment '排序',
    banner_status int      default 1                 not null comment '状态',
    remark        varchar(255)                       null comment '备注',
    create_time   datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time   datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间'
)
    comment 'app轮播图';

INSERT INTO interview_banner (id, title, image_url, webview_url, banner_sort, banner_status, remark, create_time, update_time) VALUES (1, '1', '1', '1', 1, 1, '1', '2023-08-03 15:02:48', '2023-08-03 15:02:51');
INSERT INTO interview_banner (id, title, image_url, webview_url, banner_sort, banner_status, remark, create_time, update_time) VALUES (2, 'test', 'https://www.baidu.com', 'https://www.baidu.com', 1, 1, '测试', '2023-08-03 15:04:44', '2023-08-03 15:39:29');
