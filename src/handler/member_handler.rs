use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use salvo::{Request, Response};
use salvo::prelude::*;

use crate::model::member::Member;
use crate::RB;
use crate::vo::handle_result;
use crate::vo::member_vo::*;

// 添加会员信息
#[handler]
pub async fn member_save(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<MemberSaveReq>().await.unwrap();
    log::info!("member_save params: {:?}", &item);

    let member = Member {
        id: None,
        phone: item.phone,
        name: item.name,
        password: item.password,
        level: item.level,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
    
    };

    let result = Member::insert(&mut RB.clone(), &member).await;

    res.render(Json(handle_result(result)))
}

// 删除会员信息
#[handler]
pub async fn member_delete(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<MemberDeleteReq>().await.unwrap();
    log::info!("member_delete params: {:?}", &item);

    let result = Member::delete_in_column(&mut RB.clone(), "id", &item.ids).await;

    res.render(Json(handle_result(result)))
}

// 更新会员信息
#[handler]
pub async fn member_update(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<MemberUpdateReq>().await.unwrap();
    log::info!("member_update params: {:?}", &item);

    let member = Member {
        id: Some(item.id),
        phone: item.phone,
        name: item.name,
        password: item.password,
        level: item.level,
        create_time: None,
        update_time: Some(DateTime::now()),
    
    };

    let result = Member::update_by_column(&mut RB.clone(), &member, "id").await;

    res.render(Json(handle_result(result)))
}

// 查询会员信息
#[handler]
pub async fn member_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<MemberListReq>().await.unwrap();
    log::info!("member_list params: {:?}", &item);

    let page=&PageRequest::new(item.page_no, item.page_size);
    let result = Member::select_page(&mut RB.clone(), page).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;

            let mut member_list_res: Vec<MemberListData> = Vec::new();

            for x in d.records {
                member_list_res.push(MemberListData {
                        id: x.id.unwrap(),
                        phone: x.phone,
                        name: x.name,
                        password: x.password,
                        level: x.level,
                        create_time: x.create_time.unwrap().0.to_string(),
                        update_time: x.update_time.unwrap().0.to_string(),
                    
                })
            }

            MemberListResp {
                msg: "successful".to_string(),
                code: 0,
                success: true,
                total,
                data: Some(member_list_res),
            }
        }
        Err(err) => {
            MemberListResp {
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
