use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::{PageRequest};
use salvo::{Request, Response};
use salvo::prelude::*;

use crate::model::interview_type::InterviewType;
use crate::RB;
use crate::vo::handle_result;
use crate::vo::type_vo::*;


// 添加题目类型
#[handler]
pub async fn type_save(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<TypeSaveReq>().await.unwrap();
    log::info!("type_save params: {:?}", &item);

    let interview_type = InterviewType {
        id: None,
        interview_code: item.interview_code,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
    
    };

    let result = InterviewType::insert(&mut RB.clone(), &interview_type).await;

    res.render(Json(handle_result(result)))
}

// 删除题目类型
#[handler]
pub async fn type_delete(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<TypeDeleteReq>().await.unwrap();
    log::info!("type_delete params: {:?}", &item);

    let result = InterviewType::delete_in_column(&mut RB.clone(), "id", &item.ids).await;

    res.render(Json(handle_result(result)))
}

// 更新题目类型
#[handler]
pub async fn type_update(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<TypeUpdateReq>().await.unwrap();
    log::info!("type_update params: {:?}", &item);

    let interview_type = InterviewType {
        id: Some(item.id),
        interview_code: item.interview_code,
        create_time: None,
        update_time: Some(DateTime::now()),
    
    };

    let result = InterviewType::update_by_column(&mut RB.clone(), &interview_type, "id").await;

    res.render(Json(handle_result(result)))
}

// 查询题目类型
#[handler]
pub async fn type_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<TypeListReq>().await.unwrap();
    log::info!("type_list params: {:?}", &item);

    let page=&PageRequest::new(item.page_no, item.page_size);
    let result = InterviewType::select_page(&mut RB.clone(), page).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;

            let mut type_list_res: Vec<TypeListData> = Vec::new();

            for x in d.records {
                type_list_res.push(TypeListData {
                        id: x.id.unwrap(),
                        interview_code: x.interview_code,
                        create_time: x.create_time.unwrap().0.to_string(),
                        update_time: x.update_time.unwrap().0.to_string(),
                    
                })
            }

            TypeListResp {
                msg: "successful".to_string(),
                code: 0,
                success: true,
                total,
                data: Some(type_list_res),
            }
        }
        Err(err) => {
            TypeListResp {
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
