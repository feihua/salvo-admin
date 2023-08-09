use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use salvo::{Request, Response};
use salvo::prelude::*;

use crate::model::title::Title;
use crate::RB;
use crate::vo::handle_result;
use crate::vo::title_vo::*;

// 添加面试题目
#[handler]
pub async fn title_save(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<TitleSaveReq>().await.unwrap();
    log::info!("title_save params: {:?}", &item);

    let title = Title {
        id: None,
        title: item.title,
        content: item.content,
        interview_type: item.interview_type,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
    
    };

    let result = Title::insert(&mut RB.clone(), &title).await;

    res.render(Json(handle_result(result)))
}

// 删除面试题目
#[handler]
pub async fn title_delete(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<TitleDeleteReq>().await.unwrap();
    log::info!("title_delete params: {:?}", &item);

    let result = Title::delete_in_column(&mut RB.clone(), "id", &item.ids).await;

    res.render(Json(handle_result(result)))
}

// 更新面试题目
#[handler]
pub async fn title_update(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<TitleUpdateReq>().await.unwrap();
    log::info!("title_update params: {:?}", &item);

    let title = Title {
        id: Some(item.id),
        title: item.title,
        content: item.content,
        interview_type: item.interview_type,
        create_time: None,
        update_time: Some(DateTime::now()),
    
    };

    let result = Title::update_by_column(&mut RB.clone(), &title, "id").await;

    res.render(Json(handle_result(result)))
}

// 查询面试题目
#[handler]
pub async fn title_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<TitleListReq>().await.unwrap();
    log::info!("title_list params: {:?}", &item);

    let page=&PageRequest::new(item.page_no, item.page_size);
    let result = Title::select_page(&mut RB.clone(), page).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;

            let mut title_list_res: Vec<TitleListData> = Vec::new();

            for x in d.records {
                title_list_res.push(TitleListData {
                        id: x.id.unwrap(),
                        title: x.title,
                        content: x.content,
                        interview_type: x.interview_type,
                        create_time: x.create_time.unwrap().0.to_string(),
                        update_time: x.update_time.unwrap().0.to_string(),
                    
                })
            }

            TitleListResp {
                msg: "successful".to_string(),
                code: 0,
                success: true,
                total,
                data: Some(title_list_res),
            }
        }
        Err(err) => {
            TitleListResp {
                msg: err.to_string(),
                code: 1,
                success: false,
                total: 0,
                data: None,
            }
        }
    };

    res.render(Json(resp))
}
