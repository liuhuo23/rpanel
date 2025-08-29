#![allow(non_snake_case)]

// 多参数 handler trait 宏实现 + 注册/调用模板（支持 async/serde/HashMap 动态存储）
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

// 1. Handler trait 及宏批量实现（支持 1~10 元组参数）
pub trait Handler<Args, Fut> {
    fn call(&self, args: Args) -> Fut;
}

macro_rules! impl_handler {
	($( $len:literal : ( $($t:ident),+ ) ),+) => {
		$(
			impl<Func, Fut, Res, $($t),+> Handler<($($t,)+), Fut> for Func
			where
				Func: Fn($($t),+) -> Fut,
				Fut: Future<Output = Res>,
				$( $t: Send + 'static, )+
			{
				fn call(&self, args: ($($t,)+)) -> Fut {
					let ($($t,)+) = args;
					(self)($($t),+)
				}
			}
		)+
	};
}

impl_handler! {
    1: (A),
    2: (A, B),
    3: (A, B, C),
    4: (A, B, C, D),
    5: (A, B, C, D, E),
    6: (A, B, C, D, E, F),
    7: (A, B, C, D, E, F, G),
    8: (A, B, C, D, E, F, G, H),
    9: (A, B, C, D, E, F, G, H, I),
    10:(A, B, C, D, E, F, G, H, I, J)
}
// If you need Handler for Args<T>, implement it manually here.
// impl<Func, Fut, Res, T> Handler<(Args<T>,), Fut> for Func
// where
//     Func: Fn(Args<T>) -> Fut,
//     Fut: Future<Output = Res>,
//     T: Send + 'static,
// {
//     fn call(&self, args: (Args<T>,)) -> Fut {
//         let (args,) = args;
//         (self)(args)
//     }
// }

pub trait FromJson: Sized {
    type Error;
    type Future: Future<Output = Result<Self, Self::Error>> + Send;

    fn from_json_value(val: serde_json::Value) -> Self::Future;
}

// 2. 通用 handler object-safe 类型和注册工具
pub type BoxedHandler = Box<
    dyn Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = serde_json::Value> + Send>>
        + Send
        + Sync,
>;

pub fn make_handler<Func, Args, Fut, Res>(func: Func) -> BoxedHandler
where
    Func: Handler<Args, Fut> + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send + 'static,
    Args: FromJson + Send + 'static,
    Res: serde::Serialize + Send + 'static,
    <Args as FromJson>::Error: std::fmt::Debug,
{
    let func = Arc::new(func);
    Box::new(move |args_json: serde_json::Value| {
        let func = func.clone();
        let fut = async move {
            // 这儿判断是数组还是对象，决定反序列化为元组还是结构体
            if args_json.is_array() == false {
                args_json = serde_json::Value::Array(vec![args_json]);
            }
            let args = Args::from_json_value(args_json)
                .await
                .expect("参数反序列化失败");
            let res = (func).call(args).await;
            serde_json::to_value(res).unwrap_or(serde_json::Value::Null)
        };
        Box::pin(fut)
    })
}

// 3. 测试用例：多参数元组/结构体 handler 注册与调用
#[cfg(test)]
mod test_handler_map {
    use super::*;
    use crate::task::Task;
    use crate::types::Args;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_handler_map_usage() {
        let mut map: HashMap<String, BoxedHandler> = HashMap::new();

        // 直接 async fn(x, y) -> u32
        async fn add(task: Task, args: Args<(i32, i32, i32)>) -> i32 {
            let (x, y, z) = args.into_inner();
            println!("add被调用, {}, {}, {}", x, y, z);
            x + y + z
        }
        map.insert("add".to_string(), make_handler(add));
        let fut = map["add"](serde_json::json!({"args":[1, 2, 3]}));
        let result = fut.await;
        assert_eq!(result, serde_json::json!(6i32));
        println!("succes测试");

        // 结构体参数 handler
        #[derive(serde::Deserialize)]
        struct Args {
            x: u32,
            y: u32,
        }
        async fn add_struct(args: Args) -> u32 {
            println!("add_struct被调用, {}, {}", args.x, args.y);
            args.x + args.y
        }
        map.insert("add_struct".to_string(), make_handler(add_struct));
        let fut = map["add_struct"](serde_json::json!({"x": 2, "y": 3}));
        let result = fut.await;
        assert_eq!(result, serde_json::json!(5u32));

        async fn struct_args(a: i32, b: i32, args: Args) -> i32 {
            println!("struct_args被调用, {}, {}, {}, {}", a, b, args.x, args.y);
            a + b + (args.x as i32) + (args.y as i32)
        }
        map.insert("struct_args".to_string(), make_handler(struct_args));
        let fut = map["struct_args"](serde_json::json!([{"x": 3, "y": 4}, 1, 2]));
        let result = fut.await;
        assert_eq!(result, serde_json::json!(10i32));
    }
}
