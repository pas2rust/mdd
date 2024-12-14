pub fn middleware(input: ItemStruct) -> TokenStream {
    let struct_name = &input.ident;
    let attrs = input.attrs;
    let custom_call = Helpers::get_attr::<ItemFn>(
        attrs,
        "custom_call",
    )
    .unwrap();
    let middleware_name =
        format!("{}Middleware", struct_name);

    quote! {
        use std::future::{ready, Ready};
        use actix_web::{
            dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
            Error, HttpResponse,
        };
        use futures_util::future::LocalBoxFuture;
        pub struct #struct_name;
        impl<S> Transform<S, ServiceRequest> for #struct_name
        where
            S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
            S::Future: 'static,
        {
            type Response = ServiceResponse;
            type Error = Error;
            type InitError = ();
            type Transform = #middleware_name<S>;
            type Future = Ready<Result<Self::Transform, Self::InitError>>;

            fn new_transform(&self, service: S) -> Self::Future {
                ready(Ok(#middleware_name { service }))
            }
        }
        pub struct #middleware_name<S> {
            service: S,
        }
        impl<S> Service<ServiceRequest> for #middleware_name<S>
        where
            S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
            S::Future: 'static,
        {
            type Response = ServiceResponse;
            type Error = Error;
            type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

            forward_ready!(service);

            fn call(&self, req: ServiceRequest) -> Self::Future {
                #custom_call(&self, req)
                /*let headers = req.headers();transporter
                let key = headers
                    .get("x-key")
                    .and_then(|header| header.to_str().ok())
                    .unwrap_or("");

                if key == env::var("KEY").expect("Env var not found") {
                    let fut = self.service.call(req);
                    return Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    });
                }

                let (req_parts, _body) = req.into_parts();
                let response = HttpResponse::Unauthorized().body("Unauthorized 401");
                let service = ServiceResponse::new(req_parts, response);
                Box::pin(async move { Ok(service) })*/
            }
        }
    }
}