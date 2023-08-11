use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use salvo::{Request, Response};
use salvo::prelude::*;

use crate::model::banner::Banner;
use crate::RB;
use crate::vo::{err_result_page, handle_result, ok_result_page};
use crate::vo::banner_vo::*;

// 添加app轮播图
#[handler]
pub async fn banner_save(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<BannerSaveReq>().await.unwrap();
    log::info!("banner_save params: {:?}", &item);

    let banner = Banner {
        id: None,
        title: item.title,
        image_url: item.image_url,
        webview_url: item.webview_url,
        banner_sort: item.banner_sort,
        banner_status: item.banner_status,
        remark: item.remark,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),

    };

    let result = Banner::insert(&mut RB.clone(), &banner).await;

    res.render(Json(handle_result(result)))
}

// 删除app轮播图
#[handler]
pub async fn banner_delete(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<BannerDeleteReq>().await.unwrap();
    log::info!("banner_delete params: {:?}", &item);

    let result = Banner::delete_in_column(&mut RB.clone(), "id", &item.ids).await;

    res.render(Json(handle_result(result)))
}

// 更新app轮播图
#[handler]
pub async fn banner_update(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<BannerUpdateReq>().await.unwrap();
    log::info!("banner_update params: {:?}", &item);

    let banner = Banner {
        id: item.id,
        title: item.title,
        image_url: item.image_url,
        webview_url: item.webview_url,
        banner_sort: item.banner_sort,
        banner_status: item.banner_status,
        remark: item.remark,
        create_time: None,
        update_time: Some(DateTime::now()),

    };

    let result = Banner::update_by_column(&mut RB.clone(), &banner, "id").await;

    res.render(Json(handle_result(result)))
}

// 查询app轮播图
#[handler]
pub async fn banner_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<BannerListReq>().await.unwrap();
    log::info!("banner_list params: {:?}", &item);

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = Banner::select_page(&mut RB.clone(), page).await;

    match result {
        Ok(d) => {
            let total = d.total;

            let mut banner_list_data: Vec<BannerListData> = Vec::new();

            for x in d.records {
                banner_list_data.push(BannerListData {
                    id: x.id,
                    title: x.title,
                    image_url: x.image_url,
                    webview_url: x.webview_url,
                    banner_sort: x.banner_sort,
                    banner_status: x.banner_status,
                    remark: x.remark.unwrap_or_default(),
                    create_time: x.create_time.unwrap().0.to_string(),
                    update_time: x.update_time.unwrap().0.to_string(),

                })
            }

            res.render(Json(ok_result_page(banner_list_data, total)))
        }
        Err(err) => {
            res.render(Json(err_result_page(err.to_string())))
        }
    }

}
