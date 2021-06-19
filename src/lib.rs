mod something;

pub mod test {
    pub fn test() {
        crate::something::hello();
    }    
}