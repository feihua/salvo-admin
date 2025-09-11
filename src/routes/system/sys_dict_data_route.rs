use crate::handler::system::sys_dict_data_handler::*;
use salvo::Router;
/*
 *构建字典数据表路由
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
pub fn build_sys_dict_data_route() -> Router {
    Router::new()
        .push(Router::new().path("/system/dictData/addDictData").post(add_sys_dict_data))
        .push(Router::new().path("/system/dictData/deleteDictData").post(delete_sys_dict_data))
        .push(Router::new().path("/system/dictData/updateDictData").post(update_sys_dict_data))
        .push(Router::new().path("/system/dictData/updateDictDataStatus").post(update_sys_dict_data_status))
        .push(Router::new().path("/system/dictData/queryDictDataDetail").post(query_sys_dict_data_detail))
        .push(Router::new().path("/system/dictData/queryDictDataList").post(query_sys_dict_data_list))
    //记得在main.rs中的route()函数中添加构建字典数据表路由build_sys_dict_data_route()
}
