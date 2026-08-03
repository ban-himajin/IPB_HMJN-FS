pub trait Create<'a>{

    fn create(&self, path: &'a str);

}
pub trait Delete<'a>{

    fn delete(&self, path: &'a str);

}

pub trait Open<'a>{

    fn open(path: &'a str) -> Self;

}
pub trait Close{

    fn close(self);

}
pub trait StreamWrite{

    fn write(&self, buf: &[u8]);

}
pub trait StreamRead{

    fn read(&self, buf: &mut [u8]);

}

pub trait SpecialOperations{

    fn execution(&self);

}