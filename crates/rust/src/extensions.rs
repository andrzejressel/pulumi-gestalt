use serde::Serialize;
use crate::{Context, Output};

pub trait Secretable<T: Serialize> {
    fn as_secret_output(&self, context: &Context) -> Output<T>;
}

impl <T> Secretable<T> for T where T: Serialize {
    fn as_secret_output(&self, context: &Context) -> Output<T> {
        context.new_output(self).secret()
    }
}

impl <T> Secretable<T> for Output<T> where T: Serialize {
    fn as_secret_output(&self, _: &Context) -> Output<T> {
        self.clone().secret()
    }
}


pub trait UnSecretable<T: Serialize> {
    fn as_unsecret_output(&self, context: &Context) -> Output<T>;
}
impl <T> UnSecretable<T> for T where T: Serialize {
    fn as_unsecret_output(&self, context: &Context) -> Output<T> {
        context.new_output(self)
    }
}

impl <T> UnSecretable<T> for Output<T> where T: Serialize {
    fn as_unsecret_output(&self, _: &Context) -> Output<T> {
        self.unsecret()
    }
}

#[cfg(test)]
mod tests {
    use crate::extensions::{Secretable, UnSecretable};

    #[test]
    fn should_compile() {
        
    }
    
    fn mapping(){
        let context = get_invalid_context();
        
        let o = context.new_output(&"test");
        o.as_secret_output(&context);
        o.as_unsecret_output(&context);
        
        "test".as_secret_output(&context);
        "test".as_unsecret_output(&context);
        
    }
    
    fn get_invalid_context() -> crate::Context {
        todo!()
    }
    
}