use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use salvo::{Request, Response};
use salvo::prelude::*;

use crate::model::types::Types;
use crate::RB;
use crate::vo::{err_result_page, handle_result, ok_result_page};
use crate::vo::types_vo::*;

// 添加题目类型
#[handler]
pub async fn types_save(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<TypesSaveReq>().await.unwrap();
    log::info!("types_save params: {:?}", &item);

    let types = Types {
        id: None,
        interview_code: item.interview_code,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),

    };

    let result = Types::insert(&mut RB.clone(), &types).await;

    res.render(Json(handle_result(result)))
}

// 删除题目类型
#[handler]
pub async fn types_delete(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<TypesDeleteReq>().await.unwrap();
    log::info!("types_delete params: {:?}", &item);

    let result = Types::delete_in_column(&mut RB.clone(), "id", &item.ids).await;

    res.render(Json(handle_result(result)))
}

// 更新题目类型
#[handler]
pub async fn types_update(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<TypesUpdateReq>().await.unwrap();
    log::info!("types_update params: {:?}", &item);

    let types = Types {
        id: item.id,
        interview_code: item.interview_code,
        create_time: None,
        update_time: Some(DateTime::now()),

    };

    let result = Types::update_by_column(&mut RB.clone(), &types, "id").await;

    res.render(Json(handle_result(result)))
}

// 查询题目类型
#[handler]
pub async fn types_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<TypesListReq>().await.unwrap();
    log::info!("types_list params: {:?}", &item);

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = Types::select_page(&mut RB.clone(), page).await;

    match result {
        Ok(d) => {
            let total = d.total;

            let mut types_list_data: Vec<TypesListData> = Vec::new();

            for x in d.records {
                types_list_data.push(TypesListData {
                    id: x.id,
                    interview_code: x.interview_code,
                    create_time: x.create_time.unwrap().0.to_string(),
                    update_time: x.update_time.unwrap().0.to_string(),

                })
            }

            res.render(Json(ok_result_page(types_list_data, total)))
        }
        Err(err) => {
            res.render(Json(err_result_page(err.to_string())))
        }
    }
}
