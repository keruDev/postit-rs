//! Utilities to handle XML files.
//!
//! The `XML` struct implements the [`FilePersister`] trait.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::name::QName;
use quick_xml::{Reader, Writer};

use crate::models::{Priority, Task, Todo};
use crate::traits::FilePersister;

/// Representation of a Xml file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xml {
    /// Location of the Xml file.
    path: PathBuf,
}

impl Xml {
    /// Constructor of the `Xml` struct.
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        Self { path: path.as_ref().to_path_buf() }
    }

    /// Basic structure to initialize a XML file.
    pub fn prolog() -> String {
        String::from(r#"<?xml version="1.0" encoding="UTF-8"?>\n"#)
    }

    /// Document Type Definition of a XML file.
    #[rustfmt::skip]
    pub fn dtd() -> String {
        String::from(
"<!DOCTYPE Tasks [
    <!ELEMENT Tasks (Task+)>
    <!ELEMENT Task (#PCDATA)>
    <!ATTLIST Task 
        id CDATA #REQUIRED
        priority (low | med | high | none) #REQUIRED
        checked (true | false) #REQUIRED
    >
]>\n",
        )
    }

    /// Writes a [Todo] instance into XML writer and returns a buffer with the content.
    ///
    /// # Panics
    /// In case the XML Event can't be written.
    pub fn todo_to_xml(todo: &Todo) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        writer
            .write_event(Event::Start(BytesStart::new("Tasks")))
            .unwrap();

        for task in &todo.tasks {
            Self::task_to_xml(&mut writer, task);
        }

        writer
            .write_event(Event::End(BytesEnd::new("Tasks")))
            .unwrap();

        buffer
    }

    /// Writes a [Task] instance into XML writer.
    ///
    /// # Panics
    /// In case the XML Event can't be written.
    pub fn task_to_xml(writer: &mut Writer<&mut Vec<u8>>, task: &Task) {
        let mut task_bytes = BytesStart::new("Task");
        task_bytes.push_attribute(("id", &*task.id.to_string()));
        task_bytes.push_attribute(("priority", &*task.priority));
        task_bytes.push_attribute(("checked", &*task.checked.to_string()));

        writer.write_event(Event::Start(task_bytes)).unwrap();

        writer
            .write_event(Event::Text(BytesText::new(&task.content)))
            .unwrap();

        writer
            .write_event(Event::End(BytesEnd::new("Task")))
            .unwrap();
    }

    /// Reads the tasks from an XML reader and returns a vector of tasks.
    ///
    /// # Panics
    /// If a value can't be unescaped.
    pub fn xml_to_tasks(mut reader: Reader<&[u8]>) -> Vec<Task> {
        let mut tasks = Vec::<Task>::new();
        let mut task = None::<Task>;

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) if e.name() == QName(b"Task") => {
                    let mut new_task = Task::default();

                    for attr in e.attributes().flatten() {
                        let value = attr.unescape_value().unwrap();
                        match attr.key {
                            QName(b"id") => new_task.id = value.parse().unwrap_or(0),
                            QName(b"priority") => new_task.priority = Priority::from(value),
                            QName(b"checked") => new_task.checked = value == "true",
                            _ => {}
                        }
                    }

                    task = Some(new_task);
                }

                Ok(Event::Text(e)) => {
                    if let Some(ref mut t) = task {
                        t.content = e.unescape().unwrap().into_owned();
                    }
                }

                Ok(Event::End(ref e)) if e.name() == QName(b"Task") => {
                    if let Some(t) = task.take() {
                        tasks.push(t);
                    }
                }

                Ok(Event::Eof) => break,

                Err(e) => {
                    eprintln!("Error reading the XML file: {e:?}");
                    break;
                }

                _ => {}
            }
        }

        tasks
    }
}

impl FilePersister for Xml {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn boxed(self) -> Box<dyn FilePersister> {
        Box::new(self)
    }

    fn exists(&self) -> bool {
        fs::exists(&self.path).expect("The XML file's existence couldn't be checked")
    }

    fn default(&self) -> String {
        Self::prolog()
    }

    fn tasks(&self) -> Vec<Task> {
        let xml = self.lines().join("");
        let reader = Reader::from_str(&xml);

        Self::xml_to_tasks(reader)
    }

    fn open(&self) -> fs::File {
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .expect("Should have been able to create the file")
    }

    fn lines(&self) -> Vec<String> {
        fs::read_to_string(&self.path)
            .expect("Should have been able to read the XML file")
            .lines()
            .map(|line| line.replace('\r', ""))
            .filter(|line| !line.is_empty())
            .collect()
    }

    fn write(&self, todo: &Todo) {
        let buffer = Self::todo_to_xml(todo);
        let xml = String::from_utf8(buffer).unwrap();

        let bytes = [Self::prolog().as_bytes(), Self::dtd().as_bytes(), xml.as_bytes()].concat();

        self.open().write_all(&bytes).unwrap();
    }

    fn clean(&self) {
        fs::write(&self.path, self.default()).expect("Should have been able to clean the CSV file");
    }

    fn remove(&self) {
        fs::remove_file(&self.path).expect("Should have been able to delete the XML file");
    }
}
