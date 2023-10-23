enum XmlState {
    StartElem,
    EndElem,
    InElem,
    OutElem,
}

trait State {}

pub struct XmlParser<'a> {
    state : Box<dyn State>,
    buffer: Vec<&'a str>
}

impl<'a> XmlParser<'a> {
    fn compute(xmlp : XmlParser) {
        match xmlp {
            StartElem => // clean the buffer
            EndElem => // append buffer to a "storage"
            InElem => // append to a buffer
            OutElem => //whatev ..
        }
    }   
}
