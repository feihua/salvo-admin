use crate::handler::system::sys_dict_type_handler::*;
use salvo::Router;
/*
 *构建字典类型表路由
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
pub fn build_sys_dict_type_route() -> Router {
    Router::new()
        .push(Router::new().path("/system/dictType/addDictType").post(add_sys_dict_type))
        .push(Router::new().path("/system/dictType/deleteDictType").post(delete_sys_dict_type))
        .push(Router::new().path("/system/dictType/updateDictType").post(update_sys_dict_type))
        .push(Router::new().path("/system/dictType/updateDictTypeStatus").post(update_sys_dict_type_status))
        .push(Router::new().path("/system/dictType/queryDictTypeDetail").post(query_sys_dict_type_detail))
        .push(Router::new().path("/system/dictType/queryDictTypeList").post(query_sys_dict_type_list))
    //记得在main.rs中的route()函数中添加构建字典类型表路由build_sys_dict_type_route()
}
