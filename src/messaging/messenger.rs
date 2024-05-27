pub trait AmqpMessenger {
    fn send_message(&self, message: &str);
    fn read_message(&self) -> &str;
}



pub struct BasicMessenger {

}
impl AmqpMessenger for BasicMessenger {
    fn send_message(&self, message: &str) {
        todo!()
    }

    fn read_message(&self) -> &str {
        todo!()
    }
}
