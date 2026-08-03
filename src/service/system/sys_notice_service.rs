use crate::common::error::AppError;
use crate::common::error::AppError::BusinessError;
use crate::model::system::sys_notice_model::Notice;
use crate::vo::system::sys_notice_vo::*;
use crate::RB;
use rbatis::plugin::page::PageRequest;
use rbs::value;

pub struct NoticeService;

impl NoticeService {
    /*
     *添加通知公告
     *author：刘飞华
     *date：2025/01/08 13:51:14
     */
    pub async fn add_sys_notice(mut item: NoticeReq) -> Result<u64, AppError> {
        let rb = &mut RB.clone();

        let condition = value! {
            "notice_title": &item.notice_title,
        };

        if Notice::select_by_map(rb, condition).await?.len() > 0 {
            return Err(BusinessError("公告标题已存在"));
        }

        item.id = None;
        Notice::add(rb, &Notice::from(item)).await.map(|id| Ok(id.unwrap_or_default()))?
    }

    /*
     *删除通知公告
     *author：刘飞华
     *date：2025/01/08 13:51:14
     */
    pub async fn delete_sys_notice(item: DeleteNoticeReq) -> Result<u64, AppError> {
        let rb = &mut RB.clone();

        Notice::delete(rb, item.ids).await.map(|x| Ok(x))?
    }

    /*
     *更新通知公告
     *author：刘飞华
     *date：2025/01/08 13:51:14
     */
    pub async fn update_sys_notice(item: NoticeReq) -> Result<u64, AppError> {
        let rb = &mut RB.clone();

        let id = item.id;
        if id.is_none() {
            return Err(BusinessError("主键不能为空"));
        }

        if Notice::select_by_id(rb, id.unwrap_or_default()).await?.is_none() {
            return Err(BusinessError("通知公告不存在"));
        };

        let condition = value! {
            "notice_title": &item.notice_title,
            "id !=":id
        };

        if Notice::select_by_map(rb, condition).await?.len() > 0 {
            return Err(BusinessError("公告标题已存在"));
        }

        Notice::update(rb, &Notice::from(item), id.unwrap_or_default()).await.map(|x| Ok(x))?
    }

    /*
     *更新通知公告状态
     *author：刘飞华
     *date：2025/01/08 13:51:14
     */
    pub async fn update_sys_notice_status(item: UpdateNoticeStatusReq) -> Result<u64, AppError> {
        let rb = &mut RB.clone();
        Notice::update_status(rb, item.ids, item.status).await.map(|x| Ok(x))?
    }

    /*
     *查询通知公告详情
     *author：刘飞华
     *date：2025/01/08 13:51:14
     */
    pub async fn query_sys_notice_detail(item: QueryNoticeDetailReq) -> Result<NoticeResp, AppError> {
        match Notice::select_by_id(&mut RB.clone(), item.id).await? {
            None => Err(BusinessError("通知公告不存在")),
            Some(x) => Ok(x.into()),
        }
    }

    /*
     *查询通知公告列表
     *author：刘飞华
     *date：2025/01/08 13:51:14
     */
    pub async fn query_sys_notice_list(item: QueryNoticeListReq) -> Result<(Vec<NoticeResp>, u64), AppError> {
        let rb = &mut RB.clone();

        Notice::select_by_page(rb, &PageRequest::from(&item), &item)
            .await
            .map(|x| Ok((x.records.into_iter().map(|x| x.into()).collect::<Vec<NoticeResp>>(), x.total)))?
    }
}
