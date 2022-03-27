use std::fs::File;
use std::io::Read;
use crypto::digest::Digest;
use crypto::md5::Md5;

pub struct Context {
    pub is_string: bool,
    pub quiet_mode: bool,
}

pub struct Core {
    ctx: Context,
    input: String,
    __output: String,
}

impl Core {
    pub fn new(ctx: Context, input: String) -> Self {
        Self { ctx, input, __output: String::new() }
    }

    pub fn checksum(mut self) -> Self {
        let mut md5 = Md5::new();
        if self.ctx.is_string {
            md5.input_str(&self.input)
        } else {
            let mut file = File::open(&self.input)
                .expect(&format!("md5: {}: No such file", &self.input));
            let buf = &mut String::new();
            file.read_to_string(buf)
                .expect(&format!("md5: {}: Failed to read file", &self.input));
            md5.input(buf.as_bytes());
        }
        self.__output = md5.result_str();
        self
    }

    pub fn format(self) {
        if self.ctx.quiet_mode {
            println!("{}", &self.__output)
        } else {
            println!("md5({}) = {}", &self.input, &self.__output)
        }
    }
}