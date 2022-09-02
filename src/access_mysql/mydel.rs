

/// 通过id，删除数据 ，返回 sql 语句。
/// Delete one data by id (default).
/// ```
/// let sql = mydel!("feedback", 2);  // id = 2
/// 
/// 执行 do
/// myconn.run(sql);
/// 
/// ```
/// 通过指定字段的值，删除数据 ，返回 sql 语句。
/// Delete one data by filed value.
/// ```
/// let sql = mydel!("feedback", {"uid": 12});
/// 
/// 执行 do
/// myconn.run(sql);
/// 
/// ```
#[macro_export] 
macro_rules! mydel {
    ($t:expr, {$k:tt: $v:expr}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let keys = $k.to_string();
            let temp_v = $v.clone();
            let v_type = type_of($v);
            let values = match v_type {
                "&str" => {
                    let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                    v_r = v_r.replace("\"", "\\\"");
                    "\"".to_string() + &v_r + "\""
                },
                "alloc::string::String" => {
                    let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                    v_r = v_r.replace("\"", "\\\"");
                    "\"".to_string() + &v_r + "\""
                },
                _ => {
                    temp_v.to_string() + ""
                }
            };
            
            let sql: String = "DELETE FROM ".to_string() + $t + " WHERE " + keys.as_str() + "=" + values.as_str();

            sql
        }
    };

    ($t:expr, $v: expr) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let temp_v = $v.clone();
            let v_type = type_of($v);
            let values = match v_type {
                "&str" => {
                    let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                    v_r = v_r.replace("\"", "\\\"");
                    "\"".to_string() + &v_r + "\""
                },
                "alloc::string::String" => {
                    let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                    v_r = v_r.replace("\"", "\\\"");
                    "\"".to_string() + &v_r + "\""
                },
                _ => {
                    temp_v.to_string() + ""
                }
            };

            let sql: String = "DELETE FROM ".to_string() + $t + " WHERE id=" + values.as_str();

            sql
        }
    };

   
}
