#![allow(non_snake_case)]

// 多参数 handler trait 宏实现 + 注册/调用模板（支持 async/serde/HashMap 动态存储）
use crate::task::Task;
use crate::types::Args;
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
            let args = match Args::from_json_value(args_json).await {
                Ok(a) => a,
                Err(e) => {
                    println!("参数反序列化失败, {:?}", e);
                    return serde_json::Value::Null;
                }
            };
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
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_handler_map_usage() {
        let mut map: HashMap<String, BoxedHandler> = HashMap::new();

        // 直接 async fn(x, y) -> u32
        async fn add(task: Task, args: Args<(i32, i32, i32)>) -> i32 {
            let (x, y, z) = args.into_inner();
            println!("测试");
            println!("add被调用, {}, {}, {}", x, y, z);
            x + y + z
        }
        map.insert("add".to_string(), make_handler(add));
        let fut = map["add"](serde_json::json!({"args":[1, 2, 3]}));
        let result = fut.await;
        assert_eq!(result, serde_json::json!(6i32));
        println!("succes测试");
    }
}
