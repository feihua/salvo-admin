// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// createTime：2024/12/25 10:01:11

use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;
use serde::{Deserialize, Serialize};
use crate::vo::system::sys_operate_log_vo::QueryOperateLogListReq;
/*
 *操作日志记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperateLog {
    pub id: Option<i64>,                  //日志主键
    pub title: Option<String>,            //模块标题
    pub business_type: Option<i8>,        //业务类型（0其它 1新增 2修改 3删除）
    pub method: Option<String>,           //方法名称
    pub request_method: Option<String>,   //请求方式
    pub operator_type: Option<i8>,        //操作类别（0其它 1后台用户 2手机端用户）
    pub operate_name: Option<String>,     //操作人员
    pub dept_name: Option<String>,        //部门名称
    pub operate_url: Option<String>,      //请求URL
    pub operate_ip: Option<String>,       //主机地址
    pub operate_location: Option<String>, //操作地点
    pub operate_param: Option<String>,    //请求参数
    pub json_result: Option<String>,      //返回参数
    pub status: Option<i8>,               //操作状态(0:异常,正常)
    pub error_msg: Option<String>,        //错误消息
    pub operate_time: Option<DateTime>,   //操作时间
    pub cost_time: Option<i64>,           //消耗时间
}

/*
 *操作日志记录基本操作
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
rbatis::crud!(OperateLog {}, "sys_operate_log");

/*
 *根据id查询操作日志记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(OperateLog{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_operate_log");

/*
 *分页查询操作日志记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(OperateLog{select_page() =>"
     if !sql.contains('count'):
       order by operate_time desc"
},"sys_operate_log");

/*
 *根据条件分页查询操作日志记录
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(OperateLog{select_page_by_name(
    req:&QueryOperateLogListReq) =>"
    where 1=1
     if req.title != '' && req.title != null:
       ` and title = #{req.title} `
     if req.business_type != 4:
       ` and business_type = #{req.business_type} `
     if req.method != '' && req.method != null:
       ` and method = #{req.method} `
     if req.request_method != '' && req.request_method != null:
       ` and request_method = #{req.request_method} `
     if req.operator_type != 3:
       ` and operator_type = #{req.operator_type} `
     if req.operate_name != '' && req.operate_name != null:
       ` and operate_name = #{req.operate_name} `
     if req.dept_name != '' && req.dept_name != null:
       ` and dept_name = #{req.dept_name} `
     if req.operate_url != '' && req.operate_url != null:
       ` and operate_url = #{req.operate_url} `
     if req.operate_ip != '' && req.operate_ip != null:
       ` and operate_ip = #{req.operate_ip} `
     if req.status != 2:
       ` and status = #{req.status} `
     if !sql.contains('count'):
       ` order by operate_time desc `"
},"sys_operate_log");


/*
 *清空操作日志
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[sql("truncate table sys_operate_log")]
pub async fn clean_operate_log(rb: &RBatis) -> Option<i64> {
    impled!()
}
