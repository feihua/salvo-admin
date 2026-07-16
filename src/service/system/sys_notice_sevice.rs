use crate::common::error::{AppError, AppResult, AppResultPage};
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_notice_model::Notice;
use crate::vo::system::sys_notice_vo::*;
use crate::RB;
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::DateTime;
use rbs::value;

pub struct NoticeService;

impl NoticeService {
    /*
     *添加通知公告
     *author：刘飞华
     *date：2025/01/08 13:51:14
     */
    pub async fn add_sys_notice(mut item: NoticeReq) -> AppResult<String> {
        let rb = &mut RB.clone();

        let condition = value! {
            "notice_title": &item.notice_title,
        };

        if Notice::select_by_map(rb, condition).await?.len() > 0 {
            return Err(AppError::BusinessError("公告标题已存在"));
        }

        item.id = None;
        Notice::insert(rb, &Notice::from(item)).await.map(|_| ok_result())?
    }

    /*
     *删除通知公告
     *author：刘飞华
     *date：2025/01/08 13:51:14
     */
    pub async fn delete_sys_notice(item: DeleteNoticeReq) -> AppResult<String> {
        let rb = &mut RB.clone();

        Notice::delete_by_map(rb, value! {"id": item.ids}).await.map(|_| ok_result())?
    }

    /*
     *更新通知公告
     *author：刘飞华
     *date：2025/01/08 13:51:14
     */
    pub async fn update_sys_notice(item: NoticeReq) -> AppResult<String> {
        let rb = &mut RB.clone();

        let id = item.id;
        if id.is_none() {
            return Err(AppError::BusinessError("主键不能为空"));
        }

        if Notice::select_by_map(rb, value! {"id":id}).await?.len() == 0 {
            return Err(AppError::BusinessError("通知公告不存在"));
        };

        let condition = value! {
            "notice_title": &item.notice_title,
            "id !=":id
        };

        if Notice::select_by_map(rb, condition).await?.len() > 0 {
            return Err(AppError::BusinessError("公告标题已存在"));
        }

        Notice::update_by_map(rb, &Notice::from(item), value! {"id": id}).await.map(|_| ok_result())?
    }

    /*
     *更新通知公告状态
     *author：刘飞华
     *date：2025/01/08 13:51:14
     */
    pub async fn update_sys_notice_status(item: UpdateNoticeStatusReq) -> AppResult<String> {
        let update_sql = format!(
            "update sys_notice set status = ? ,update_time = ? where id in ({})",
            item.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
        );

        let mut param = vec![value!(item.status), value!(DateTime::now())];
        param.extend(item.ids.iter().map(|&id| value!(id)));

        RB.clone().exec(&update_sql, param).await.map(|_| ok_result())?
    }

    /*
     *查询通知公告详情
     *author：刘飞华
     *date：2025/01/08 13:51:14
     */
    pub async fn query_sys_notice_detail(item: QueryNoticeDetailReq) -> AppResult<NoticeResp> {
        match Notice::select_by_id(&mut RB.clone(), &item.id).await? {
            None => Err(AppError::BusinessError("通知公告不存在")),
            Some(x) => ok_result_data(x.into()),
        }
    }

    /*
     *查询通知公告列表
     *author：刘飞华
     *date：2025/01/08 13:51:14
     */
    pub async fn query_sys_notice_list(item: QueryNoticeListReq) -> AppResultPage<Vec<NoticeResp>> {
        let rb = &mut RB.clone();

        Notice::select_by_page(rb, &PageRequest::from(&item), &item)
            .await
            .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<NoticeResp>>(), x.total))?
    }
}
