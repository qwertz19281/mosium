use std::sync::Arc;

pub trait RefClonable {
    fn refc(&self) -> Self;
}

impl<T> RefClonable for Arc<T> where T: ?Sized {
    #[inline] fn refc(&self) -> Self {
        Arc::clone(self)
    }
}
/// async parallel mass processing
/// args: input vec, meta arc, #parallel, refname for input, refname for mety, async code block accessing refname'd vars
#[macro_export]
macro_rules! async_par {
    ($inp:ident,$meta:ident,$par:expr,$inp_ref:ident,$meta_ref:ident,$f:block) => {{
        let mut out = Vec::with_capacity($inp.len());

        //so we always spawn up to 64 tasks concurrently and let the stupid executer in parallel
        for c in $inp.into_iter().chunks($par).into_iter() {
            block_on(async {
                let tasks = c
                    .map(|$inp_ref| {
                        let $meta_ref = $meta.refc();
                        spawn(async{ $f })
                    })
                    .collect::<Vec<_>>();

                for t in tasks {
                    if let Ok(r) = t.await {
                        out.push(r);
                    }
                }
            });
        }

        out
    }};
}