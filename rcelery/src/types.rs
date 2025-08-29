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

impl<T> FromJson for (Args<T>, Task)
where
    Args<T>: FromJson,
    <Args<T> as FromJson>::Error: std::fmt::Debug,
{
    type Error = String;
    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>> + Send>>;

    fn from_json_value(val: serde_json::Value) -> Self::Future {
        Box::pin(async move {
            let arg_res = Args::<T>::from_json_value(val.clone()).await;

            let res = serde_json::from_value::<Task>(val).map_err(|e| e.to_string());
            match res {
                Err(e) => {
                    println!("Task 反序列化失败, {:?}", e);
                    return Err(e);
                }
                Ok(t) => match arg_res {
                    Ok(arg) => return Ok((arg, t)),
                    Err(e) => return Err(format!("Args 反序列化失败, {:?}", e)),
                },
            }
        })
    }
}

impl<T> FromJson for (Task, Args<T>)
where
    Args<T>: FromJson,
    <Args<T> as FromJson>::Error: std::fmt::Debug,
{
    type Error = String;
    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>> + Send>>;

    fn from_json_value(val: serde_json::Value) -> Self::Future {
        Box::pin(async move {
            let arg_res = Args::<T>::from_json_value(val.clone()).await;

            let res = serde_json::from_value::<Task>(val).map_err(|e| e.to_string());
            match res {
                Err(e) => {
                    println!("Task 反序列化失败, {:?}", e);
                    return Err(e);
                }
                Ok(t) => match arg_res {
                    Ok(arg) => return Ok((t, arg)),
                    Err(e) => return Err(format!("Args 反序列化失败, {:?}", e)),
                },
            }
        })
    }
}
