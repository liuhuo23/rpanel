use std::ops::{Deref, DerefMut};

use crate::handler::FromJson;
use crate::task::Task;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Serialize, Deserialize)]
pub struct Args<T>(pub T);
impl<T> Args<T>
where
    T: DeserializeOwned + Send + 'static,
{
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Args<T>
where
    T: DeserializeOwned + Send + 'static,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Args<T>
where
    T: DeserializeOwned + Send + 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: DeserializeOwned> From<T> for Args<T> {
    fn from(t: T) -> Args<T> {
        Args(t)
    }
}

impl<T> FromJson for Args<T>
where
    T: DeserializeOwned + Send + Default + 'static,
{
    type Error = String;
    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>> + Send>>;
    fn from_json_value(val: serde_json::Value) -> Self::Future {
        Box::pin(async move {
            match val.get("args") {
                Some(args) => {
                    let inner =
                        serde_json::from_value::<T>(args.clone()).map_err(|e| e.to_string())?;
                    Ok(Args(inner))
                }
                None => Ok(Args(T::default())),
            }
        })
    }
}

// Implement FromJson for (Args<T>,) so make_handler works with single Args<T> argument
impl<T> FromJson for (Args<T>,)
where
    Args<T>: FromJson,
{
    type Error = <Args<T> as FromJson>::Error;
    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>> + Send>>;

    fn from_json_value(val: serde_json::Value) -> Self::Future {
        Box::pin(async move {
            let args = Args::<T>::from_json_value(val).await?;
            Ok((args,))
        })
    }
}

// 支持 n 元组的 FromJson 宏实现（最多 5 元组，可自行扩展）
macro_rules! impl_from_json_tuple_n {
    // 2 元组
    ($name1:ident : $ty1:ty, $name2:ident : $ty2:ty) => {
        impl<T> FromJson for ($ty1, $ty2)
        where
            $ty1: for<'de> serde::Deserialize<'de>,
            $ty2: FromJson,
            <$ty2 as FromJson>::Error: std::fmt::Debug,
        {
            type Error = String;
            type Future = std::pin::Pin<
                Box<dyn std::future::Future<Output = Result<Self, Self::Error>> + Send>,
            >;
            fn from_json_value(val: serde_json::Value) -> Self::Future {
                Box::pin(async move {
                    let $name2 = <$ty2>::from_json_value(val.clone()).await;
                    let $name1 = serde_json::from_value::<$ty1>(val).map_err(|e| e.to_string());
                    match $name1 {
                        Err(e) => {
                            println!(concat!(stringify!($ty1), " 反序列化失败, {:?}"), e);
                            return Err(e);
                        }
                        Ok($name1) => match $name2 {
                            Ok($name2) => Ok(($name1, $name2)),
                            Err(e) => {
                                Err(format!(concat!(stringify!($ty2), " 反序列化失败, {:?}"), e))
                            }
                        },
                    }
                })
            }
        }
    };
    // 3 元组
    ($name1:ident : $ty1:ty, $name2:ident : $ty2:ty, $name3:ident : $ty3:ty) => {
        impl<T> FromJson for ($ty1, $ty2, $ty3)
        where
            $ty1: for<'de> serde::Deserialize<'de>,
            ($ty2, $ty3): FromJson,
            <($ty2, $ty3) as FromJson>::Error: std::fmt::Debug,
        {
            type Error = String;
            type Future = std::pin::Pin<
                Box<dyn std::future::Future<Output = Result<Self, Self::Error>> + Send>,
            >;
            fn from_json_value(val: serde_json::Value) -> Self::Future {
                Box::pin(async move {
                    let rest = <($ty2, $ty3)>::from_json_value(val.clone()).await;
                    let $name1 = serde_json::from_value::<$ty1>(val).map_err(|e| e.to_string());
                    match $name1 {
                        Err(e) => {
                            println!(concat!(stringify!($ty1), " 反序列化失败, {:?}"), e);
                            return Err(e);
                        }
                        Ok($name1) => match rest {
                            Ok(($name2, $name3)) => Ok(($name1, $name2, $name3)),
                            Err(e) => Err(format!("rest 反序列化失败, {:?}", e)),
                        },
                    }
                })
            }
        }
    };
    // 4 元组
    ($name1:ident : $ty1:ty, $name2:ident : $ty2:ty, $name3:ident : $ty3:ty, $name4:ident : $ty4:ty) => {
        impl<T> FromJson for ($ty1, $ty2, $ty3, $ty4)
        where
            $ty1: for<'de> serde::Deserialize<'de>,
            ($ty2, $ty3, $ty4): FromJson,
            <($ty2, $ty3, $ty4) as FromJson>::Error: std::fmt::Debug,
        {
            type Error = String;
            type Future = std::pin::Pin<
                Box<dyn std::future::Future<Output = Result<Self, Self::Error>> + Send>,
            >;
            fn from_json_value(val: serde_json::Value) -> Self::Future {
                Box::pin(async move {
                    let rest = <($ty2, $ty3, $ty4)>::from_json_value(val.clone()).await;
                    let $name1 = serde_json::from_value::<$ty1>(val).map_err(|e| e.to_string());
                    match $name1 {
                        Err(e) => {
                            println!(concat!(stringify!($ty1), " 反序列化失败, {:?}"), e);
                            return Err(e);
                        }
                        Ok($name1) => match rest {
                            Ok(($name2, $name3, $name4)) => Ok(($name1, $name2, $name3, $name4)),
                            Err(e) => Err(format!("rest 反序列化失败, {:?}", e)),
                        },
                    }
                })
            }
        }
    };
    // 5 元组
    ($name1:ident : $ty1:ty, $name2:ident : $ty2:ty, $name3:ident : $ty3:ty, $name4:ident : $ty4:ty, $name5:ident : $ty5:ty) => {
        impl<T> FromJson for ($ty1, $ty2, $ty3, $ty4, $ty5)
        where
            $ty1: for<'de> serde::Deserialize<'de>,
            ($ty2, $ty3, $ty4, $ty5): FromJson,
            <($ty2, $ty3, $ty4, $ty5) as FromJson>::Error: std::fmt::Debug,
        {
            type Error = String;
            type Future = std::pin::Pin<
                Box<dyn std::future::Future<Output = Result<Self, Self::Error>> + Send>,
            >;
            fn from_json_value(val: serde_json::Value) -> Self::Future {
                Box::pin(async move {
                    let rest = <($ty2, $ty3, $ty4, $ty5)>::from_json_value(val.clone()).await;
                    let $name1 = serde_json::from_value::<$ty1>(val).map_err(|e| e.to_string());
                    match $name1 {
                        Err(e) => {
                            println!(concat!(stringify!($ty1), " 反序列化失败, {:?}"), e);
                            return Err(e);
                        }
                        Ok($name1) => match rest {
                            Ok(($name2, $name3, $name4, $name5)) => {
                                Ok(($name1, $name2, $name3, $name4, $name5))
                            }
                            Err(e) => Err(format!("rest 反序列化失败, {:?}", e)),
                        },
                    }
                })
            }
        }
    };
}

// 用法示例：
impl_from_json_tuple_n!(a: Task, b: Args<T>);
impl_from_json_tuple_n!(a: Args<T>, b: Task);
// 如需支持更多元组，继续扩展宏即可。
