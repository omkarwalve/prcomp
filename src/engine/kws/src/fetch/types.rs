use std::{
    fmt::{self,Display,Debug},
    fs::{read_dir,File},
    io::Read,
    path::PathBuf,
    time::Duration,
}; 
use crate::types::Map;

use crate::vars::{
    DEFAULT_TIMEOUT,
    FILE_READ_TO_STRING_FAILURE,
};

//  ┬─┐┌─┐┌─┐ ┬ ┬┌─┐┌─┐┌┬┐
//  ├┬┘├┤ │─┼┐│ │├┤ └─┐ │ 
//  ┴└─└─┘└─┘└└─┘└─┘└─┘ ┴ 
/// ## Request `:: descriptor`
/// Input of `kws::fetch` holding information regarding query.
pub struct Request<T> {
    pub category: T,
    pub schema: Schema,
    pub opt: Option<Vec<(String,String)>>,
    pub query: T,
    pub rank: usize,
    pub wait: Duration,
}

impl<T> Request<T> {
    /// ## `new()`
    /// Create a new `Request` for `fetch()`
    pub fn new( category: T,
            schema: Schema,
            opt: Option<Vec<(String,String)>>,
            query: T,
            rank: Option<usize>,
            wait: Option<u64>,
        ) -> Self {
            Self {
                category,
                schema,
                opt,
                query,
                rank: match rank { Some(rk) => rk, None => 0 },
                wait: match wait { Some(time) => Duration::from_secs(time), None => Duration::from_secs(DEFAULT_TIMEOUT) },
            }
    }
    /// ## `default`
    /// Spins up a `Request` with default setups for `opt`,`rank` & `wait`. 
    /// > **Does require _`category`_,_`dir`_ & _`query`_.**
    pub fn default(category: T, query: T, schema: Schema) -> Self {
        Self {
            category,
            schema,
            opt: None,
            query,
            rank: 0,
            wait: Duration::from_secs(DEFAULT_TIMEOUT)
        }
    }
}

impl<T: Display + Debug> Display for Request<T> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let emptyopt = vec![(String::new(),String::new())];
        write!(f,"Request {{\n\tcategory: {}\n\tdir: {}\n\topt: {:?}\n\tquery: {}\n\trank: {}\n\twait: {}\n}}",
            self.category,
            self.schema,
            match &self.opt {Some(x) => x, None => &emptyopt },
            self.query,
            self.rank,
            self.wait.as_secs()
        )
    }
}

//  ┌─┐┌─┐┬ ┬┌─┐┌┬┐┌─┐
//  └─┐│  ├─┤├┤ │││├─┤
//  └─┘└─┘┴ ┴└─┘┴ ┴┴ ┴
#[derive(Clone)]
pub struct Schema {
    pub root: PathBuf,
    pub dirs: Map<String,PathBuf>,
}
impl Schema {
    pub fn new(root: PathBuf,dirs: Map<String,PathBuf>) -> Self {
        Self { root,dirs }
    }

    /// ### get_dir `:: inexpensive`
    /// Get directory path [`PathBuf`] for a specified `target` if it exists in `Schema`.
    pub fn get_dir(&self,target: String) -> Option<PathBuf> {
        match self.dirs.get(target) {
            Some(path) => Some(self.root.join(path)),
            None => None
        }
    }

    /// ## get_files `:: inexpensive`
    /// Get files(as `File` handles) from a `target` directory residing in the `Schema`.
    pub fn get_files(&self,target: String) -> Option<Vec<Option<File>>> {
        let dir = self.get_dir(target)?;
        match read_dir(dir) {
            Ok(paths) => {
                Some(paths.map(|path| {
                    if let Ok(dirent) = path {
                        if let Ok(file) = File::open(dirent.path()) {
                            Some(file)
                        } else { None }
                    } else { None }
                    }).collect::<Vec<Option<File>>>()
                )
            },
            Err(_) => None
        }
    }

    /// ### get_files_with_contents `:: expensive`
    /// Get files(as `String`'s) from a `target` directory residing in the `Schema`. Reads each file into a `String` and returns a `Vector` of `String`'s.
    pub fn get_files_with_contents(&self,target: String) -> Option<Vec<String>> {
        let dir = self.get_dir(target)?;
        match read_dir(dir) {
            Ok(paths) => {
                Some(paths.map(|path| {
                    match path {
                        Ok(direntry) => {
                            let mut stg = String::new();
                            match File::open(direntry.path()) {
                                Ok(mut file) => { file.read_to_string(&mut stg).expect(FILE_READ_TO_STRING_FAILURE); },
                                Err(_) => { stg = String::new(); }
                            }
                            stg
                        },
                        Err(_) => String::from("")
                    }
                    }).collect::<Vec<String>>()
                )
            },
            Err(_) => None
        }
    }

}
impl From<(PathBuf,Vec<(&str,&str)>)> for Schema {
    fn from(tuple: (PathBuf,Vec<(&str,&str)>)) -> Self {
        Self {
            root: tuple.0,
            dirs: Map::from(tuple.1
                                 .into_iter()
                                 .map(|(t0,t1)| (t0.to_string(),PathBuf::from(t1)))
                                 .collect::<Vec<(String,PathBuf)>>())
        }
    }
}
impl Display for Schema {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Schema {{\n\troot: {}\n\tdirs: {:?}\n}}",
        self.root.display(),
        self.dirs
    )
    }
}