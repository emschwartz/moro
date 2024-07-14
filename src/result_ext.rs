use crate::Scope;

#[async_trait::async_trait(?Send)]
pub trait UnwrapOrCancel: Sized {
    type Ok;
    type Err;

    async fn unwrap_or_cancel<'scope, 'env, T>(
        self,
        scope: &'scope Scope<'scope, 'env, Result<T, Self::Err>>,
    ) -> Self::Ok
    where
        T: Send,
        Self: 'env;
}

#[async_trait::async_trait(?Send)]
impl<O, E> UnwrapOrCancel for Result<O, E> {
    type Ok = O;
    type Err = E;

    async fn unwrap_or_cancel<'scope, 'env, T>(
        self,
        scope: &'scope Scope<'scope, 'env, Result<T, E>>,
    ) -> O
    where
        Self: 'env,
    {
        match self {
            Ok(o) => o,
            Err(e) => scope.terminate(Err(e)).await,
        }
    }
}
