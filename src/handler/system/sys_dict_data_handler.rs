// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// date：2025/01/08 13:51:14

use crate::common::error::{AppError, AppResult};
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_dict_data_model::DictData;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_dict_data_vo::*;
use crate::RB;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use salvo::prelude::*;
use salvo::{Request, Response};

/*
 *添加字典数据表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn add_sys_dict_data(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<AddDictDataReq>().await?;

    log::info!("add sys_dict_data params: {:?}", &item);

    let rb = &mut RB.clone();
    if DictData::select_by_dict_label(rb, &item.dict_type, &item.dict_label).await?.is_some() {
        return Err(AppError::BusinessError("字典标签已存在"));
    }

    if DictData::select_by_dict_value(rb, &item.dict_type, &item.dict_value).await?.is_some() {
        return Err(AppError::BusinessError("字典键值已存在"));
    }

    let sys_dict_data = DictData {
        dict_code: None,                         //字典编码
        dict_sort: item.dict_sort,               //字典排序
        dict_label: item.dict_label,             //字典标签
        dict_value: item.dict_value,             //字典键值
        dict_type: item.dict_type,               //字典类型
        css_class: item.css_class,               //样式属性（其他样式扩展）
        list_class: item.list_class,             //格回显样式
        is_default: item.is_default,             //是否默认（Y是 N否）
        status: item.status,                     //状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //修改时间
    };

    DictData::insert(rb, &sys_dict_data).await?;
    ok_result(res)
}

/*
 *删除字典数据表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn delete_sys_dict_data(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<DeleteDictDataReq>().await?;
    log::info!("delete sys_dict_data params: {:?}", &item);

    let rb = &mut RB.clone();

    DictData::delete_by_map(rb, value! {"dict_code": &item.ids}).await?;
    ok_result(res)
}

/*
 *更新字典数据表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn update_sys_dict_data(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<UpdateDictDataReq>().await?;
    log::info!("update sys_dict_data params: {:?}", &item);

    let rb = &mut RB.clone();

    if DictData::select_by_id(rb, &item.dict_code).await?.is_none() {
        return Err(AppError::BusinessError("字典数据不存在"));
    }

    if let Some(x) = DictData::select_by_dict_label(rb, &item.dict_type, &item.dict_label).await? {
        if x.dict_code.unwrap_or_default() != item.dict_code {
            return Err(AppError::BusinessError("字典标签已存在"));
        }
    }

    if let Some(x) = DictData::select_by_dict_value(rb, &item.dict_type, &item.dict_value).await? {
        if x.dict_code.unwrap_or_default() != item.dict_code {
            return Err(AppError::BusinessError("字典键值已存在"));
        }
    }

    let sys_dict_data = DictData {
        dict_code: Some(item.dict_code),         //字典编码
        dict_sort: item.dict_sort,               //字典排序
        dict_label: item.dict_label,             //字典标签
        dict_value: item.dict_value,             //字典键值
        dict_type: item.dict_type,               //字典类型
        css_class: item.css_class,               //样式属性（其他样式扩展）
        list_class: item.list_class,             //格回显样式
        is_default: item.is_default,             //是否默认（Y是 N否）
        status: item.status,                     //状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //修改时间
    };

    DictData::update_by_map(rb, &sys_dict_data, value! {"dict_code": &item.dict_code}).await?;
    ok_result(res)
}

/*
 *更新字典数据表状态
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn update_sys_dict_data_status(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<UpdateDictDataStatusReq>().await?;
    log::info!("update sys_dict_data_status params: {:?}", &item);

    let update_sql = format!(
        "update sys_dict_data set status = ? where dict_code in ({})",
        item.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
    );

    let mut param = vec![value!(item.status)];
    param.extend(item.ids.iter().map(|&id| value!(id)));

    let _ = &mut RB.clone().exec(&update_sql, param).await?;
    ok_result(res)
}

/*
 *查询字典数据表详情
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_dict_data_detail(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<QueryDictDataDetailReq>().await?;
    log::info!("query sys_dict_data_detail params: {:?}", &item);

    match DictData::select_by_id(&mut RB.clone(), &item.id).await? {
        None => Err(AppError::BusinessError("字典数据不存在")),
        Some(x) => {
            let sys_dict_data = QueryDictDataDetailResp {
                dict_code: x.dict_code.unwrap_or_default(), //字典编码
                dict_sort: x.dict_sort,                     //字典排序
                dict_label: x.dict_label,                   //字典标签
                dict_value: x.dict_value,                   //字典键值
                dict_type: x.dict_type,                     //字典类型
                css_class: x.css_class,                     //样式属性（其他样式扩展）
                list_class: x.list_class,                   //格回显样式
                is_default: x.is_default,                   //是否默认（Y是 N否）
                status: x.status,                           //状态（0：停用，1:正常）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            };

            ok_result_data(res, sys_dict_data)
        }
    }
}

/*
 *查询字典数据表列表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_dict_data_list(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let item = req.parse_json::<QueryDictDataListReq>().await?;
    log::info!("query sys_dict_data_list params: {:?}", &item);

    let page = &PageRequest::new(item.page_no, item.page_size);

    let p = DictData::select_dict_data_list(&mut RB.clone(), page, &item).await?;

    let total = p.total;
    let list = p
        .records
        .into_iter()
        .map(|x| {
            DictDataListDataResp {
                dict_code: x.dict_code.unwrap_or_default(), //字典编码
                dict_sort: x.dict_sort,                     //字典排序
                dict_label: x.dict_label,                   //字典标签
                dict_value: x.dict_value,                   //字典键值
                dict_type: x.dict_type,                     //字典类型
                css_class: x.css_class,                     //样式属性（其他样式扩展）
                list_class: x.list_class,                   //格回显样式
                is_default: x.is_default,                   //是否默认（Y是 N否）
                status: x.status,                           //状态（0：停用，1:正常）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            }
        })
        .collect::<Vec<DictDataListDataResp>>();

    ok_result_page(res, list, total)
}
